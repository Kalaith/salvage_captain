//! Deterministic voyage coverage quotes and setback claim settlement.

use crate::data::{InsuranceTuning, SiteData};
use crate::engine::risk::RiskOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InsuranceQuote {
    pub premium: i64,
}

pub fn quote_for(site: &SiteData, tuning: &InsuranceTuning) -> InsuranceQuote {
    quote_for_danger(site.danger, tuning)
}

pub fn quote_for_danger(danger: i32, tuning: &InsuranceTuning) -> InsuranceQuote {
    InsuranceQuote {
        premium: tuning
            .premium_base
            .saturating_add(i64::from(danger.max(0)).saturating_mul(tuning.premium_per_danger))
            .max(0),
    }
}

pub fn claim_payout(
    outcome: RiskOutcome,
    impacted_value: i64,
    emergency_bill: i64,
    tuning: &InsuranceTuning,
) -> i64 {
    let eligible = match outcome {
        RiskOutcome::DamagedModule => tuning.damaged_module_payout,
        RiskOutcome::LostSalvage | RiskOutcome::ForcedAbandon => impacted_value,
        RiskOutcome::EmergencyRepair => emergency_bill,
        RiskOutcome::OrdinaryReturn => 0,
    };
    eligible.max(0) * i64::from(tuning.coverage_percent) / 100
}
