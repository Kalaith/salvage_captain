//! Seeded, readable expedition setbacks with visible danger inputs.

use crate::data::RiskTuning;
use crate::engine::ModuleStats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskOutcome {
    OrdinaryReturn,
    DamagedModule,
    LostSalvage,
    EmergencyRepair,
    ForcedAbandon,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskResult {
    pub outcome: RiskOutcome,
    pub danger_score: i32,
    pub explanation: String,
}

pub fn resolve_risk(
    seed: u64,
    danger: i32,
    hull: i32,
    stats: ModuleStats,
    tuning: &RiskTuning,
) -> RiskResult {
    let protection = stats.shielding + stats.scanning / 2 + (hull + stats.hull) * 2;
    let adjusted = (danger - protection).clamp(0, 100);
    let roll = ((seed ^ (seed >> 17)).wrapping_mul(0x9E37_79B9) % 100) as i32;
    let outcome = if adjusted <= tuning.safe_danger_threshold
        || roll >= adjusted
        || tuning.ordinary_return_weight >= 100
    {
        RiskOutcome::OrdinaryReturn
    } else {
        let failure_total = 100 - tuning.ordinary_return_weight;
        let failure_roll =
            ((seed.rotate_left(23) ^ seed.rotate_right(11)) % failure_total as u64) as i32;
        let damaged_limit = tuning.damaged_module_weight;
        let lost_limit = damaged_limit + tuning.lost_salvage_weight;
        let repair_limit = lost_limit + tuning.emergency_repair_weight;
        if failure_roll < damaged_limit {
            RiskOutcome::DamagedModule
        } else if failure_roll < lost_limit {
            RiskOutcome::LostSalvage
        } else if failure_roll < repair_limit {
            RiskOutcome::EmergencyRepair
        } else {
            RiskOutcome::ForcedAbandon
        }
    };
    let explanation = match outcome {
        RiskOutcome::OrdinaryReturn => "The return corridor held. No setback.".to_owned(),
        RiskOutcome::DamagedModule => "A shockwave damaged one ship system.".to_owned(),
        RiskOutcome::LostSalvage => "A loose crate tore free during the return burn.".to_owned(),
        RiskOutcome::EmergencyRepair => {
            "The drive came home hot; the yard wants a repair bill.".to_owned()
        }
        RiskOutcome::ForcedAbandon => {
            "A pressure alarm forced you to abandon one packed item.".to_owned()
        }
    };
    RiskResult {
        outcome,
        danger_score: adjusted,
        explanation,
    }
}

#[cfg(test)]
mod tests;
