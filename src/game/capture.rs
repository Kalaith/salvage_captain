//! Deterministic scene setup used by the shared verification harness.

use super::{prompts, Game};
use crate::state::{GameSession, GameState};

mod debrief;
mod logbook;
mod port;
mod return_travel;
mod scenes;
mod selection;
mod transit;

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
        self.target_details_open = false;
        self.transit_details_open = false;
        self.manifest_page = crate::ui::decision_panel::navigation::ManifestPage::default();
        self.workspace_scan_elapsed = 0.0;
        self.workspace_selected_target = None;
        self.workspace_placement_rotation = None;
        self.workspace_extraction = None;
        self.workspace_risk = None;
        self.workspace_notice.clear();
        self.workspace_notice_warning = false;
        self.workspace_notice_timer = 0.0;
        self.port_selected_module = Some("engine_core".to_owned());
        self.wreck_selection = crate::ui::site_cards::WreckSelection::default();
        self.selected_voyage_plan = crate::engine::VoyagePlan::Standard;
        self.session.briefing_voyage_plan = crate::engine::VoyagePlan::Standard;
        self.return_elapsed = 0.0;
        self.port_hold_expanded = false;
        self.port_tab = crate::ui::port_panel::PortTab::default();
        self.port_stock_page = 0;
        self.port_loadouts_open = false;
        self.voyage_archive_open = false;
        self.voyage_archive = crate::ui::voyage_archive::ArchiveState::default();
        self.settings_open = scene == "settings";
        self.settings.reduced_motion = scene.ends_with("_reduced_motion");
        self.exit_requested = false;
        self.state = scenes::prepare(self, scene);
        selection::prepare(self, scene);
        if self.session.career.is_empty() && !self.session.voyage_log.is_empty() {
            self.session.career =
                crate::state::CareerStats::from_voyage_log(&self.session.voyage_log);
        }
        if scene.starts_with("logbook") {
            self.session.career.record_repair(125, 1);
            self.session.career.record_field_power_cell_purchase(80);
            self.session.career.record_field_power_cell_use();
            self.session.career.record_refuel(8, 144);
            self.session.career.record_contract_income(1180);
            self.session.career.record_sale(2300);
            self.session.career.record_module_change(360);
            self.session.career.record_cargo_bay_upgrade();
            self.session.career.record_contract_failure();
            logbook::configure(self, scene);
        }
        let capture_message = (scene == "port_repaired" || scene.starts_with("salvage_placement"))
            .then(|| self.message.clone());
        self.resume_state = if scene == "travel_paused" {
            GameState::Travel
        } else {
            GameState::Port
        };
        self.dragged_item = None;
        self.message =
            capture_message.unwrap_or_else(|| prompts::state_prompt(self.state).to_owned());
        self.debug.visible = false;
        self.refresh_save_state();
    }
}

fn begin_capture_transfer(game: &mut Game, target_id: &str) {
    let target = game
        .data
        .salvage_objects
        .get(target_id)
        .expect("capture target exists");
    if let Some((position, rotation)) = game.session.ship_layout.first_fit(
        &format!("cargo:{target_id}"),
        target.footprint,
        target.rotatable,
    ) {
        let _ = game
            .session
            .begin_workspace_transfer(target_id, position, rotation, &game.data);
    }
}

fn recover_capture_cargo(game: &mut Game) {
    let _ = game.session.scan_workspace(&game.data);
    for target_id in ["industrial_battery", "navigation_computer"] {
        begin_capture_transfer(game, target_id);
        let _ = game.session.recover_workspace_target(target_id, &game.data);
    }
}
