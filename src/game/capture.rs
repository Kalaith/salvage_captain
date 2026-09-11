//! Deterministic scene setup used by the shared verification harness.

use super::{prompts, Game};
use crate::data::GridPosition;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{
    CargoStatus, GameSession, GameState, TargetSurveyNote, WorkspaceLogEntry, WorkspaceLogEvent,
};

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
        self.port_hold_expanded = false;
        self.settings_open = scene == "settings";
        self.exit_requested = false;
        self.state = match scene {
            "main_menu" => GameState::MainMenu,
            "settings" => GameState::Pause,
            "gameplay" | "port" => GameState::Port,
            "port_preview" => {
                self.port_selected_module = Some("scanner_module".to_owned());
                GameState::Port
            }
            "sites" => GameState::SiteSelection,
            "sites_progress" => {
                if let Some(progress) = self.session.site_progress.get_mut("merchant_wreck") {
                    progress.condition = 64;
                    progress.visits = 1;
                    progress.discovered_sections = vec!["cargo_bay".to_owned()];
                    progress.removed_targets = vec!["industrial_battery".to_owned()];
                    progress.operation_log = vec![
                        WorkspaceLogEntry::new(
                            1,
                            WorkspaceLogEvent::Departed,
                            Some("cargo_bay"),
                            None,
                        ),
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
                    ];
                }
                GameState::SiteSelection
            }
            "travel" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 2.0;
                GameState::Travel
            }
            "travel_cruise" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 1.6;
                GameState::Travel
            }
            "travel_scanner" => {
                let _ = self.session.purchase_module("scanner_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 2.0;
                GameState::Travel
            }
            "salvage_scan" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                GameState::SalvageWorkspace
            }
            "salvage_scanner" => {
                let _ = self.session.purchase_module("scanner_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_drones" => {
                let _ = self.session.purchase_module("drone_bay", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.4;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                let duration = self
                    .session
                    .extraction_duration("navigation_computer", &self.data)
                    .unwrap_or(4.5);
                let _ = self
                    .session
                    .reserve_workspace_energy("navigation_computer", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "navigation_computer".to_owned(),
                    elapsed: 1.6,
                    duration,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_drones_log" => {
                let _ = self.session.purchase_module("drone_bay", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.4;
                self.workspace_log_open = true;
                GameState::SalvageWorkspace
            }
            "salvage_stabilize" => {
                let _ = self.session.purchase_module("shield_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_stabilized" => {
                let _ = self.session.purchase_module("shield_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self
                    .session
                    .stabilize_workspace_target("navigation_computer", &self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_log" => {
                let _ = self.session.purchase_module("shield_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self
                    .session
                    .stabilize_workspace_target("navigation_computer", &self.data);
                let _ = self
                    .session
                    .recover_workspace_target("industrial_battery", &self.data);
                let _ = self
                    .session
                    .switch_workspace_section("engineering_access", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_log_open = true;
                GameState::SalvageWorkspace
            }
            "salvage_revisit" => {
                let survey_note =
                    self.data
                        .salvage_objects
                        .get("navigation_computer")
                        .map(|target| {
                            TargetSurveyNote::from_target(
                                "navigation_computer",
                                "cargo_bay",
                                target,
                            )
                        });
                if let Some(progress) = self.session.site_progress.get_mut("merchant_wreck") {
                    if let Some(note) = survey_note {
                        progress.surveyed_targets.push(note);
                    }
                }
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self
                    .session
                    .recover_workspace_target("industrial_battery", &self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_notice" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self
                    .session
                    .recover_workspace_target("industrial_battery", &self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_notice =
                    "INDUSTRIAL BATTERY RECOVERED  |  CARGO  |  ~160 cr".to_owned();
                self.workspace_notice_timer = 5.0;
                GameState::SalvageWorkspace
            }
            "salvage_hazard_notice" => {
                let _ = self.session.purchase_module("reactor_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 3.3;
                self.workspace_selected_target = Some("engine_assembly".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("engine_assembly", &self.data)
                    .ok();
                self.workspace_notice = "ENGINE ASSEMBLY LOST  |  TOW  |  ~1458 cr".to_owned();
                self.workspace_notice_warning = true;
                self.workspace_notice_timer = 5.0;
                GameState::SalvageWorkspace
            }
            "salvage_shift" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self
                    .session
                    .switch_workspace_section("engineering_access", &self.data);
                self.workspace_elapsed = 0.4;
                self.workspace_camera_shift = 0.42;
                GameState::SalvageWorkspace
            }
            "salvage_arrival" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self
                    .session
                    .switch_workspace_section("engineering_access", &self.data);
                self.workspace_elapsed = 0.78;
                self.workspace_camera_shift = 0.96;
                GameState::SalvageWorkspace
            }
            "salvage_extract" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("industrial_battery".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("industrial_battery", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("industrial_battery", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "industrial_battery".to_owned(),
                    elapsed: 0.0,
                    duration: 3.8,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_capture" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 5.5;
                self.workspace_selected_target = Some("industrial_battery".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("industrial_battery", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("industrial_battery", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "industrial_battery".to_owned(),
                    elapsed: 92.8,
                    duration: 100.0,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_clamp" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 3.3;
                self.workspace_selected_target = Some("titanium_plating".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("titanium_plating", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("titanium_plating", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "titanium_plating".to_owned(),
                    elapsed: 2.2,
                    duration: 4.5,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_tow" => {
                let _ = self.session.purchase_module("reactor_module", &self.data);
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 3.3;
                self.workspace_selected_target = Some("engine_assembly".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("engine_assembly", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("engine_assembly", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "engine_assembly".to_owned(),
                    elapsed: 2.2,
                    duration: 9.0,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_military" => {
                let _ = self.session.begin_expedition("military_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                GameState::SalvageWorkspace
            }
            "salvage_research" => {
                let _ = self.session.begin_expedition("research_vessel", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                GameState::SalvageWorkspace
            }
            "packing" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                GameState::SalvagePacking
            }
            "results" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
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
        self.message = prompts::state_prompt(self.state).to_owned();
        self.debug.visible = false;
        self.refresh_save_state();
    }
}
