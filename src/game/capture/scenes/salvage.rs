//! Salvage, packing, and results capture scenes.

use super::super::{purchase_capture_module, return_travel};
use crate::data::GridPosition;
use crate::game::Game;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{
    CargoStatus, GameState, TargetSurveyNote, WorkspaceLogEntry, WorkspaceLogEvent,
};

pub(super) fn prepare(game: &mut Game, scene: &str) -> GameState {
    match scene {
        "salvage_details" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            game.target_details_open = true;
            GameState::SalvageWorkspace
        }
        "salvage_scan" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            GameState::SalvageWorkspace
        }
        "salvage_route_familiarity" => {
            game.session
                .site_progress
                .get_mut("merchant_wreck")
                .expect("capture site exists")
                .visits = 3;
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            GameState::SalvageWorkspace
        }
        "salvage_power" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.session.field_power_cells = 1;
            if let Some(expedition) = game.session.expedition.as_mut() {
                expedition.workspace_energy = 1;
            }
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_power_cell_log" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.session.field_power_cells = 2;
            if let Some(expedition) = game.session.expedition.as_mut() {
                expedition.workspace_energy = 2;
            }
            let _ = game.session.use_field_power_cell();
            game.workspace_elapsed = 2.0;
            game.workspace_log_open = true;
            GameState::SalvageWorkspace
        }
        "salvage_clearance" => {
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
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            GameState::SalvageWorkspace
        }
        "salvage_scanner" => {
            purchase_capture_module(game, "scanner_module");
            let _ = game
                .session
                .buy_reconnaissance("merchant_wreck", &game.data);
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_drones" => {
            purchase_capture_module(game, "drone_bay");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.4;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            let duration = game
                .session
                .extraction_duration("navigation_computer", &game.data)
                .unwrap_or(4.5);
            let _ = game
                .session
                .reserve_workspace_energy("navigation_computer", &game.data);
            game.workspace_extraction = Some(ExtractionRuntime {
                target_id: "navigation_computer".to_owned(),
                elapsed: 1.6,
                duration,
                resolved: false,
            });
            GameState::SalvageWorkspace
        }
        "salvage_drone_orders" => {
            purchase_capture_module(game, "drone_bay");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game.session.cycle_drone_directive(&game.data);
            let _ = game.session.cycle_drone_directive(&game.data);
            game.workspace_elapsed = 2.4;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_drones_log" => {
            purchase_capture_module(game, "drone_bay");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game.session.cycle_drone_directive(&game.data);
            game.workspace_elapsed = 2.4;
            game.workspace_log_open = true;
            GameState::SalvageWorkspace
        }
        "salvage_stabilize" => {
            purchase_capture_module(game, "shield_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_stabilized" => {
            purchase_capture_module(game, "shield_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game
                .session
                .stabilize_workspace_target("navigation_computer", &game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_log" => {
            purchase_capture_module(game, "shield_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game
                .session
                .stabilize_workspace_target("navigation_computer", &game.data);
            let _ = game
                .session
                .recover_workspace_target("industrial_battery", &game.data);
            let _ = game
                .session
                .switch_workspace_section("engineering_access", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game.session.power_cycle_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_log_open = true;
            GameState::SalvageWorkspace
        }
        "salvage_revisit" => {
            let survey_note = game
                .data
                .salvage_objects
                .get("navigation_computer")
                .map(|target| {
                    TargetSurveyNote::from_target("navigation_computer", "cargo_bay", target)
                });
            if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
                if let Some(note) = survey_note {
                    progress.surveyed_targets.push(note);
                }
            }
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game
                .session
                .recover_workspace_target("industrial_battery", &game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("navigation_computer".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("navigation_computer", &game.data)
                .ok();
            GameState::SalvageWorkspace
        }
        "salvage_notice" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            let _ = game
                .session
                .recover_workspace_target("industrial_battery", &game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_notice = "INDUSTRIAL BATTERY RECOVERED  |  CARGO  |  ~160 cr".to_owned();
            game.workspace_notice_timer = 5.0;
            GameState::SalvageWorkspace
        }
        "salvage_hazard_notice" => {
            purchase_capture_module(game, "reactor_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 3.3;
            game.workspace_selected_target = Some("engine_assembly".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("engine_assembly", &game.data)
                .ok();
            game.workspace_notice = "ENGINE ASSEMBLY LOST  |  TOW  |  ~1458 cr".to_owned();
            game.workspace_notice_warning = true;
            game.workspace_notice_timer = 5.0;
            GameState::SalvageWorkspace
        }
        "salvage_shift" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game
                .session
                .switch_workspace_section("engineering_access", &game.data);
            game.workspace_elapsed = 0.4;
            game.workspace_camera_shift = 0.42;
            GameState::SalvageWorkspace
        }
        "salvage_arrival" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game
                .session
                .switch_workspace_section("engineering_access", &game.data);
            game.workspace_elapsed = 0.78;
            game.workspace_camera_shift = 0.96;
            GameState::SalvageWorkspace
        }
        "salvage_extract" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            game.workspace_selected_target = Some("industrial_battery".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("industrial_battery", &game.data)
                .ok();
            let _ = game
                .session
                .reserve_workspace_energy("industrial_battery", &game.data);
            game.workspace_extraction = Some(ExtractionRuntime {
                target_id: "industrial_battery".to_owned(),
                elapsed: 0.0,
                duration: 3.8,
                resolved: false,
            });
            GameState::SalvageWorkspace
        }
        "salvage_capture" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 5.5;
            game.workspace_selected_target = Some("industrial_battery".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("industrial_battery", &game.data)
                .ok();
            let _ = game
                .session
                .reserve_workspace_energy("industrial_battery", &game.data);
            game.workspace_extraction = Some(ExtractionRuntime {
                target_id: "industrial_battery".to_owned(),
                elapsed: 92.8,
                duration: 100.0,
                resolved: false,
            });
            GameState::SalvageWorkspace
        }
        "salvage_clamp" => {
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 3.3;
            game.workspace_selected_target = Some("titanium_plating".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("titanium_plating", &game.data)
                .ok();
            let _ = game
                .session
                .reserve_workspace_energy("titanium_plating", &game.data);
            game.workspace_extraction = Some(ExtractionRuntime {
                target_id: "titanium_plating".to_owned(),
                elapsed: 2.2,
                duration: 4.5,
                resolved: false,
            });
            GameState::SalvageWorkspace
        }
        "salvage_tow" => {
            purchase_capture_module(game, "reactor_module");
            let _ = game.session.begin_expedition("merchant_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 3.3;
            game.workspace_selected_target = Some("engine_assembly".to_owned());
            game.workspace_risk = game
                .session
                .workspace_risk_preview("engine_assembly", &game.data)
                .ok();
            let _ = game
                .session
                .reserve_workspace_energy("engine_assembly", &game.data);
            game.workspace_extraction = Some(ExtractionRuntime {
                target_id: "engine_assembly".to_owned(),
                elapsed: 2.2,
                duration: 9.0,
                resolved: false,
            });
            GameState::SalvageWorkspace
        }
        "salvage_military" => {
            let _ = game.session.begin_expedition("military_wreck", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            GameState::SalvageWorkspace
        }
        "salvage_research" => {
            let _ = game.session.begin_expedition("research_vessel", &game.data);
            let _ = game.session.scan_workspace(&game.data);
            game.workspace_elapsed = 2.0;
            GameState::SalvageWorkspace
        }
        "packing" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            if let Some(progress) = game.session.site_progress.get_mut("merchant_wreck") {
                progress.visits = 3;
                progress.removed_targets = vec![
                    "industrial_battery".to_owned(),
                    "navigation_computer".to_owned(),
                    "engine_assembly".to_owned(),
                ];
                progress.operation_log = vec![
                    WorkspaceLogEntry::new(
                        1,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("industrial_battery"),
                    ),
                    WorkspaceLogEntry::new(
                        2,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("navigation_computer"),
                    ),
                    WorkspaceLogEntry::new(
                        3,
                        WorkspaceLogEvent::TargetRecovered,
                        Some("cargo_bay"),
                        Some("engine_assembly"),
                    ),
                ];
            }
            let _ = game.session.begin_expedition_with_plan(
                "merchant_wreck",
                &game.data,
                true,
                game.selected_voyage_plan,
            );
            GameState::SalvagePacking
        }
        "packing_private_haul" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            let _ = game.session.begin_expedition_with_plan_and_contract(
                "merchant_wreck",
                &game.data,
                false,
                game.selected_voyage_plan,
                false,
            );
            GameState::SalvagePacking
        }
        "return_travel" => return_travel::prepare(game, false),
        "return_travel_private_haul" => return_travel::prepare(game, true),
        "results" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            let _ = game
                .session
                .buy_reconnaissance("merchant_wreck", &game.data);
            let _ = game.session.begin_expedition_with_plan(
                "merchant_wreck",
                &game.data,
                true,
                game.selected_voyage_plan,
            );
            let _ = game.session.scan_workspace(&game.data);
            if let Some(expedition) = game.session.expedition.as_mut() {
                for item in &mut expedition.cargo {
                    if item.status == CargoStatus::Pending {
                        item.status = CargoStatus::Packed;
                        item.position = Some(GridPosition::new(2, 2));
                    }
                }
            }
            let _ = game.session.finish_packing(&game.data);
            GameState::Results
        }
        "results_private_haul" => {
            game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
            game.session.briefing_voyage_plan = game.selected_voyage_plan;
            let _ = game.session.begin_expedition_with_plan_and_contract(
                "merchant_wreck",
                &game.data,
                false,
                game.selected_voyage_plan,
                false,
            );
            let _ = game.session.scan_workspace(&game.data);
            if let Some(expedition) = game.session.expedition.as_mut() {
                for item in &mut expedition.cargo {
                    if item.status == CargoStatus::Pending {
                        item.status = CargoStatus::Packed;
                        item.position = Some(GridPosition::new(2, 2));
                    }
                }
            }
            let _ = game.session.finish_packing(&game.data);
            GameState::Results
        }
        "paused" => GameState::Pause,
        _ => panic!("Unknown Salvage Captain capture scene: {scene}"),
    }
}
