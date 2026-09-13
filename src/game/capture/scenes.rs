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
    if let Some(state) = prepare_port(game, scene) {
        return state;
    }
    if let Some(state) = prepare_sites(game, scene) {
        return state;
    }
    if let Some(state) = prepare_travel(game, scene) {
        return state;
    }
    match scene {
        "main_menu" => GameState::MainMenu,
        "settings" => GameState::Pause,
        _ => salvage::prepare(game, scene),
    }
}

fn prepare_port(game: &mut Game, scene: &str) -> Option<GameState> {
    match scene {
        "gameplay" | "port" => Some(GameState::Port),
        "port_upgraded" | "port_upgraded_damage" => Some(game.capture_upgraded_ship(scene)),
        "port_crew_tired" => Some(prepare_tired_crew(game)),
        "port_crew_training" => Some(prepare_crew_training(game)),
        "port_worn" => Some(prepare_worn_port(game)),
        "port_cargo_bay" => Some(prepare_cargo_bay(game, true)),
        "port_cargo_bay_low_funds" => Some(prepare_cargo_bay(game, false)),
        "port_services" => Some(prepare_services(game, None, 1, 1)),
        "port_services_low_funds" => Some(prepare_services(game, Some(50), 0, 0)),
        "port_services_no_materials" => Some(prepare_services(game, None, 0, 0)),
        "port_services_full_cells" => Some(prepare_full_power_cells(game)),
        "port_loadouts" => Some(prepare_loadouts(game)),
        "port_damage" | "port_repair_low_funds" | "port_repaired" => {
            Some(game.capture_port_scene(scene))
        }
        scene if scene.starts_with("logbook") => {
            logbook::prepare(game);
            Some(GameState::Port)
        }
        "port_preview" => Some(prepare_equipment_preview(game)),
        "port_heavy_tractor" => Some(prepare_heavy_equipment(game)),
        "port_equipment" | "port_equipment_last" => Some(prepare_equipment_page(game, scene)),
        "port_grid" => {
            game.port_hold_expanded = true;
            Some(GameState::Port)
        }
        "port_crew_low_funds" | "port_crew_veteran" => Some(prepare_crew_edge(game, scene)),
        "port_crew" => {
            game.port_tab = crate::ui::port_panel::PortTab::Crew;
            Some(GameState::Port)
        }
        "port_refinery" => Some(prepare_refinery(game)),
        _ => None,
    }
}

fn prepare_tired_crew(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Crew;
    game.session.crew_fatigue = 21;
    GameState::Port
}

fn prepare_crew_training(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Crew;
    game.session.career.crew_experience[crate::state::CrewRole::Deckhand.index()] = 2;
    GameState::Port
}

fn prepare_worn_port(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Service;
    game.session.ship_wear = 42;
    GameState::Port
}

fn prepare_cargo_bay(game: &mut Game, upgraded: bool) -> GameState {
    game.port_hold_expanded = true;
    if upgraded {
        game.session.cargo_bay_level = 1;
        game.session.economy.credits = 1_000;
    } else {
        game.session.economy.credits = 100;
    }
    GameState::Port
}

fn prepare_services(
    game: &mut Game,
    credits: Option<i64>,
    alloy: i32,
    electronics: i32,
) -> GameState {
    let _ = game.capture_port_scene("port_damage");
    if let Some(credits) = credits {
        game.session.economy.credits = credits;
    }
    game.session.ship_wear = 42;
    game.session.economy.alloy = alloy;
    game.session.economy.electronics = electronics;
    game.port_tab = crate::ui::port_panel::PortTab::Service;
    GameState::Port
}

fn prepare_full_power_cells(game: &mut Game) -> GameState {
    let state = prepare_services(game, None, 1, 1);
    game.session.field_power_cells = crate::state::workspace_energy::MAX_FIELD_POWER_CELLS;
    state
}

fn prepare_loadouts(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Equipment;
    let _ = game.session.store_loadout(0);
    game.port_loadouts_open = true;
    GameState::Port
}

fn prepare_equipment_preview(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Equipment;
    game.port_selected_module = Some("scanner_module".to_owned());
    GameState::Port
}

fn prepare_heavy_equipment(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Equipment;
    game.port_stock_page = 1;
    game.port_selected_module = Some("reactor_module".to_owned());
    GameState::Port
}

fn prepare_equipment_page(game: &mut Game, scene: &str) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Equipment;
    game.port_stock_page = if scene.ends_with("last") { 2 } else { 0 };
    GameState::Port
}

fn prepare_crew_edge(game: &mut Game, scene: &str) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Crew;
    if scene.ends_with("veteran") {
        game.session.career.crew_experience[0] = crate::state::crew::MAX_CREW_EXPERIENCE;
    } else {
        game.session.economy.credits = 0;
    }
    GameState::Port
}

fn prepare_refinery(game: &mut Game) -> GameState {
    game.port_tab = crate::ui::port_panel::PortTab::Service;
    game.session.economy.alloy = 8;
    game.session.economy.electronics = 5;
    GameState::Port
}

fn prepare_sites(game: &mut Game, scene: &str) -> Option<GameState> {
    match scene {
        "sites_military" | "sites_research" | "sites_insured" | "sites_low_fuel"
        | "sites_low_credits" | "sites_details" | "sites_tired" | "sites_completed"
        | "sites_failed" => Some(GameState::SiteSelection),
        "sites" => Some(prepare_sites_default(game)),
        "sites_route_familiarity" => Some(prepare_route_familiarity(game)),
        "sites_contract_streak" => Some(prepare_contract_streak(game)),
        "sites_crew_progress" => Some(prepare_crew_progress(game)),
        "sites_crew_veteran" => Some(prepare_crew_veteran(game)),
        "sites_progress" => Some(prepare_site_progress(game)),
        "sites_private_haul" => Some(prepare_private_haul(game)),
        _ => None,
    }
}

fn prepare_sites_default(game: &mut Game) -> GameState {
    game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
    game.session.briefing_voyage_plan = game.selected_voyage_plan;
    GameState::SiteSelection
}

fn prepare_route_familiarity(game: &mut Game) -> GameState {
    prepare_sites_default(game);
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

fn prepare_contract_streak(game: &mut Game) -> GameState {
    prepare_sites_default(game);
    game.session.career.contract_streak = 2;
    game.session.career.best_contract_streak = 3;
    GameState::SiteSelection
}

fn prepare_crew_progress(game: &mut Game) -> GameState {
    prepare_sites_default(game);
    game.session.crew_role = crate::state::CrewRole::Navigator;
    game.session.career.crew_experience = [0, 4, 0, 0, 0];
    GameState::SiteSelection
}

fn prepare_crew_veteran(game: &mut Game) -> GameState {
    prepare_sites_default(game);
    game.session.crew_role = crate::state::CrewRole::Broker;
    game.session.career.crew_experience = [0, 0, 0, 0, 6];
    GameState::SiteSelection
}

fn prepare_site_progress(game: &mut Game) -> GameState {
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

fn prepare_private_haul(game: &mut Game) -> GameState {
    game.session.career.contract_streak = 2;
    game.session.career.best_contract_streak = 3;
    game.session.reputation = 3;
    game.session.voyage_log = vec![logbook::private_record()];
    GameState::SiteSelection
}

fn prepare_travel(game: &mut Game, scene: &str) -> Option<GameState> {
    match scene {
        "travel" => Some(prepare_basic_travel(game)),
        "travel_cargo_bay" => Some(prepare_cargo_bay_travel(game)),
        "travel_familiarity" => Some(prepare_familiar_travel(game)),
        "travel_cruise" => Some(prepare_cruise_travel(game)),
        "travel_scanner" => Some(prepare_scanner_travel(game)),
        _ => None,
    }
}

fn prepare_basic_travel(game: &mut Game) -> GameState {
    let _ = game.session.begin_expedition("merchant_wreck", &game.data);
    game.travel_elapsed = 2.0;
    GameState::Travel
}

fn prepare_cargo_bay_travel(game: &mut Game) -> GameState {
    game.session.cargo_bay_level = 1;
    prepare_basic_travel(game)
}

fn prepare_familiar_travel(game: &mut Game) -> GameState {
    game.session
        .site_progress
        .get_mut("merchant_wreck")
        .expect("capture site exists")
        .visits = 3;
    prepare_basic_travel(game)
}

fn prepare_cruise_travel(game: &mut Game) -> GameState {
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

fn prepare_scanner_travel(game: &mut Game) -> GameState {
    purchase_capture_module(game, "scanner_module");
    prepare_basic_travel(game)
}
