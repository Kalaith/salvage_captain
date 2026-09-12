//! Voyage risk outcome and mitigation rules.

use super::*;
use crate::data::RiskTuning;
use crate::engine::risk::resolve_risk;

fn tuning() -> RiskTuning {
    RiskTuning {
        safe_danger_threshold: 25,
        ordinary_return_weight: 35,
        damaged_module_weight: 20,
        lost_salvage_weight: 20,
        emergency_repair_weight: 15,
        forced_abandon_weight: 10,
        external_cargo_risk_per_item: 8,
    }
}

#[test]
fn low_adjusted_danger_is_always_a_safe_return() {
    let result = resolve_risk(99, 20, 8, ModuleStats::default(), &tuning());
    assert_eq!(result.outcome, RiskOutcome::OrdinaryReturn);
}

#[test]
fn protection_reduces_the_previewed_danger_score() {
    let unprotected = resolve_risk(1, 70, 8, ModuleStats::default(), &tuning());
    let protected = resolve_risk(
        1,
        70,
        8,
        ModuleStats {
            shielding: 30,
            scanning: 20,
            ..Default::default()
        },
        &tuning(),
    );
    assert!(protected.danger_score < unprotected.danger_score);
}
