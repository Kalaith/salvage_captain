//! Authoritative runtime state, explicit screen states, and versioned saves.

pub mod career;
mod cargo_actions;
pub mod cargo_bay;
pub mod contracts;
pub mod crew;
pub mod crew_readiness;
pub mod discovery;
pub mod drone_directive;
pub mod expedition;
mod expedition_state;
pub mod insurance;
pub mod ledger;
pub mod loadout;
pub mod main_menu;
pub mod maintenance;
pub mod market;
pub mod pause;
mod persistence;
pub mod port;
pub mod progression;
pub mod reconnaissance;
pub mod refinery;
pub mod reputation;
pub mod results;
pub mod return_policy;
pub mod route_familiarity;
pub mod salvage_packing;
pub mod scan_profile;
pub mod section_clearance;
pub mod ship_wear;
pub mod site_selection;
pub mod survey;
pub mod validation;
mod voyage_record;
pub mod workspace;
pub mod workspace_condition;
mod workspace_drones;
pub mod workspace_energy;
pub mod workspace_records;
pub mod wreck_readiness;
pub mod wrecks;

pub use career::{CareerAward, CareerStats};
pub use crew::CrewRole;
pub use drone_directive::DroneDirective;
pub use expedition_state::ExpeditionState;
pub use ledger::VoyageRecordInput;
pub use return_policy::ReturnPolicy;
pub use scan_profile::WorkspaceScanProfile;
pub use voyage_record::VoyageRecord;
pub use workspace_records::{TargetSurveyNote, WorkspaceLogEntry, WorkspaceLogEvent};

use crate::data::{GameData, GridPosition};
use crate::engine::VoyagePlan;
use crate::engine::{installation, progression as engine_progression};
use crate::engine::{resolve_disposition, Disposition, RiskOutcome, RiskResult, ShipLayout};
use crate::state::workspace::TransferMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Port,
    SiteSelection,
    Travel,
    SalvageWorkspace,
    CargoInventory,
    ReturnTravel,
    Results,
    Pause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)] // Destination names make the state graph self-documenting.
pub enum StateTransition {
    ToMainMenu,
    ToPort,
    ToSiteSelection,
    ToTravel,
    ToSalvageWorkspace,
    ToInventory,
    ToReturnTravel,
    ToResults,
    ToPause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyState {
    pub credits: i64,
    pub fuel: i32,
    pub alloy: i32,
    pub electronics: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteProgress {
    pub condition: i32,
    pub visits: u32,
    #[serde(default)]
    pub discovered_sections: Vec<String>,
    #[serde(default)]
    pub removed_targets: Vec<String>,
    #[serde(default)]
    pub operation_log: Vec<WorkspaceLogEntry>,
    #[serde(default)]
    pub surveyed_targets: Vec<TargetSurveyNote>,
    #[serde(default)]
    pub contract_completed: bool,
    #[serde(default)]
    pub contract_failed: bool,
    #[serde(default)]
    pub reconnaissance_level: u8,
    #[serde(default)]
    pub cleared_sections: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CargoStatus {
    Pending,
    Packed,
    LeftBehind,
    Discarded,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItem {
    pub object_id: String,
    pub status: CargoStatus,
    pub position: Option<GridPosition>,
    pub rotation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnedItem {
    pub object_id: String,
    pub position: GridPosition,
    pub rotation: u8,
    #[serde(default)]
    pub market_cycle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub object_id: String,
    pub disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    #[serde(default)]
    pub wrecks: Vec<wrecks::WreckInstance>,
    #[serde(default)]
    pub discovery_serial: u64,
    #[serde(default = "default_discovery_seed")]
    pub discovery_seed: u64,
    pub economy: EconomyState,
    pub ship_layout: ShipLayout,
    pub hull: i32,
    pub max_hull: i32,
    pub damaged_modules: Vec<String>,
    pub site_progress: HashMap<String, SiteProgress>,
    pub selected_site: Option<String>,
    #[serde(default)]
    pub briefing_voyage_plan: VoyagePlan,
    pub expedition: Option<ExpeditionState>,
    pub returned: Vec<ReturnedItem>,
    pub last_risk: Option<RiskResult>,
    pub decisions: Vec<DecisionRecord>,
    #[serde(default)]
    pub voyage_log: Vec<VoyageRecord>,
    #[serde(default)]
    pub career: CareerStats,
    #[serde(default)]
    pub crew_role: CrewRole,
    #[serde(default)]
    pub crew_fatigue: u8,
    #[serde(default)]
    pub ship_wear: u8,
    #[serde(default)]
    pub field_power_cells: u8,
    #[serde(default)]
    pub cargo_bay_level: u8,
    #[serde(default)]
    pub last_return_policy: ReturnPolicy,
    #[serde(default)]
    pub loadout_slots: [Option<loadout::LoadoutPreset>; loadout::SLOT_COUNT],
    pub unlocked_modules: Vec<String>,
    pub milestone_reached: bool,
    #[serde(default)]
    pub reputation: i32,
    #[serde(default)]
    pub market_cycle: u32,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub session: GameSession,
}

impl GameSession {
    pub fn new(data: &GameData) -> Self {
        let mut ship_layout = ShipLayout::new(data.config.grid_width, data.config.grid_height);
        for starting in &data.config.starting_modules {
            let module = data
                .modules
                .get(&starting.module_id)
                .expect("validated starting module");
            ship_layout
                .place(
                    module.id.clone(),
                    module.footprint,
                    starting.position,
                    starting.rotation,
                    true,
                )
                .expect("validated starting layout");
        }
        let site_progress = data
            .sites
            .iter()
            .filter(|(id, _)| !id.starts_with("wreck:"))
            .map(|(id, site)| (id.clone(), SiteProgress::fresh(site.condition)))
            .collect();
        let unlocked_modules = data
            .config
            .starting_modules
            .iter()
            .map(|module| module.module_id.clone())
            .collect();
        let mut session = Self {
            wrecks: Vec::new(),
            discovery_serial: 0,
            discovery_seed: default_discovery_seed(),
            economy: EconomyState {
                credits: data.config.starting_credits,
                fuel: data.config.starting_fuel,
                alloy: 0,
                electronics: 0,
            },
            ship_layout,
            hull: data.config.starting_hull,
            max_hull: data.config.max_hull,
            damaged_modules: Vec::new(),
            site_progress,
            selected_site: None,
            briefing_voyage_plan: VoyagePlan::Standard,
            expedition: None,
            returned: Vec::new(),
            last_risk: None,
            decisions: Vec::new(),
            voyage_log: Vec::new(),
            career: CareerStats::default(),
            crew_role: CrewRole::default(),
            crew_fatigue: 0,
            ship_wear: 0,
            field_power_cells: 0,
            cargo_bay_level: 0,
            last_return_policy: ReturnPolicy::default(),
            loadout_slots: [None, None, None],
            unlocked_modules,
            milestone_reached: false,
            reputation: 0,
            market_cycle: 0,
            seed: 7,
        };
        session.refresh_module_unlocks(data);
        session
    }

    pub fn module_stats(&self, data: &GameData) -> engine_progression::ModuleStats {
        engine_progression::stats_from_layout(&self.ship_layout, data, &self.damaged_modules)
    }

    pub fn max_hull_with_modules(&self, data: &GameData) -> i32 {
        self.max_hull + self.module_stats(data).hull
    }

    pub fn effective_fuel_cost(&self, site_id: &str, data: &GameData) -> Option<i32> {
        self.effective_fuel_cost_with_plan(site_id, data, VoyagePlan::Standard)
    }

    pub fn effective_fuel_cost_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> Option<i32> {
        let site = data.sites.get(site_id)?;
        let stats = self.module_stats(data);
        Some(plan.adjust_fuel(
            (site.fuel_cost - stats.fuel_efficiency + self.crew_fuel_delta()).max(1),
            &data.config.voyage_plan,
        ))
    }

    pub fn departure_fuel_required(&self, site_id: &str, data: &GameData) -> Option<i32> {
        Some(self.effective_fuel_cost(site_id, data)? + data.config.safe_return_buffer)
    }

    pub fn departure_fuel_required_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> Option<i32> {
        Some(
            self.effective_fuel_cost_with_plan(site_id, data, plan)?
                + data.config.safe_return_buffer,
        )
    }

    pub fn can_depart(&self, site_id: &str, data: &GameData) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && !self.wreck_depleted(site_id, data)
            && self
                .departure_fuel_required(site_id, data)
                .is_some_and(|required| self.economy.fuel >= required)
    }

    pub fn can_depart_with_plan(&self, site_id: &str, data: &GameData, plan: VoyagePlan) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && !self.wreck_depleted(site_id, data)
            && self
                .departure_fuel_required_with_plan(site_id, data, plan)
                .is_some_and(|required| self.economy.fuel >= required)
    }
}

fn default_discovery_seed() -> u64 {
    971
}

fn cargo_layout_id(object_id: &str) -> String {
    format!("cargo:{object_id}")
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: serde_json::Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);
    let mut save: SaveData = serde_json::from_value(payload)
        .map_err(|error| format!("Unsupported save format {:?}: {error}", detected_version))?;
    save.version = data.config.version.clone();
    GameSession::from_save(save.clone(), data)?;
    Ok(save)
}
