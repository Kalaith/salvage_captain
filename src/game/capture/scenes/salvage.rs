//! Salvage, packing, and results capture scenes.

use super::super::{
    begin_capture_transfer, purchase_capture_module, recover_capture_cargo, return_travel,
};
use crate::game::Game;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{GameState, TargetSurveyNote, WorkspaceLogEntry, WorkspaceLogEvent};

pub(super) fn prepare(game: &mut Game, scene: &str) -> GameState {
    match scene {
        "salvage_placement" | "salvage_placement_rotated" => {
            prepare_placement(game, scene.ends_with("rotated"))
        }
        "salvage_details" => prepare_details(game),
        "salvage_scan" => prepare_scanned_workspace(game, "merchant_wreck"),
        "salvage_route_familiarity" => prepare_familiar_workspace(game),
        "salvage_power" => prepare_power_workspace(game, 1, 1),
        "salvage_power_cell_log" => prepare_power_cell_log(game),
        "salvage_clearance" => prepare_clearance(game),
        "salvage_scanner" => prepare_scanner(game),
        "salvage_drones" => prepare_drones(game),
        "salvage_drone_orders" => prepare_drone_orders(game),
        "salvage_drones_log" => prepare_drone_log(game),
        "salvage_stabilize" => prepare_stabilize(game),
        "salvage_stabilized" => prepare_stabilized(game),
        "salvage_log" => prepare_workspace_log(game),
        "salvage_revisit" => prepare_revisit(game),
        "salvage_notice" => prepare_notice(game),
        "salvage_hazard_notice" => prepare_hazard_notice(game),
        "salvage_shift" => prepare_shift(game, 0.4, 0.42),
        "salvage_arrival" => prepare_shift(game, 0.78, 0.96),
        "salvage_extract" => {
            prepare_extraction(game, "merchant_wreck", "industrial_battery", 2.0, 0.0, 3.8)
        }
        "salvage_capture" => prepare_extraction(
            game,
            "merchant_wreck",
            "industrial_battery",
            5.5,
            92.8,
            100.0,
        ),
        "salvage_clamp" => {
            prepare_extraction(game, "military_wreck", "titanium_plating", 3.3, 2.2, 4.5)
        }
        "salvage_tow" => prepare_tow_extraction(game),
        "salvage_military" => prepare_scanned_workspace(game, "military_wreck"),
        "salvage_research" => prepare_scanned_workspace(game, "research_vessel"),
        "packing" => prepare_packing(game, true),
        "packing_private_haul" => prepare_packing(game, false),
        "return_travel" => return_travel::prepare(game, false),
        "return_travel_private_haul" => return_travel::prepare(game, true),
        "paused" => GameState::Pause,
        _ => panic!("Unknown Salvage Captain capture scene: {scene}"),
    }
}

fn begin_workspace(game: &mut Game, site_id: &str) {
    let _ = game.session.begin_expedition(site_id, &game.data);
    let _ = game.session.scan_workspace(&game.data);
}

fn select_workspace_target(game: &mut Game, target_id: &str) {
    game.workspace_selected_target = Some(target_id.to_owned());
    game.workspace_risk = game
        .session
        .workspace_risk_preview(target_id, &game.data)
        .ok();
}

fn prepare_placement(game: &mut Game, rotated: bool) -> GameState {
    game.state = GameState::SalvageWorkspace;
    begin_workspace(game, "military_wreck");
    game.apply_action(crate::ui::UiAction::SelectTarget(
        "titanium_plating".to_owned(),
    ));
    game.apply_action(crate::ui::UiAction::Extract("titanium_plating".to_owned()));
    if rotated {
        game.apply_action(crate::ui::UiAction::RotateWorkspacePlacement);
    }
    game.workspace_elapsed = 2.0;
    GameState::SalvageWorkspace
}

fn prepare_details(game: &mut Game) -> GameState {
    begin_workspace(game, "merchant_wreck");
    game.workspace_elapsed = 2.0;
    select_workspace_target(game, "navigation_computer");
    game.target_details_open = true;
    GameState::SalvageWorkspace
}

fn prepare_scanned_workspace(game: &mut Game, site_id: &str) -> GameState {
    begin_workspace(game, site_id);
    game.workspace_elapsed = 2.0;
    GameState::SalvageWorkspace
}

fn prepare_familiar_workspace(game: &mut Game) -> GameState {
    game.session
        .site_progress
        .get_mut("merchant_wreck")
        .expect("capture site exists")
        .visits = 3;
    prepare_scanned_workspace(game, "merchant_wreck")
}

fn prepare_power_workspace(game: &mut Game, cells: u8, energy: i32) -> GameState {
    begin_workspace(game, "merchant_wreck");
    game.session.field_power_cells = cells;
    if let Some(expedition) = game.session.expedition.as_mut() {
        expedition.workspace_energy = energy;
    }
    game.workspace_elapsed = 2.0;
    select_workspace_target(game, "navigation_computer");
    GameState::SalvageWorkspace
}

fn prepare_power_cell_log(game: &mut Game) -> GameState {
    prepare_power_workspace(game, 2, 2);
    let _ = game.session.use_field_power_cell();
    game.workspace_log_open = true;
    GameState::SalvageWorkspace
}

fn prepare_clearance(game: &mut Game) -> GameState {
    seed_clearance_progress(game);
    prepare_scanned_workspace(game, "merchant_wreck")
}

fn seed_clearance_progress(game: &mut Game) {
    if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
        progress.discovered_sections = vec!["cargo_bay".to_owned()];
        progress.removed_targets = vec![
            "industrial_battery".to_owned(),
            "navigation_computer".to_owned(),
            "engine_assembly".to_owned(),
        ];
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
}

fn prepare_scanner(game: &mut Game) -> GameState {
    purchase_capture_module(game, "scanner_module");
    let _ = game
        .session
        .buy_reconnaissance("merchant_wreck", &game.data);
    let state = prepare_details(game);
    game.target_details_open = false;
    state
}

fn prepare_drones(game: &mut Game) -> GameState {
    purchase_capture_module(game, "drone_bay");
    begin_workspace(game, "merchant_wreck");
    game.workspace_elapsed = 2.4;
    select_workspace_target(game, "navigation_computer");
    let duration = game
        .session
        .extraction_duration("navigation_computer", &game.data)
        .unwrap_or(4.5);
    configure_extraction(game, "navigation_computer", 1.6, duration);
    GameState::SalvageWorkspace
}

fn prepare_drone_orders(game: &mut Game) -> GameState {
    purchase_capture_module(game, "drone_bay");
    begin_workspace(game, "merchant_wreck");
    let _ = game.session.cycle_drone_directive(&game.data);
    let _ = game.session.cycle_drone_directive(&game.data);
    game.workspace_elapsed = 2.4;
    select_workspace_target(game, "navigation_computer");
    GameState::SalvageWorkspace
}

fn prepare_drone_log(game: &mut Game) -> GameState {
    purchase_capture_module(game, "drone_bay");
    begin_workspace(game, "merchant_wreck");
    let _ = game.session.cycle_drone_directive(&game.data);
    game.workspace_elapsed = 2.4;
    game.workspace_log_open = true;
    GameState::SalvageWorkspace
}

fn prepare_stabilize(game: &mut Game) -> GameState {
    purchase_capture_module(game, "shield_module");
    begin_workspace(game, "merchant_wreck");
    game.workspace_elapsed = 2.0;
    select_workspace_target(game, "navigation_computer");
    GameState::SalvageWorkspace
}

fn prepare_stabilized(game: &mut Game) -> GameState {
    purchase_capture_module(game, "shield_module");
    begin_workspace(game, "merchant_wreck");
    let _ = game
        .session
        .stabilize_workspace_target("navigation_computer", &game.data);
    game.workspace_elapsed = 2.0;
    select_workspace_target(game, "navigation_computer");
    GameState::SalvageWorkspace
}

fn prepare_workspace_log(game: &mut Game) -> GameState {
    prepare_stabilized(game);
    begin_capture_transfer(game, "industrial_battery");
    let _ = game
        .session
        .recover_workspace_target("industrial_battery", &game.data);
    let _ = game
        .session
        .switch_workspace_section("engineering_access", &game.data);
    let _ = game.session.scan_workspace(&game.data);
    let _ = game.session.power_cycle_workspace(&game.data);
    game.workspace_log_open = true;
    GameState::SalvageWorkspace
}

fn prepare_revisit(game: &mut Game) -> GameState {
    add_revisit_survey_note(game);
    begin_workspace(game, "merchant_wreck");
    begin_capture_transfer(game, "industrial_battery");
    let _ = game
        .session
        .recover_workspace_target("industrial_battery", &game.data);
    game.workspace_elapsed = 2.0;
    select_workspace_target(game, "navigation_computer");
    GameState::SalvageWorkspace
}

fn add_revisit_survey_note(game: &mut Game) {
    let survey_note = game
        .data
        .salvage_objects
        .get("navigation_computer")
        .map(|target| TargetSurveyNote::from_target("navigation_computer", "cargo_bay", target));
    if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
        if let Some(note) = survey_note {
            progress.surveyed_targets.push(note);
        }
    }
}

fn prepare_notice(game: &mut Game) -> GameState {
    begin_workspace(game, "merchant_wreck");
    begin_capture_transfer(game, "industrial_battery");
    let _ = game
        .session
        .recover_workspace_target("industrial_battery", &game.data);
    game.workspace_elapsed = 2.0;
    game.workspace_notice = "INDUSTRIAL BATTERY RECOVERED  |  CARGO  |  ~160 cr".to_owned();
    game.workspace_notice_timer = 5.0;
    GameState::SalvageWorkspace
}

fn prepare_hazard_notice(game: &mut Game) -> GameState {
    purchase_capture_module(game, "reactor_module");
    begin_workspace(game, "merchant_wreck");
    game.workspace_elapsed = 3.3;
    select_workspace_target(game, "engine_assembly");
    game.workspace_notice = "ENGINE ASSEMBLY LOST  |  TOW  |  ~1458 cr".to_owned();
    game.workspace_notice_warning = true;
    game.workspace_notice_timer = 5.0;
    GameState::SalvageWorkspace
}

fn prepare_shift(game: &mut Game, elapsed: f32, camera_shift: f32) -> GameState {
    begin_workspace(game, "merchant_wreck");
    let _ = game
        .session
        .switch_workspace_section("engineering_access", &game.data);
    game.workspace_elapsed = elapsed;
    game.workspace_camera_shift = camera_shift;
    GameState::SalvageWorkspace
}

fn prepare_extraction(
    game: &mut Game,
    site_id: &str,
    target_id: &str,
    workspace_elapsed: f32,
    extraction_elapsed: f32,
    duration: f32,
) -> GameState {
    begin_workspace(game, site_id);
    game.workspace_elapsed = workspace_elapsed;
    select_workspace_target(game, target_id);
    configure_extraction(game, target_id, extraction_elapsed, duration);
    GameState::SalvageWorkspace
}

fn configure_extraction(game: &mut Game, target_id: &str, elapsed: f32, duration: f32) {
    begin_capture_transfer(game, target_id);
    game.workspace_extraction = Some(ExtractionRuntime {
        target_id: target_id.to_owned(),
        elapsed,
        duration,
        resolved: false,
    });
}

fn prepare_tow_extraction(game: &mut Game) -> GameState {
    purchase_capture_module(game, "reactor_module");
    prepare_extraction(game, "merchant_wreck", "engine_assembly", 3.3, 2.2, 9.0)
}

fn prepare_packing(game: &mut Game, insured: bool) -> GameState {
    game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
    game.session.briefing_voyage_plan = game.selected_voyage_plan;
    if insured {
        if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
            progress.visits = 3;
        }
    }
    if insured {
        let _ = game.session.begin_expedition_with_plan(
            "merchant_wreck",
            &game.data,
            true,
            game.selected_voyage_plan,
        );
    } else {
        let _ = game.session.begin_expedition_with_plan_and_contract(
            "merchant_wreck",
            &game.data,
            false,
            game.selected_voyage_plan,
            false,
        );
    }
    recover_capture_cargo(game);
    game.state = GameState::SalvageWorkspace;
    game.apply_action(crate::ui::UiAction::ViewInventory);
    game.state
}
