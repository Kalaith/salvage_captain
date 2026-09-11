//! Persisted summary of a completed salvage voyage.

use super::{DroneDirective, ReturnPolicy, WorkspaceScanProfile};
use crate::engine::{RiskOutcome, VoyagePlan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoyageRecord {
    pub site_id: String,
    pub recovered_count: u32,
    pub recovered_value: i64,
    #[serde(default)]
    pub recovered_alloy: i32,
    #[serde(default)]
    pub recovered_electronics: i32,
    pub external_load: u32,
    pub risk_outcome: RiskOutcome,
    pub danger_score: i32,
    #[serde(default)]
    pub reconnaissance_level: u8,
    #[serde(default)]
    pub voyage_plan: VoyagePlan,
    #[serde(default)]
    pub return_policy: ReturnPolicy,
    pub contract_completed: bool,
    #[serde(default)]
    pub contract_failed: bool,
    #[serde(default = "default_contract_accepted")]
    pub contract_accepted: bool,
    #[serde(default)]
    pub scan_profile: WorkspaceScanProfile,
    #[serde(default)]
    pub drone_directive: DroneDirective,
    pub condition_after: i32,
    #[serde(default)]
    pub cleared_sections: Vec<String>,
    #[serde(default)]
    pub clearance_payout: i64,
    #[serde(default)]
    pub return_fuel: i32,
    #[serde(default)]
    pub market_cycle: u32,
    #[serde(default)]
    pub insured: bool,
    #[serde(default)]
    pub insurance_premium: i64,
    #[serde(default)]
    pub insurance_payout: i64,
}

fn default_contract_accepted() -> bool {
    true
}
