//! Deterministic scene setup used by the shared verification harness.

use super::{prompts, Game};
use crate::data::GridPosition;
use crate::engine::RiskOutcome;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{
    CargoStatus, GameSession, GameState, TargetSurveyNote, VoyageRecord, WorkspaceLogEntry,
    WorkspaceLogEvent, WorkspaceScanProfile,
};

mod logbook;
mod port;

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
        self.state = match scene {
            "main_menu" => GameState::MainMenu,
            "settings" => GameState::Pause,
            "gameplay" | "port" => GameState::Port,
            "port_crew_tired" => {
                self.session.crew_fatigue = 21;
                GameState::Port
            }
            "port_worn" => {
                self.session.ship_wear = 42;
                GameState::Port
            }
            "port_services" => {
                let _ = self.capture_port_scene("port_damage");
                self.session.ship_wear = 42;
                self.session.economy.alloy = 1;
                self.session.economy.electronics = 1;
                self.port_service_open = true;
                GameState::Port
            }
            "port_services_low_funds" => {
                let _ = self.capture_port_scene("port_damage");
                self.session.economy.credits = 50;
                self.session.ship_wear = 42;
                self.port_service_open = true;
                GameState::Port
            }
            "port_services_no_materials" => {
                let _ = self.capture_port_scene("port_damage");
                self.session.ship_wear = 42;
                self.session.economy.alloy = 0;
                self.session.economy.electronics = 0;
                self.port_service_open = true;
                GameState::Port
            }
            "port_services_full_cells" => {
                let _ = self.capture_port_scene("port_damage");
                self.session.ship_wear = 42;
                self.session.field_power_cells =
                    crate::state::workspace_energy::MAX_FIELD_POWER_CELLS;
                self.session.economy.alloy = 1;
                self.session.economy.electronics = 1;
                self.port_service_open = true;
                GameState::Port
            }
            "port_loadouts" => {
                let _ = self.session.store_loadout(0);
                self.port_loadouts_open = true;
                GameState::Port
            }
            "port_damage" | "port_repair_low_funds" | "port_repaired" => {
                self.capture_port_scene(scene)
            }
            "logbook" => {
                logbook::prepare(self);
                GameState::Port
            }
            "port_preview" => {
                self.port_selected_module = Some("scanner_module".to_owned());
                GameState::Port
            }
            "port_refinery" => {
                self.session.economy.alloy = 8;
                self.session.economy.electronics = 5;
                GameState::Port
            }
            "sites" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                GameState::SiteSelection
            }
            "sites_route_familiarity" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                for (site_id, visits) in [
                    ("merchant_wreck", 1),
                    ("military_wreck", 2),
                    ("research_vessel", 3),
                ] {
                    if let Some(progress) = self.session.site_progress.get_mut(site_id) {
                        progress.visits = visits;
                    }
                }
                GameState::SiteSelection
            }
            "sites_contract_streak" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                self.session.career.contract_streak = 2;
                self.session.career.best_contract_streak = 3;
                GameState::SiteSelection
            }
            "sites_crew_progress" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                self.session.crew_role = crate::state::CrewRole::Navigator;
                self.session.career.crew_experience = [0, 4, 0, 0, 0];
                GameState::SiteSelection
            }
            "sites_crew_veteran" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                self.session.crew_role = crate::state::CrewRole::Broker;
                self.session.career.crew_experience = [0, 0, 0, 0, 6];
                GameState::SiteSelection
            }
            "sites_progress" => {
                if let Some(progress) = self.session.site_progress.get_mut("merchant_wreck") {
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
                self.session.voyage_log = vec![VoyageRecord {
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
            "travel" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 2.0;
                GameState::Travel
            }
            "travel_familiarity" => {
                self.session
                    .site_progress
                    .get_mut("merchant_wreck")
                    .expect("capture site exists")
                    .visits = 3;
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 2.0;
                GameState::Travel
            }
            "travel_cruise" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Expedited;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                let _ = self
                    .session
                    .buy_reconnaissance("merchant_wreck", &self.data);
                let _ = self.session.begin_expedition_with_plan(
                    "merchant_wreck",
                    &self.data,
                    false,
                    self.selected_voyage_plan,
                );
                self.travel_elapsed = 1.6;
                GameState::Travel
            }
            "travel_scanner" => {
                purchase_capture_module(self, "scanner_module");
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
            "salvage_power" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.session.field_power_cells = 1;
                if let Some(expedition) = self.session.expedition.as_mut() {
                    expedition.workspace_energy = 1;
                }
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_power_cell_log" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.session.field_power_cells = 2;
                if let Some(expedition) = self.session.expedition.as_mut() {
                    expedition.workspace_energy = 2;
                }
                let _ = self.session.use_field_power_cell();
                self.workspace_elapsed = 2.0;
                self.workspace_log_open = true;
                GameState::SalvageWorkspace
            }
            "salvage_clearance" => {
                if let Some(progress) = self.session.site_progress.get_mut("merchant_wreck") {
                    progress.discovered_sections = vec!["cargo_bay".to_owned()];
                    progress.removed_targets = vec![
                        "industrial_battery".to_owned(),
                        "navigation_computer".to_owned(),
                        "engine_assembly".to_owned(),
                    ];
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
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                GameState::SalvageWorkspace
            }
            "salvage_scanner" => {
                purchase_capture_module(self, "scanner_module");
                let _ = self
                    .session
                    .buy_reconnaissance("merchant_wreck", &self.data);
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
                purchase_capture_module(self, "drone_bay");
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
            "salvage_drone_orders" => {
                purchase_capture_module(self, "drone_bay");
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self.session.cycle_drone_directive(&self.data);
                let _ = self.session.cycle_drone_directive(&self.data);
                self.workspace_elapsed = 2.4;
                self.workspace_selected_target = Some("navigation_computer".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("navigation_computer", &self.data)
                    .ok();
                GameState::SalvageWorkspace
            }
            "salvage_drones_log" => {
                purchase_capture_module(self, "drone_bay");
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                let _ = self.session.cycle_drone_directive(&self.data);
                self.workspace_elapsed = 2.4;
                self.workspace_log_open = true;
                GameState::SalvageWorkspace
            }
            "salvage_stabilize" => {
                purchase_capture_module(self, "shield_module");
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
                purchase_capture_module(self, "shield_module");
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
                purchase_capture_module(self, "shield_module");
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
                let _ = self.session.power_cycle_workspace(&self.data);
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
                purchase_capture_module(self, "reactor_module");
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
                purchase_capture_module(self, "reactor_module");
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
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                if let Some(progress) = self.session.site_progress.get_mut("merchant_wreck") {
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
                let _ = self.session.begin_expedition_with_plan(
                    "merchant_wreck",
                    &self.data,
                    true,
                    self.selected_voyage_plan,
                );
                GameState::SalvagePacking
            }
            "return_travel" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                let _ = self.session.begin_expedition_with_plan(
                    "merchant_wreck",
                    &self.data,
                    true,
                    self.selected_voyage_plan,
                );
                let cargo_ids = self
                    .session
                    .expedition
                    .as_ref()
                    .map(|expedition| {
                        expedition
                            .cargo
                            .iter()
                            .map(|item| item.object_id.clone())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                for object_id in cargo_ids {
                    let _ = self.session.auto_place(&object_id, &self.data);
                }
                let _ = self.session.leave_all_pending();
                let _ = self.session.finish_packing(&self.data);
                self.return_elapsed = 1.8;
                GameState::ReturnTravel
            }
            "results" => {
                self.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
                self.session.briefing_voyage_plan = self.selected_voyage_plan;
                let _ = self
                    .session
                    .buy_reconnaissance("merchant_wreck", &self.data);
                let _ = self.session.begin_expedition_with_plan(
                    "merchant_wreck",
                    &self.data,
                    true,
                    self.selected_voyage_plan,
                );
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
