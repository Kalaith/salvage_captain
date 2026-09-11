use super::*;

#[test]
fn plan_cycle_returns_to_the_cautious_start() {
    assert_eq!(VoyagePlan::Cautious.next(), VoyagePlan::Standard);
    assert_eq!(VoyagePlan::Standard.next(), VoyagePlan::Expedited);
    assert_eq!(VoyagePlan::Expedited.next(), VoyagePlan::Cautious);
}

#[test]
fn plan_adjustments_keep_fuel_affordable_and_danger_bounded() {
    let tuning = VoyagePlanTuning {
        cautious_fuel_delta: 1,
        cautious_danger_delta: -10,
        expedited_fuel_delta: -1,
        expedited_danger_delta: 10,
    };

    assert_eq!(VoyagePlan::Cautious.adjust_fuel(4, &tuning), 5);
    assert_eq!(VoyagePlan::Expedited.adjust_fuel(1, &tuning), 1);
    assert_eq!(VoyagePlan::Cautious.adjust_danger(4, &tuning), 0);
    assert_eq!(VoyagePlan::Expedited.adjust_danger(95, &tuning), 100);
}
