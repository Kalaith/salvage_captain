//! Top-level game coordinator: input intents become explicit state changes.

use crate::data::{GameData, GridPosition};
use crate::engine::RiskOutcome;
use crate::save;
use crate::state::{CargoStatus, GameSession, GameState, StateTransition};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::debug::DebugOverlay;
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame};

pub struct Game {
    pub data: GameData,
    pub session: GameSession,
    pub state: GameState,
    resume_state: GameState,
    pub dragged_item: Option<String>,
    pub message: String,
    pub loaded_assets: usize,
    pub save_exists: bool,
    pub save_slots: Vec<String>,
    debug: DebugOverlay,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|error| {
            panic!("Salvage Captain data failed validation: {error}");
        });
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.13, 0.4, 0.53, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        let loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;
        let session = GameSession::new(&data);
        let save_exists = save::has_save(&data);
        let save_slots = save::slots(&data);
        Self {
            data,
            session,
            state: GameState::Port,
            resume_state: GameState::Port,
            dragged_item: None,
            message: "Ship systems online. Pick a wreck when you are ready.".to_owned(),
            loaded_assets,
            save_exists,
            save_slots,
            debug: DebugOverlay::new(),
        }
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.session = GameSession::new(&self.data);
        self.state = match scene {
            "gameplay" => GameState::Port,
            "sites" => GameState::SiteSelection,
            "packing" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                GameState::SalvagePacking
            }
            "results" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                if let Some(expedition) = self.session.expedition.as_mut() {
                    for item in &mut expedition.cargo {
                        if item.status == CargoStatus::Pending {
                            item.status = CargoStatus::Packed;
                            item.position = Some(GridPosition::new(2, 2));
                        }
                    }
                }
                let _ = self.session.finish_packing(&self.data);
                GameState::Results
            }
            "paused" => GameState::Pause,
            _ => panic!("Unknown Salvage Captain capture scene: {scene}"),
        };
        self.resume_state = GameState::Port;
        self.dragged_item = None;
        self.message = format!("Capture scene: {scene}");
        self.debug.visible = false;
        self.refresh_save_state();
    }

    pub fn update(&mut self, dt: f32) {
        self.debug.record_frame(dt);
        if is_key_pressed(KeyCode::Escape) {
            self.apply_action(UiAction::TogglePause);
        }
        if is_key_pressed(KeyCode::S) && self.state != GameState::SalvagePacking {
            self.apply_action(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) {
            self.apply_action(UiAction::Load);
        }
        for action in ui::keyboard_actions() {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let pointer = macroquad_toolkit::ui::Pointer::read(|point| virtual_ui.screen_to_ui(point));
        let context = UiContext {
            data: &self.data,
            session: &self.session,
            state: self.state,
            resume_state: self.resume_state,
            dragged_item: self.dragged_item.as_deref(),
            message: &self.message,
            save_exists: self.save_exists,
            save_slots: &self.save_slots,
            loaded_assets: self.loaded_assets,
            pointer,
            pointer_started: is_mouse_button_pressed(MouseButton::Left)
                || touches()
                    .iter()
                    .any(|touch| touch.phase == TouchPhase::Started),
            ui: &virtual_ui,
        };
        let actions = ui::draw_game_ui(context);
        end_virtual_ui_frame();
        for action in actions {
            self.apply_action(action);
        }
        self.debug.draw(&[]);
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::NewGame => {
                self.session = GameSession::new(&self.data);
                self.transition(StateTransition::ToPort);
                self.note("Fresh ship, fresh debt. The port is yours.");
            }
            UiAction::GoToPort => {
                if self.state == GameState::SalvagePacking
                    || (self.state == GameState::Results && !self.session.returned.is_empty())
                {
                    self.note("Resolve the current expedition before returning to port.");
                } else {
                    self.transition(StateTransition::ToPort);
                }
            }
            UiAction::GoToSites => self.transition(StateTransition::ToSiteSelection),
            UiAction::Depart(site_id) => {
                match self.session.begin_expedition(&site_id, &self.data) {
                    Ok(message) => {
                        self.note(message);
                        self.transition(StateTransition::ToPacking);
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::AutoPlace(object_id) => match self.session.auto_place(&object_id, &self.data)
            {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::BeginDrag(object_id) => {
                self.dragged_item = Some(object_id);
                self.note("Drag the ghost footprint onto an open grid cell.");
            }
            UiAction::DropDragged(position, rotation) => {
                if let Some(object_id) = self.dragged_item.take() {
                    match self
                        .session
                        .place_cargo(&object_id, position, rotation, &self.data)
                    {
                        Ok(message) => self.note(message),
                        Err(error) => self.note(error),
                    }
                }
            }
            UiAction::CancelDrag => {
                self.dragged_item = None;
                self.note("Salvage drag cancelled.");
            }
            UiAction::Rotate(object_id) => {
                match self.session.rotate_cargo(&object_id, &self.data) {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
            }
            UiAction::Leave(object_id) => {
                match self
                    .session
                    .set_cargo_status(&object_id, CargoStatus::LeftBehind)
                {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
            }
            UiAction::Discard(object_id) => {
                match self
                    .session
                    .set_cargo_status(&object_id, CargoStatus::Discarded)
                {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
            }
            UiAction::LeaveAll => match self.session.leave_all_pending() {
                Ok(()) => self.note("All unplaced salvage left behind."),
                Err(error) => self.note(error),
            },
            UiAction::FinishPacking => match self.session.finish_packing(&self.data) {
                Ok(message) => {
                    self.note(message);
                    self.transition(StateTransition::ToResults);
                }
                Err(error) => self.note(error),
            },
            UiAction::Disposition(object_id, disposition) => {
                match self.session.dispose(&object_id, disposition, &self.data) {
                    Ok(message) => {
                        self.note(message);
                        if self.session.returned.is_empty() {
                            self.transition(StateTransition::ToPort);
                        }
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::RemoveModule(module_id) => {
                match self.session.remove_module(&module_id, &self.data) {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
            }
            UiAction::Refuel => match self.session.refuel(&self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::Repair => match self.session.repair(&self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::Save => match save::save_session(&self.session, &self.data) {
                Ok(()) => {
                    self.refresh_save_state();
                    self.note("Safe checkpoint saved.");
                }
                Err(error) => self.note(format!("Save failed: {error}")),
            },
            UiAction::Load => match save::load_session(&self.data) {
                Ok(session) => {
                    self.session = session;
                    let restored_state = if self.session.expedition.is_some() {
                        GameState::SalvagePacking
                    } else if !self.session.returned.is_empty() {
                        GameState::Results
                    } else {
                        GameState::Port
                    };
                    self.state = restored_state;
                    self.resume_state = restored_state;
                    self.dragged_item = None;
                    self.refresh_save_state();
                    self.note("Safe checkpoint loaded.");
                }
                Err(error) => self.note(format!("Load failed: {error}")),
            },
            UiAction::TogglePause => {
                if self.state == GameState::Pause {
                    self.state = self.resume_state;
                } else {
                    self.resume_state = self.state;
                    self.transition(StateTransition::ToPause);
                }
            }
            UiAction::ToggleStats => self.debug.toggle(),
        }
    }

    fn transition(&mut self, transition: StateTransition) {
        if transition == StateTransition::ToPause {
            if self.state != GameState::Pause {
                self.resume_state = self.state;
            }
            self.state = GameState::Pause;
            return;
        }
        self.state = match transition {
            StateTransition::ToPort => GameState::Port,
            StateTransition::ToSiteSelection => GameState::SiteSelection,
            StateTransition::ToPacking => GameState::SalvagePacking,
            StateTransition::ToResults => GameState::Results,
            StateTransition::ToPause => GameState::Pause,
        };
        self.dragged_item = None;
    }

    fn note(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    fn refresh_save_state(&mut self) {
        self.save_exists = save::has_save(&self.data);
        self.save_slots = save::slots(&self.data);
    }
}

#[allow(dead_code)]
fn risk_label(outcome: RiskOutcome) -> &'static str {
    match outcome {
        RiskOutcome::OrdinaryReturn => "ORDINARY RETURN",
        RiskOutcome::DamagedModule => "DAMAGED MODULE",
        RiskOutcome::LostSalvage => "LOST SALVAGE",
        RiskOutcome::EmergencyRepair => "EMERGENCY REPAIR",
        RiskOutcome::ForcedAbandon => "FORCED ABANDON",
    }
}
