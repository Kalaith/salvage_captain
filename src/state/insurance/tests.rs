use super::*;
use crate::data::GameData;
use crate::engine::VoyagePlan;

#[test]
fn insurance_quote_scales_between_the_authored_wrecks() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        session
            .insurance_quote("merchant_wreck", &data)
            .unwrap()
            .premium,
        65
    );
    assert_eq!(
        session
            .insurance_quote("research_vessel", &data)
            .unwrap()
            .premium,
        175
    );
}

#[test]
fn insurance_departure_requires_the_premium_in_addition_to_fuel() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = 64;
    session.economy.fuel = 12;

    assert!(!session.can_depart_insured("merchant_wreck", &data));
    session.economy.credits = 65;
    assert!(session.can_depart_insured("merchant_wreck", &data));
}

#[test]
fn plan_aware_coverage_tracks_route_danger_and_intel() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    let standard = session
        .insurance_quote_with_plan("merchant_wreck", &data, VoyagePlan::Standard)
        .unwrap();
    let cautious = session
        .insurance_quote_with_plan("merchant_wreck", &data, VoyagePlan::Cautious)
        .unwrap();
    let expedited = session
        .insurance_quote_with_plan("merchant_wreck", &data, VoyagePlan::Expedited)
        .unwrap();

    assert_eq!(standard.premium, 65);
    assert!(cautious.premium < standard.premium);
    assert!(expedited.premium > standard.premium);

    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .reconnaissance_level = 1;
    let intel_quote = session
        .insurance_quote_with_plan("merchant_wreck", &data, VoyagePlan::Standard)
        .unwrap();
    assert!(intel_quote.premium < standard.premium);
}
