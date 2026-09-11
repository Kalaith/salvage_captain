use super::*;

#[test]
fn packing_manifest_names_the_current_market_band() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        packing_market_label(session.market_quote("industrial_battery", &data)),
        "MKT STEADY +0%"
    );
}

#[test]
fn packing_manifest_keeps_material_yields_beside_the_market_ask() {
    let data = GameData::load().unwrap();
    let object = data.salvage_objects.get("industrial_battery").unwrap();
    let session = GameSession::new(&data);
    let label = packing_value_label(object, session.market_quote(&object.id, &data));

    assert!(label.contains("BASE ¢160 -> ASK ¢160"));
    assert!(label.contains("A2 E3"));
}

#[test]
fn packing_hold_labels_the_claim_estimate() {
    assert_eq!(
        packing_claim_label(45, 120),
        "COVER ¢45  //  CLAIM EST ¢120"
    );
}

#[test]
fn packing_hold_names_the_reserved_return_burn() {
    assert_eq!(
        return_burn_label(7, 2),
        "RETURN BURN 2 FUEL  //  5 REMAIN AFTER DOCKING"
    );
}

#[test]
fn packing_hold_names_field_power_cost_and_savings() {
    assert_eq!(power_cycle_label(0), "FIELD POWER RESET UNUSED  //  FUEL 0");
    assert_eq!(power_cycle_label(1), "FIELD POWER RESET USED  //  FUEL -1");
}

#[test]
fn packing_hold_names_the_active_crew_beside_clamp_capacity() {
    assert_eq!(
        packing_crew_label(2, 4, crate::state::CrewRole::Rigger),
        "EXTERNAL CLAMPS  2/4  //  CREW RIGGER"
    );
}

#[test]
fn packing_hold_names_the_active_return_policy() {
    assert_eq!(
        return_policy_button_label(crate::state::ReturnPolicy::ProtectObjective),
        "POLICY  OBJECTIVE"
    );
    assert_eq!(
        crate::state::ReturnPolicy::ProtectObjective.description(),
        "sacrifice another load before the contract target"
    );
}

#[test]
fn packing_hold_forecasts_the_new_section_bounty() {
    assert_eq!(
        clearance_forecast_label(1, 140),
        "CLEARANCE 01 READY  //  BOUNTY +¢140"
    );
    assert_eq!(
        clearance_forecast_label(0, 0),
        "CLEARANCE 00 READY  //  NO NEW BOUNTY"
    );
}

#[test]
fn packing_hold_repeats_the_order_that_will_be_filed() {
    assert_eq!(
        drone_order_label(crate::state::DroneDirective::Survey, true),
        "DRONE ORDER  SURVEY // SAFETY"
    );
    assert_eq!(
        drone_order_label(crate::state::DroneDirective::PullSupport, true),
        "DRONE ORDER  PULL // SPEED"
    );
    assert_eq!(
        drone_order_label(crate::state::DroneDirective::Standby, false),
        "DRONE ORDER  STANDBY // NO ASSIST"
    );
}
