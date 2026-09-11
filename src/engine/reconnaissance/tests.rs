use super::*;
use crate::data::ReconnaissanceTuning;

fn tuning() -> ReconnaissanceTuning {
    ReconnaissanceTuning {
        first_cost: 70,
        cost_step: 50,
        danger_reduction_per_level: 8,
        max_level: 2,
    }
}

#[test]
fn reconnaissance_quotes_rise_until_the_route_is_fully_known() {
    let tuning = tuning();

    assert_eq!(quote_for(0, &tuning).unwrap().cost, 70);
    assert_eq!(quote_for(1, &tuning).unwrap().cost, 120);
    assert_eq!(quote_for(1, &tuning).unwrap().danger_reduction, 16);
    assert!(quote_for(2, &tuning).is_none());
}

#[test]
fn reconnaissance_reduces_danger_without_crossing_zero() {
    let tuning = tuning();

    assert_eq!(danger_after_intel(70, 0, &tuning), 70);
    assert_eq!(danger_after_intel(70, 2, &tuning), 54);
    assert_eq!(danger_after_intel(5, 2, &tuning), 0);
}
