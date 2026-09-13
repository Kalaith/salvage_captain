//! Top-level game coordinator: input intents become explicit state changes.

use crate::data::GameData;
use crate::engine::{VoyagePlan, WorkspaceRiskReport};
use crate::save;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{GameSession, GameState};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::debug::DebugOverlay;
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, end_virtual_ui_frame};
use macroquad_toolkit::settings::GameSettings;

mod actions;
mod briefing;
mod capture;
mod port_actions;
pub mod prompts;
mod runtime;
mod settings;
mod workspace_actions;
mod workspace_placement;

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
    pub return_elapsed: f32,
    pub workspace_elapsed: f32,
    pub workspace_camera_shift: f32,
    pub workspace_arrival_flash: f32,
    pub workspace_log_open: bool,
    pub target_details_open: bool,
    pub transit_details_open: bool,
    pub workspace_scan_elapsed: f32,
    pub workspace_selected_target: Option<String>,
    pub workspace_placement_rotation: Option<u8>,
    pub workspace_extraction: Option<ExtractionRuntime>,
    pub workspace_risk: Option<WorkspaceRiskReport>,
    pub workspace_notice: String,
    pub workspace_notice_warning: bool,
    pub workspace_notice_timer: f32,
    pub port_selected_module: Option<String>,
    pub selected_voyage_plan: VoyagePlan,
    pub wreck_selection: ui::site_cards::WreckSelection,
    pub port_hold_expanded: bool,
    pub port_tab: ui::port_panel::PortTab,
    pub port_stock_page: usize,
    pub port_loadouts_open: bool,
    pub voyage_archive_open: bool,
    pub voyage_archive: crate::ui::voyage_archive::ArchiveState,
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
            return_elapsed: 0.0,
            workspace_elapsed: 0.0,
            workspace_camera_shift: 1.0,
            workspace_arrival_flash: 0.0,
            workspace_log_open: false,
            target_details_open: false,
            transit_details_open: false,
            workspace_scan_elapsed: 0.0,
            workspace_selected_target: None,
            workspace_placement_rotation: None,
            workspace_extraction: None,
            workspace_risk: None,
            workspace_notice: String::new(),
            workspace_notice_warning: false,
            workspace_notice_timer: 0.0,
            port_selected_module: Some("engine_core".to_owned()),
            selected_voyage_plan: VoyagePlan::Standard,
            wreck_selection: ui::site_cards::WreckSelection::default(),
            port_hold_expanded: false,
            port_tab: ui::port_panel::PortTab::default(),
            port_stock_page: 0,
            port_loadouts_open: false,
            voyage_archive_open: false,
            voyage_archive: ui::voyage_archive::ArchiveState::default(),
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

    pub fn draw(&mut self) {
        clear_background(ui::visual_theme::space());
        let (viewport_width, viewport_height) = (ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let virtual_ui = begin_virtual_ui_frame(viewport_width, viewport_height);
        let pointer = macroquad_toolkit::ui::Pointer::read(|point| virtual_ui.screen_to_ui(point));
        let context = UiContext {
            data: &self.data,
            session: &self.session,
            state: self.state,
            resume_state: self.resume_state,
            dragged_item: self.dragged_item.as_deref(),
            workspace_placement_rotation: self.workspace_placement_rotation,
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
            return_elapsed: self.return_elapsed,
            workspace_elapsed: self.workspace_elapsed,
            workspace_camera_shift: self.workspace_camera_shift,
            workspace_arrival_flash: self.workspace_arrival_flash,
            workspace_log_open: self.workspace_log_open,
            target_details_open: self.target_details_open,
            transit_details_open: self.transit_details_open,
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
            voyage_plan: self.selected_voyage_plan,
            wreck_selection: &self.wreck_selection,
            port_hold_expanded: self.port_hold_expanded,
            port_tab: self.port_tab,
            port_stock_page: self.port_stock_page,
            port_loadouts_open: self.port_loadouts_open,
            voyage_archive_open: self.voyage_archive_open,
            voyage_archive: self.voyage_archive,
        };
        let actions = ui::draw_game_ui(context);
        end_virtual_ui_frame();
        for action in actions {
            self.apply_action(action);
        }
        self.debug.draw(&[]);
    }

    fn apply_action(&mut self, action: UiAction) {
        if self.apply_workspace_placement_action(&action) {
            return;
        }
        if self.apply_port_action(&action)
            || self.apply_navigation_action(&action)
            || self.apply_briefing_action(&action)
        {
            return;
        }
        if matches!(
            &action,
            UiAction::Scan
                | UiAction::PowerCycle
                | UiAction::CycleDroneDirective
                | UiAction::SelectSection(_)
                | UiAction::SelectTarget(_)
                | UiAction::Stabilize(_)
                | UiAction::Extract(_)
                | UiAction::AbandonTarget
                | UiAction::CancelExtraction
                | UiAction::ReturnFromWorkspace
        ) {
            self.apply_workspace_action(action);
            return;
        }
        if let UiAction::ToggleTransitDetails = action {
            if self.state == GameState::Travel {
                self.transit_details_open = !self.transit_details_open;
            }
            return;
        }
        if let UiAction::ToggleTargetDetails = action {
            if self.state == GameState::SalvageWorkspace && self.workspace_selected_target.is_some()
            {
                self.target_details_open = !self.target_details_open;
            }
            return;
        }
        if let UiAction::ToggleWorkspaceLog = action {
            if self.state == GameState::SalvageWorkspace {
                self.workspace_log_open = !self.workspace_log_open;
            }
            return;
        }
        if self.apply_packing_action(&action)
            || self.apply_disposition_action(&action)
            || self.apply_shipyard_selection_action(&action)
            || self.apply_shipyard_service_action(&action)
            || self.apply_save_load_action(&action)
            || self.apply_pause_action(&action)
        {
            return;
        }
        unreachable!("unhandled action");
    }
}
