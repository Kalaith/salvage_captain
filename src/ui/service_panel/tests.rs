use super::*;

#[test]
fn service_plans_name_their_tradeoffs() {
    assert_eq!(ServicePlan::Full.label(), "FULL OVERHAUL");
    assert_eq!(ServicePlan::Hull.button_label(), "PATCH HULL");
    assert!(ServicePlan::Systems
        .description()
        .contains("accumulated wear"));
}

#[test]
fn service_scope_label_reports_the_parts_a_plan_will_restore() {
    let label = service_scope_label(crate::state::maintenance::ServiceQuote {
        plan: ServicePlan::Systems,
        missing_hull: 0,
        offline_modules: 1,
        ship_wear: 42,
        total_cost: 307,
    });

    assert_eq!(label, "RESTORES HULL 0  //  MODULES 1  //  WEAR 42%");
}
