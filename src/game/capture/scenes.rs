//! Named verification scene builders for the capture harness.

use super::{logbook, purchase_capture_module};
use crate::engine::RiskOutcome;
use crate::game::Game;
use crate::state::{
    GameState, VoyageRecord, WorkspaceLogEntry, WorkspaceLogEvent, WorkspaceScanProfile,
};
mod salvage;

pub(super) fn prepare(game: &mut Game, scene: &str) -> GameState {
    if scene.starts_with("results") {
        return super::debrief::prepare(game, scene);
    }
    if let Some(state) = super::transit::prepare(game, scene) {
        return state;
    }
    match scene {
        "main_menu" => GameState::MainMenu,
        "settings" => GameState::Pause,
        "gameplay" | "port" => GameState::Port,
        "port_upgraded" | "port_upgraded_damage" => game.capture_upgraded_ship(scene),
        "port_crew_tired" => {
            game.port_tab = crate::ui::port_panel::PortTab::Crew;
            game.session.crew_fatigue = 21;
            GameState::Port
        }
        "port_crew_training" => {
            game.port_tab = crate::ui::port_panel::PortTab::Crew;
            game.session.career.crew_experience[crate::state::CrewRole::Deckhand.index()] = 2;
            GameState::Port
        }
        "port_worn" => {
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            game.session.ship_wear = 42;
            GameState::Port
        }
        "port_cargo_bay" => {
            game.port_hold_expanded = true;
            game.session.cargo_bay_level = 1;
            game.session.economy.credits = 1_000;
            GameState::Port
        }
        "port_cargo_bay_low_funds" => {
            game.port_hold_expanded = true;
            game.session.economy.credits = 100;
            GameState::Port
        }
        "port_services" => {
            let _ = game.capture_port_scene("port_damage");
            game.session.ship_wear = 42;
            game.session.economy.alloy = 1;
            game.session.economy.electronics = 1;
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            GameState::Port
        }
        "port_services_low_funds" => {
            let _ = game.capture_port_scene("port_damage");
            game.session.economy.credits = 50;
            game.session.ship_wear = 42;
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            GameState::Port
        }
        "port_services_no_materials" => {
            let _ = game.capture_port_scene("port_damage");
            game.session.ship_wear = 42;
            game.session.economy.alloy = 0;
            game.session.economy.electronics = 0;
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            GameState::Port
        }
        "port_services_full_cells" => {
            let _ = game.capture_port_scene("port_damage");
            game.session.ship_wear = 42;
            game.session.field_power_cells = crate::state::workspace_energy::MAX_FIELD_POWER_CELLS;
            game.session.economy.alloy = 1;
            game.session.economy.electronics = 1;
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            GameState::Port
        }
        "port_loadouts" => {
            game.port_tab = crate::ui::port_panel::PortTab::Equipment;
            let _ = game.session.store_loadout(0);
            game.port_loadouts_open = true;
            GameState::Port
        }
        "port_damage" | "port_repair_low_funds" | "port_repaired" => game.capture_port_scene(scene),
        scene if scene.starts_with("logbook") => {
            logbook::prepare(game);
            GameState::Port
        }
        "port_preview" => {
            game.port_tab = crate::ui::port_panel::PortTab::Equipment;
            game.port_selected_module = Some("scanner_module".to_owned());
            GameState::Port
        }
        "port_heavy_tractor" => {
            game.port_tab = crate::ui::port_panel::PortTab::Equipment;
            game.port_stock_page = 1;
            game.port_selected_module = Some("reactor_module".to_owned());
            GameState::Port
        }
        "port_equipment" | "port_equipment_last" => {
            game.port_tab = crate::ui::port_panel::PortTab::Equipment;
            game.port_stock_page = if scene.ends_with("last") { 2 } else { 0 };
            GameState::Port
        }
        "port_grid" => {
            game.port_hold_expanded = true;
            GameState::Port
        }
        "port_crew_low_funds" | "port_crew_veteran" => {
            game.port_tab = crate::ui::port_panel::PortTab::Crew;
            if scene.ends_with("veteran") {
                game.session.career.crew_experience[0] = crate::state::crew::MAX_CREW_EXPERIENCE;
            } else {
                game.session.economy.credits = 0;
            }
            GameState::Port
        }
        "port_crew" => {
            game.port_tab = crate::ui::port_panel::PortTab::Crew;
            GameState::Port
        }
        "port_refinery" => {
            game.port_tab = crate::ui::port_panel::PortTab::Service;
            game.session.economy.alloy = 8;
            game.session.economy.electronics = 5;
            GameState::Port
        }
        "sites_military" | "sites_research" | "sites_insured" | "sites_low_fuel"
        | "sites_low_credits" | "sites_details" | "sites_tired" | "sites_completed"
        | "sites_failed" => GameState::SiteSelection,
        "sites" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            GameState::SiteSelection
        }
        "sites_route_familiarity" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            for (site_id, visits) in [
                ("merchant_wreck", 1),
                ("military_wreck", 2),
                ("research_vessel", 3),
            ] {
                if let Some(progress) = game.session.site_progress.get_mut(site_id) {
                    progress.visits = visits;
                }
            }
            GameState::SiteSelection
        }
        "sites_contract_streak" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            game.session.career.contract_streak = 2;
            game.session.career.best_contract_streak = 3;
            GameState::SiteSelection
        }
        "sites_crew_progress" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            game.session.crew_role = crate::state::CrewRole::Navigator;
            game.session.career.crew_experience = [0, 4, 0, 0, 0];
            GameState::SiteSelection
        }
        "sites_crew_veteran" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            game.session.crew_role = crate::state::CrewRole::Broker;
            game.session.career.crew_experience = [0, 0, 0, 0, 6];
            GameState::SiteSelection
        }
        "sites_progress" => {
            if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
                progress.condition = 64;
                progress.visits = 1;
                progress.discovered_sections = vec!["cargo_bay".to_owned()];
                progress.removed_targets = vec![
                    "industrial_battery".to_owned(),
                    "navigation_computer".to_owned(),
                    "engine_assembly".to_owned(),
                ];
                progress.cleared_sections = vec!["cargo_bay".to_owned()];
                progress.reconnaissance_level = 1;
                progress.operation_log = vec![
                    WorkspaceLogEntry::new(1, WorkspaceLogEvent::Departed, Some("cargo_bay"), None),
                    WorkspaceLogEntry::new(
                        2,
                        WorkspaceLogEvent::SectionScanned,
                        Some("cargo_bay"),
                        None,
                    ),
                    WorkspaceLogEntry::new(
                        3,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("industrial_battery"),
                    ),
                    WorkspaceLogEntry::new(
                        4,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("navigation_computer"),
                    ),
                    WorkspaceLogEntry::new(
                        5,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("engine_assembly"),
                    ),
                ];
            }
            game.session.voyage_log = vec![VoyageRecord {
                site_id: "merchant_wreck".to_owned(),
                recovered_count: 2,
                recovered_value: 250,
                recovered_alloy: 5,
                recovered_electronics: 2,
                external_load: 0,
                risk_outcome: RiskOutcome::OrdinaryReturn,
                danger_score: 15,
                reconnaissance_level: 1,
                voyage_plan: crate::engine::VoyagePlan::Cautious,
                return_policy: crate::state::ReturnPolicy::default(),
                contract_completed: true,
                contract_failed: false,
                contract_accepted: true,
                scan_profile: WorkspaceScanProfile::Array,
                drone_directive: crate::state::DroneDirective::PullSupport,
                condition_after: 64,
                cleared_sections: vec!["cargo_bay".to_owned()],
                clearance_payout: 140,
                return_fuel: 2,
                market_cycle: 0,
                insured: false,
                insurance_premium: 0,
                insurance_payout: 0,
            }];
            GameState::SiteSelection
        }
        "sites_private_haul" => {
            game.session.career.contract_streak = 2;
            game.session.career.best_contract_streak = 3;
            game.session.reputation = 3;
            game.session.voyage_log = vec![logbook::private_record()];
            GameState::SiteSelection
        }
        "travel" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            game.travel_elapsed = 2.0;
            GameState::Travel
        }
        "travel_cargo_bay" => {
            game.session.cargo_bay_level = 1;
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            game.travel_elapsed = 2.0;
            GameState::Travel
        }
        "travel_familiarity" => {
            game.session
                .site_progress
                .get_mut("merchant_wreck")
                .expect("capture site exists")
                .visits = 3;
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            game.travel_elapsed = 2.0;
            GameState::Travel
        }
        "travel_cruise" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Expedited;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            let _ = game
                .session
                .buy_reconnaissance("merchant_wreck", &game.data);
            let _ = game.session.begin_expedition_with_plan(
                "merchant_wreck",
                &game.data,
                false,
                game.selected_voyage_plan,
            );
            game.travel_elapsed = 1.6;
            GameState::Travel
        }
        "travel_scanner" => {
            purchase_capture_module(game, "scanner_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            game.travel_elapsed = 2.0;
            GameState::Travel
        }
        _ => salvage::prepare(game, scene),
    }
}
