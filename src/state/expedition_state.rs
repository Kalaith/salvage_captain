//! Serialized state for a salvage run in transit or at the wreck.

use super::{CargoItem, DroneDirective, WorkspaceScanProfile};
use crate::engine::{RiskResult, VoyagePlan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpeditionState {
    pub site_id: String,
    pub cargo: Vec<CargoItem>,
    pub risk: RiskResult,
    #[serde(default = "default_expedition_seed")]
    pub seed: u64,
    #[serde(default)]
    pub workspace_section: String,
    #[serde(default)]
    pub workspace_scanned: bool,
    #[serde(default)]
    pub revealed_targets: Vec<String>,
    #[serde(default)]
    pub stabilized_targets: Vec<String>,
    #[serde(default)]
    pub scan_profile: WorkspaceScanProfile,
    #[serde(default)]
    pub drones_deployed: bool,
    #[serde(default)]
    pub drone_directive: DroneDirective,
    #[serde(default = "default_workspace_energy")]
    pub workspace_energy: i32,
    #[serde(default = "default_workspace_energy")]
    pub workspace_energy_capacity: i32,
    #[serde(default)]
    pub power_cycles_used: u8,
    #[serde(default)]
    pub insured: bool,
    #[serde(default)]
    pub voyage_plan: VoyagePlan,
}

fn default_expedition_seed() -> u64 {
    7
}

fn default_workspace_energy() -> i32 {
    12
}
