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
        hull_cost: 0,
        module_cost: 55,
        wear_cost: 252,
        total_cost: 307,
    });

    assert_eq!(label, "RESTORES HULL 0  //  MODULES 1  //  WEAR 42%");
}

#[test]
fn service_cost_label_explains_each_plan_invoice() {
    let full = crate::state::maintenance::ServiceQuote {
        plan: ServicePlan::Full,
        missing_hull: 3,
        offline_modules: 1,
        ship_wear: 10,
        hull_cost: 105,
        module_cost: 55,
        wear_cost: 60,
        total_cost: 220,
    };
    let systems = crate::state::maintenance::ServiceQuote {
        plan: ServicePlan::Systems,
        missing_hull: 0,
        offline_modules: 1,
        ship_wear: 10,
        hull_cost: 0,
        module_cost: 55,
        wear_cost: 60,
        total_cost: 115,
    };

    assert_eq!(
        service_cost_label(full),
        "COST HULL ¢105  //  MODULES ¢55  //  WEAR ¢60"
    );
    assert_eq!(
        service_cost_label(systems),
        "COST MODULES ¢55  //  WEAR ¢60"
    );
}
