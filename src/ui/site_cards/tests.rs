use super::*;
use crate::state::WorkspaceScanProfile;

#[test]
fn mission_briefing_names_the_next_ship_blueprint() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let label = blueprint_progress_label(&session, &data);

    assert!(label.contains("SHIP BLUEPRINTS 3/9"));
    assert!(label.contains("NEXT NAV MODULE @ ¢900"));
    assert!(label.contains("STAND INDEPENDENT // REP 0/2"));
}

#[test]
fn mission_briefing_keeps_blueprints_on_empty_last_run_line() {
    let label = site_last_run_label(None);
    let memory_label = site_last_run_memory_label(None, 0, 0, 3, 9, "REP 0/2");

    assert_eq!(label, "LAST RUN  NONE");
    assert_eq!(
        memory_label,
        "LOG 00  //  SURV 00  //  BP 03/09  //  REP 0/2"
    );
}

#[test]
fn mission_briefing_keeps_blueprints_on_completed_last_run_line() {
    let record = VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 2,
        recovered_value: 250,
        recovered_alloy: 5,
        recovered_electronics: 2,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 80,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    };
    let label = site_last_run_label(Some(&record));
    let memory_label = site_last_run_memory_label(Some(&record), 4, 3, 5, 9, "REP 4/7");

    assert_eq!(label, "LAST ORDINARY RETURN  //  TGT 2  //  HOME 2 FUEL");
    assert_eq!(
        memory_label,
        "VALUE ¢250  //  LOG 04  //  SURV 03  //  BP 05/09  //  REP 4/7"
    );
}

#[test]
fn site_cards_reduce_standing_to_a_compact_progress_label() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.reputation = 4;

    assert_eq!(site_standing_progress_label(&session), "REP 4/7");
}

#[test]
fn site_cards_preview_the_strongest_buyer_demand_in_the_wreck() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let site = data.sites.get("merchant_wreck").unwrap();
    let label = site_market_outlook_label(site, &session, &data);

    assert!(label.starts_with("MKT "));
    assert!(label.ends_with('%'));
}

#[test]
fn site_cards_price_optional_coverage_before_departure() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let quote = session.insurance_quote("merchant_wreck", &data);

    assert_eq!(insurance_button_label(quote, true, true), "COVER ¢65");
    assert_eq!(insurance_button_label(quote, true, false), "LOW CR ¢65");
}

#[test]
fn mission_briefing_exposes_route_intelligence_progress_and_price() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let quote = session.reconnaissance_quote("merchant_wreck", &data);

    assert_eq!(
        site_reconnaissance_label(&session, "merchant_wreck", &data),
        "INTEL 0/2"
    );
    assert_eq!(reconnaissance_button_label(quote, true, true), "INTEL ¢70");
}

#[test]
fn mission_briefing_shows_the_danger_reduction_from_saved_intel() {
    let data = GameData::load().unwrap();
    let site = data.sites.get("military_wreck").unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("military_wreck")
        .unwrap()
        .reconnaissance_level = 1;

    assert_eq!(
        site_danger_label(site, &session, &data, crate::engine::VoyagePlan::Standard),
        "DANGER  45% -> 37%  //  INTEL -8"
    );
}

#[test]
fn mission_briefing_exposes_route_wear_pressure() {
    let data = GameData::load().unwrap();
    let site = data.sites.get("military_wreck").unwrap();
    let mut session = GameSession::new(&data);
    session.ship_wear = 41;

    assert_eq!(
        site_danger_label(site, &session, &data, crate::engine::VoyagePlan::Standard),
        "DANGER  45% -> 53%  //  WEAR +8"
    );
}

#[test]
fn mission_briefing_names_the_remaining_section_bounty() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        site_clearance_label(&session, "merchant_wreck", &data),
        "CLR 0/2 // +¢320 LEFT"
    );
}

#[test]
fn mission_briefing_separates_paid_and_remaining_section_bounties() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .cleared_sections = vec!["cargo_bay".to_owned()];

    assert_eq!(
        site_clearance_label(&session, "merchant_wreck", &data),
        "CLR 1/2 // PAID +¢140 // LEFT +¢180"
    );
}
