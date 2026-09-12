//! Deterministic scene setup used by the shared verification harness.

use super::{prompts, Game};
use crate::state::{GameSession, GameState};

mod logbook;
mod port;
mod return_travel;
mod scenes;

fn purchase_capture_module(game: &mut Game, module_id: &str) {
    let Some(module) = game.data.modules.get(module_id) else {
        return;
    };
    game.session.economy.credits = game
        .session
        .economy
        .credits
        .max(module.unlock_credits + module.purchase_cost + 100);
    game.session.refresh_module_unlocks(&game.data);
    let _ = game.session.purchase_module(module_id, &game.data);
}

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.session = GameSession::new(&self.data);
        self.travel_elapsed = 0.0;
        self.workspace_elapsed = 0.0;
        self.workspace_camera_shift = 1.0;
        self.workspace_arrival_flash = 0.0;
        self.workspace_log_open = false;
        self.workspace_scan_elapsed = 0.0;
        self.workspace_selected_target = None;
        self.workspace_extraction = None;
        self.workspace_risk = None;
        self.workspace_notice.clear();
        self.workspace_notice_warning = false;
        self.workspace_notice_timer = 0.0;
        self.port_selected_module = Some("engine_core".to_owned());
        self.selected_voyage_plan = crate::engine::VoyagePlan::Standard;
        self.session.briefing_voyage_plan = crate::engine::VoyagePlan::Standard;
        self.return_elapsed = 0.0;
        self.port_hold_expanded = false;
        self.port_service_open = false;
        self.port_loadouts_open = false;
        self.voyage_archive_open = false;
        self.voyage_archive_offset = 0;
        self.voyage_archive_filter = crate::ui::voyage_archive::ArchiveFilter::All;
        self.settings_open = scene == "settings";
        self.exit_requested = false;
        self.state = scenes::prepare(self, scene);
        if self.session.career.is_empty() && !self.session.voyage_log.is_empty() {
            self.session.career =
                crate::state::CareerStats::from_voyage_log(&self.session.voyage_log);
        }
        if scene == "logbook" {
            self.session.career.record_repair(125, 1);
            self.session.career.record_field_power_cell_purchase(80);
            self.session.career.record_field_power_cell_use();
            self.session.career.record_refuel(8, 144);
            self.session.career.record_contract_income(1180);
            self.session.career.record_sale(2300);
            self.session.career.record_module_change(360);
            self.session.career.record_cargo_bay_upgrade();
            self.session.career.record_contract_failure();
        }
        let capture_message = (scene == "port_repaired").then(|| self.message.clone());
        self.resume_state = GameState::Port;
        self.dragged_item = None;
        self.message =
            capture_message.unwrap_or_else(|| prompts::state_prompt(self.state).to_owned());
        self.debug.visible = false;
        self.refresh_save_state();
    }
}
