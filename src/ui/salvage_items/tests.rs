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
fn packing_hold_forecasts_the_wear_left_by_this_return() {
    let data = crate::data::GameData::load().unwrap();

    assert_eq!(
        packing_wear_label(
            10,
            Some(crate::engine::RiskOutcome::OrdinaryReturn),
            0,
            0,
            &data.config.maintenance,
        ),
        "WEAR AFTER RETURN 14%  //  +4  //  SERVICE ¢84"
    );
}

#[test]
fn packing_hold_names_the_active_crew_beside_clamp_capacity() {
    assert_eq!(
        packing_crew_label(2, 4, crate::state::CrewRole::Rigger, 79),
        "EXTERNAL CLAMPS  2/4  //  CREW RIGGER  //  READY 79%"
    );
}

#[test]
fn packing_hold_names_internal_cargo_berths() {
    assert_eq!(packing_cargo_capacity_label(2, 3), "CARGO 02/03");
    assert_eq!(packing_cargo_capacity_label(5, 7), "CARGO 05/07");
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
    assert_eq!(
        return_policy_effect_label(
            crate::state::ReturnPolicy::Standard,
            Some(crate::engine::RiskOutcome::LostSalvage)
        ),
        "HIGHEST LOAD AT RISK"
    );
    assert_eq!(
        return_policy_effect_label(
            crate::state::ReturnPolicy::ProtectValue,
            Some(crate::engine::RiskOutcome::ForcedAbandon)
        ),
        "SAVE HIGH VALUE"
    );
}

#[test]
fn packing_hold_names_a_declined_contract() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .begin_expedition_with_plan_and_contract(
            "merchant_wreck",
            &data,
            false,
            crate::engine::VoyagePlan::Standard,
            false,
        )
        .unwrap();

    assert!(session
        .contract_objective_status("merchant_wreck", &data)
        .is_none());
    assert!(!session.expedition.as_ref().unwrap().contract_accepted);
}

#[test]
fn packing_hold_names_the_active_route_memory() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .visits = 3;

    assert_eq!(
        packing_route_label(&session, "merchant_wreck"),
        "ROUTE FAMILIAR // -12"
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
