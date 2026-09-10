//! Top-level game coordinator: input intents become explicit state changes.

use crate::data::GameData;
use crate::engine::{WorkspaceOutcome, WorkspaceRiskReport};
use crate::save;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{CargoStatus, GameSession, GameState, StateTransition};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::debug::DebugOverlay;
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame};
use macroquad_toolkit::settings::GameSettings;

mod capture;
mod prompts;

pub struct Game {
    pub data: GameData,
    pub session: GameSession,
    pub state: GameState,
    resume_state: GameState,
    pub dragged_item: Option<String>,
    pub message: String,
    pub save_exists: bool,
    settings: GameSettings,
    settings_open: bool,
    exit_requested: bool,
    pub travel_elapsed: f32,
    pub workspace_elapsed: f32,
    pub workspace_camera_shift: f32,
    pub workspace_scan_elapsed: f32,
    pub workspace_selected_target: Option<String>,
    pub workspace_extraction: Option<ExtractionRuntime>,
    pub workspace_risk: Option<WorkspaceRiskReport>,
    pub workspace_notice: String,
    pub workspace_notice_warning: bool,
    pub workspace_notice_timer: f32,
    pub port_selected_module: Option<String>,
    pub port_hold_expanded: bool,
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
        let _loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;
        let session = GameSession::new(&data);
        let save_exists = save::has_save(&data);
        let mut settings = GameSettings::load(&data.config.game_name);
        settings.sanitize();
        settings.apply_display();
        Self {
            data,
            session,
            state: GameState::Port,
            resume_state: GameState::Port,
            dragged_item: None,
            message: prompts::state_prompt(GameState::Port).to_owned(),
            save_exists,
            settings,
            settings_open: false,
            exit_requested: false,
            travel_elapsed: 0.0,
            workspace_elapsed: 0.0,
            workspace_camera_shift: 1.0,
            workspace_scan_elapsed: 0.0,
            workspace_selected_target: None,
            workspace_extraction: None,
            workspace_risk: None,
            workspace_notice: String::new(),
            workspace_notice_warning: false,
            workspace_notice_timer: 0.0,
            port_selected_module: Some("engine_core".to_owned()),
            port_hold_expanded: false,
            debug: DebugOverlay::new(),
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub fn update(&mut self, dt: f32) {
        self.debug.record_frame(dt);
        self.update_runtime(dt);
        if is_key_pressed(KeyCode::Escape) {
            if self.settings_open {
                self.apply_action(UiAction::CloseSettings);
            } else if self.state != GameState::MainMenu {
                self.apply_action(UiAction::TogglePause);
            }
        }
        if is_key_pressed(KeyCode::S) && self.state == GameState::Port {
            self.apply_action(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) {
            self.apply_action(UiAction::Load);
        }
        for action in ui::keyboard_actions() {
            self.apply_action(action);
        }
    }

    fn update_runtime(&mut self, dt: f32) {
        match self.state {
            GameState::Travel => {
                self.travel_elapsed =
                    (self.travel_elapsed + dt).min(ui::travel::TRAVEL_DURATION_SECONDS)
            }
            GameState::SalvageWorkspace => {
                self.workspace_elapsed += dt;
                self.workspace_camera_shift = (self.workspace_camera_shift
                    + dt / ui::salvage_scene::SECTION_SHIFT_SECONDS)
                    .min(1.0);
                if self.workspace_scan_elapsed > 0.0 {
                    self.workspace_scan_elapsed += dt;
                    if self.workspace_scan_elapsed >= 0.9 {
                        self.workspace_scan_elapsed = 0.0;
                    }
                }
                self.workspace_notice_timer = (self.workspace_notice_timer - dt).max(0.0);
                let mut completed_target = None;
                let mut clear_extraction = false;
                if let Some(extraction) = self.workspace_extraction.as_mut() {
                    extraction.elapsed += dt;
                    if extraction.elapsed >= extraction.duration && !extraction.resolved {
                        extraction.resolved = true;
                        completed_target = Some(extraction.target_id.clone());
                    }
                    if extraction.resolved && extraction.elapsed > extraction.duration + 0.55 {
                        clear_extraction = true;
                    }
                }
                if let Some(target_id) = completed_target {
                    let resolution = self.workspace_risk.take();
                    self.workspace_notice_warning = resolution.as_ref().is_some_and(|report| {
                        matches!(
                            report.outcome,
                            WorkspaceOutcome::DamagedHull | WorkspaceOutcome::LostTarget
                        )
                    });
                    let resolution_explanation =
                        resolution.as_ref().map(|report| report.explanation.clone());
                    let result = match resolution.as_ref().map(|report| report.outcome.clone()) {
                        Some(WorkspaceOutcome::LostTarget) => {
                            self.session.lose_workspace_target(&target_id, &self.data)
                        }
                        Some(WorkspaceOutcome::DamagedHull) => self
                            .session
                            .recover_workspace_target(&target_id, &self.data)
                            .map(|message| {
                                format!(
                                    "{message}{}",
                                    self.session.apply_workspace_damage(&self.data)
                                )
                            }),
                        _ => self
                            .session
                            .recover_workspace_target(&target_id, &self.data),
                    };
                    match result {
                        Ok(message) => {
                            let display_name = self.data.salvage_objects.get(&target_id).map_or(
                                target_id.as_str(),
                                |target| {
                                    if target.workspace_name.is_empty() {
                                        target.display_name.as_str()
                                    } else {
                                        target.workspace_name.as_str()
                                    }
                                },
                            );
                            let cargo = self.session.expedition.as_ref().map_or(0, |expedition| {
                                expedition
                                    .cargo
                                    .iter()
                                    .filter(|item| item.status == CargoStatus::Pending)
                                    .count()
                            });
                            let used_cells = self.session.ship_layout.occupied_cells();
                            let total_cells =
                                self.session.ship_layout.width * self.session.ship_layout.height;
                            let transfer_label = self.data.salvage_objects.get(&target_id).map_or(
                                "CARGO",
                                |target| match target.transfer_mode.as_str() {
                                    "external_clamp" => "CLAMP",
                                    "tow" => "TOW",
                                    _ => "CARGO",
                                },
                            );
                            let outcome_label = match self.session.target_is_removed(&target_id) {
                                true if message.contains("lost in the wreckage") => "LOST",
                                _ => "RECOVERED",
                            };
                            self.workspace_notice = format!(
                                "{} {}  |  {}  |  Cargo: {}  |  Hold: {}/{} cells  |  ~{} cr",
                                display_name.to_uppercase(),
                                outcome_label,
                                transfer_label,
                                cargo,
                                used_cells,
                                total_cells,
                                self.data
                                    .salvage_objects
                                    .get(&target_id)
                                    .map_or(0, |target| target.sale_value)
                            );
                            self.workspace_notice_timer = 5.0;
                            self.note(match resolution_explanation {
                                Some(explanation) => format!("{message} {explanation}"),
                                None => message,
                            });
                        }
                        Err(error) => self.note(error),
                    }
                }
                if clear_extraction {
                    self.workspace_extraction = None;
                }
            }
            _ => {}
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let active_screen = if self.state == GameState::Pause {
            self.resume_state
        } else {
            self.state
        };
        let (viewport_width, viewport_height) = if active_screen == GameState::Port {
            (screen_width(), screen_height())
        } else {
            (ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT)
        };
        let virtual_ui = begin_virtual_ui_frame(viewport_width, viewport_height);
        let pointer = macroquad_toolkit::ui::Pointer::read(|point| virtual_ui.screen_to_ui(point));
        let context = UiContext {
            data: &self.data,
            session: &self.session,
            state: self.state,
            resume_state: self.resume_state,
            dragged_item: self.dragged_item.as_deref(),
            message: &self.message,
            save_exists: self.save_exists,
            settings_open: self.settings_open,
            fullscreen: self.settings.fullscreen,
            reduced_motion: self.settings.reduced_motion,
            interaction_enabled: true,
            pointer,
            pointer_started: is_mouse_button_pressed(MouseButton::Left)
                || touches()
                    .iter()
                    .any(|touch| touch.phase == TouchPhase::Started),
            ui: &virtual_ui,
            viewport_width,
            viewport_height,
            travel_elapsed: self.travel_elapsed,
            workspace_elapsed: self.workspace_elapsed,
            workspace_camera_shift: self.workspace_camera_shift,
            workspace_scanned: self
                .session
                .expedition
                .as_ref()
                .is_some_and(|expedition| expedition.workspace_scanned),
            workspace_scan_progress: if self.workspace_scan_elapsed > 0.0 {
                (self.workspace_scan_elapsed / 0.9).clamp(0.0, 1.0)
            } else {
                0.0
            },
            workspace_selected_target: self.workspace_selected_target.as_deref(),
            workspace_extraction_target: self
                .workspace_extraction
                .as_ref()
                .map(|extraction| extraction.target_id.as_str()),
            workspace_extraction_progress: self
                .workspace_extraction
                .as_ref()
                .map_or(0.0, ExtractionRuntime::progress),
            workspace_extraction_phase: self
                .workspace_extraction
                .as_ref()
                .map(ExtractionRuntime::phase),
            workspace_risk: self.workspace_risk.as_ref(),
            workspace_notice: &self.workspace_notice,
            workspace_notice_warning: self.workspace_notice_warning,
            workspace_notice_timer: self.workspace_notice_timer,
            port_selected_module: self.port_selected_module.as_deref(),
            port_hold_expanded: self.port_hold_expanded,
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
            UiAction::ContinueGame => {
                if self.resume_state != GameState::MainMenu {
                    self.state = self.resume_state;
                    self.note(prompts::state_prompt(self.state));
                }
            }
            UiAction::NewGame => {
                self.session = GameSession::new(&self.data);
                self.port_selected_module = Some("engine_core".to_owned());
                self.port_hold_expanded = false;
                self.settings_open = false;
                self.transition(StateTransition::ToPort);
                self.note("Fresh ship, fresh debt. Shipyard online.");
            }
            UiAction::BackToMainMenu => {
                self.settings_open = false;
                self.transition(StateTransition::ToMainMenu);
                self.note("Main menu. Tap CONTINUE RUN to return to the current operation.");
            }
            UiAction::ExitGame => {
                self.settings_open = false;
                self.exit_requested = true;
            }
            UiAction::OpenSettings => {
                if self.state == GameState::Pause {
                    self.settings_open = true;
                }
            }
            UiAction::CloseSettings => {
                self.settings_open = false;
            }
            UiAction::ToggleFullscreen => {
                self.settings.fullscreen = !self.settings.fullscreen;
                self.settings.apply_display();
                self.persist_settings("Fullscreen setting saved.");
            }
            UiAction::ToggleReducedMotion => {
                self.settings.reduced_motion = !self.settings.reduced_motion;
                self.settings.apply_effects();
                self.persist_settings("Motion setting saved.");
            }
            UiAction::GoToPort => {
                if matches!(
                    self.state,
                    GameState::Travel | GameState::SalvageWorkspace | GameState::SalvagePacking
                ) || (self.state == GameState::Results && !self.session.returned.is_empty())
                {
                    self.note("Finish the current salvage run before returning to port.");
                } else {
                    self.transition(StateTransition::ToPort);
                    self.note("At the port. Shipyard ready.");
                }
            }
            UiAction::GoToSites => {
                self.transition(StateTransition::ToSiteSelection);
                self.note("Choose a wreck, then tap DEPART FOR WRECK.");
            }
            UiAction::Depart(site_id) => {
                match self.session.begin_expedition(&site_id, &self.data) {
                    Ok(_message) => {
                        self.transition(StateTransition::ToTravel);
                        self.note("Transit underway. Tap ARRIVE to enter the wreck workspace.");
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::ContinueTravel => {
                if self.state == GameState::Travel {
                    self.transition(StateTransition::ToSalvageWorkspace);
                    self.note("Arrival confirmed. Let the scene breathe, then tap SCAN.");
                }
            }
            UiAction::Scan => {
                if self.state != GameState::SalvageWorkspace {
                    return;
                }
                match self.session.scan_workspace(&self.data) {
                    Ok(message) => {
                        self.workspace_scan_elapsed = 0.001;
                        self.workspace_selected_target = None;
                        self.workspace_risk = None;
                        self.note(format!("{message} Tap a bracketed target to inspect it."));
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::SelectSection(section_id) => {
                if self.workspace_extraction.is_some() {
                    self.note("Finish or cancel the active extraction before moving the camera.");
                    return;
                }
                let moving_camera = self
                    .session
                    .expedition
                    .as_ref()
                    .is_some_and(|expedition| expedition.workspace_section != section_id);
                match self
                    .session
                    .switch_workspace_section(&section_id, &self.data)
                {
                    Ok(message) => {
                        if moving_camera {
                            self.workspace_camera_shift = 0.0;
                        }
                        self.workspace_selected_target = None;
                        self.workspace_scan_elapsed = 0.0;
                        self.workspace_risk = None;
                        self.note(format!("{message} Tap SCAN to reveal this section."));
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::SelectTarget(target_id) => {
                if self.session.target_is_revealed(&target_id)
                    && !self.session.target_is_removed(&target_id)
                {
                    self.workspace_selected_target = Some(target_id.clone());
                    self.workspace_risk = self
                        .session
                        .workspace_risk_preview(&target_id, &self.data)
                        .ok();
                    self.note("Target selected. Tap EXTRACT to begin the pull.");
                }
            }
            UiAction::Extract(target_id) => {
                if self.workspace_extraction.is_some() {
                    return;
                }
                match self.session.extraction_block_reason(&target_id, &self.data) {
                    Ok(None) => {
                        let duration = self
                            .session
                            .extraction_duration(&target_id, &self.data)
                            .unwrap_or(4.0);
                        self.workspace_selected_target = Some(target_id.clone());
                        self.workspace_risk = self
                            .session
                            .workspace_risk_preview(&target_id, &self.data)
                            .ok();
                        match self
                            .session
                            .reserve_workspace_energy(&target_id, &self.data)
                        {
                            Ok(power_message) => {
                                self.workspace_extraction =
                                    Some(ExtractionRuntime::new(target_id, duration));
                                self.note(format!(
                                    "Emitter aligned. {power_message} Hold steady while the mount comes free."
                                ));
                            }
                            Err(error) => self.note(error),
                        }
                    }
                    Ok(Some(reason)) => self.note(reason),
                    Err(error) => self.note(error),
                }
            }
            UiAction::AbandonTarget => {
                self.workspace_selected_target = None;
                self.workspace_risk = None;
                self.note("Target abandoned. The wreck remains stable.");
            }
            UiAction::CancelExtraction => {
                if self
                    .workspace_extraction
                    .as_ref()
                    .is_some_and(|extraction| !extraction.resolved)
                {
                    self.workspace_extraction = None;
                    self.workspace_risk = None;
                    self.note(
                        "Extraction cancelled. Tap EXTRACT to try again or RETURN TO PACKING.",
                    );
                }
            }
            UiAction::ReturnFromWorkspace => {
                if self
                    .workspace_extraction
                    .as_ref()
                    .map_or(true, |extraction| extraction.resolved)
                {
                    self.workspace_extraction = None;
                    self.workspace_risk = None;
                    self.transition(StateTransition::ToPacking);
                    self.note("Back aboard. Resolve the cargo footprint before the return burn.");
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
                Ok(()) => self.note("All unplaced salvage left behind. Tap RETURN WITH HAUL."),
                Err(error) => self.note(error),
            },
            UiAction::FinishPacking => match self.session.finish_packing(&self.data) {
                Ok(message) => {
                    self.transition(StateTransition::ToResults);
                    self.note(format!("{message} Choose SELL, INSTALL, or BREAK DOWN."));
                }
                Err(error) => self.note(error),
            },
            UiAction::Disposition(object_id, disposition) => {
                match self.session.dispose(&object_id, disposition, &self.data) {
                    Ok(message) => {
                        self.note(message);
                        if self.session.returned.is_empty() {
                            self.transition(StateTransition::ToPort);
                            self.note("At the port. Shipyard ready.");
                        }
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::SelectPortModule(module_id) => {
                if self.data.modules.contains(&module_id) {
                    self.port_selected_module = Some(module_id.clone());
                    if let Some(module) = self.data.modules.get(&module_id) {
                        let installed = self
                            .session
                            .ship_layout
                            .placements
                            .iter()
                            .any(|item| item.permanent && item.id == module_id);
                        self.note(format!(
                            "{} {}. Read the mount detail in SHIPYARD.",
                            module.display_name,
                            if installed {
                                "selected"
                            } else {
                                "preview selected"
                            }
                        ));
                    }
                }
            }
            UiAction::TogglePortHold => {
                self.port_hold_expanded = !self.port_hold_expanded;
            }
            UiAction::RemoveModule(module_id) => {
                match self.session.remove_module(&module_id, &self.data) {
                    Ok(message) => {
                        if self.port_selected_module.as_deref() == Some(module_id.as_str()) {
                            self.port_selected_module = self
                                .session
                                .ship_layout
                                .placements
                                .iter()
                                .find(|item| item.permanent)
                                .map(|item| item.id.clone());
                        }
                        self.note(message)
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::PurchaseModule(module_id) => {
                match self.session.purchase_module(&module_id, &self.data) {
                    Ok(message) => {
                        self.port_selected_module = Some(module_id.clone());
                        self.note(message)
                    }
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
            UiAction::Save => {
                if self.state != GameState::Port && self.resume_state != GameState::Port {
                    self.note("Save is available at the port checkpoint.");
                    return;
                }
                match save::save_session(&self.session, &self.data) {
                    Ok(()) => {
                        self.refresh_save_state();
                        self.note("Safe checkpoint saved.");
                    }
                    Err(error) => self.note(format!("Save failed: {error}")),
                }
            }
            UiAction::Load => match save::load_session(&self.data) {
                Ok(session) => {
                    self.session = session;
                    let restored_state = if let Some(expedition) = &self.session.expedition {
                        if expedition.workspace_scanned {
                            GameState::SalvageWorkspace
                        } else {
                            GameState::Travel
                        }
                    } else if !self.session.returned.is_empty() {
                        GameState::Results
                    } else {
                        GameState::Port
                    };
                    self.state = restored_state;
                    self.resume_state = restored_state;
                    self.settings_open = false;
                    self.dragged_item = None;
                    self.travel_elapsed = 0.0;
                    self.workspace_elapsed = 0.0;
                    self.workspace_camera_shift = 1.0;
                    self.workspace_scan_elapsed = 0.0;
                    self.workspace_selected_target = None;
                    self.workspace_extraction = None;
                    self.workspace_risk = None;
                    self.workspace_notice.clear();
                    self.workspace_notice_warning = false;
                    self.workspace_notice_timer = 0.0;
                    self.port_selected_module = self
                        .session
                        .ship_layout
                        .placements
                        .iter()
                        .find(|item| item.permanent)
                        .map(|item| item.id.clone());
                    self.port_hold_expanded = false;
                    self.refresh_save_state();
                    self.note(format!(
                        "Safe checkpoint loaded. {}",
                        prompts::state_prompt(restored_state)
                    ));
                }
                Err(error) => self.note(format!("Load failed: {error}")),
            },
            UiAction::TogglePause => {
                if self.state == GameState::Pause {
                    self.settings_open = false;
                    self.state = self.resume_state;
                    self.note(prompts::state_prompt(self.state));
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
            StateTransition::ToMainMenu => GameState::MainMenu,
            StateTransition::ToPort => GameState::Port,
            StateTransition::ToSiteSelection => GameState::SiteSelection,
            StateTransition::ToTravel => GameState::Travel,
            StateTransition::ToSalvageWorkspace => GameState::SalvageWorkspace,
            StateTransition::ToPacking => GameState::SalvagePacking,
            StateTransition::ToResults => GameState::Results,
            StateTransition::ToPause => GameState::Pause,
        };
        self.dragged_item = None;
        match self.state {
            GameState::Travel => {
                self.travel_elapsed = 0.0;
                self.workspace_extraction = None;
                self.workspace_risk = None;
                self.workspace_selected_target = None;
                self.workspace_notice_warning = false;
            }
            GameState::SalvageWorkspace => {
                self.workspace_elapsed = 0.0;
                self.workspace_camera_shift = 1.0;
                self.workspace_scan_elapsed = 0.0;
                self.workspace_extraction = None;
                self.workspace_risk = None;
                self.workspace_selected_target = None;
                self.workspace_notice.clear();
                self.workspace_notice_warning = false;
                self.workspace_notice_timer = 0.0;
            }
            GameState::Port
            | GameState::MainMenu
            | GameState::SiteSelection
            | GameState::SalvagePacking
            | GameState::Results
            | GameState::Pause => {}
        }
    }

    fn note(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    fn refresh_save_state(&mut self) {
        self.save_exists = save::has_save(&self.data);
    }

    fn persist_settings(&mut self, success_message: &str) {
        self.settings.sanitize();
        match self.settings.save(&self.data.config.game_name) {
            Ok(()) => self.note(success_message),
            Err(error) => self.note(format!("Settings save failed: {error}")),
        }
    }
}
