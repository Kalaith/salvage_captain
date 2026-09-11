use super::*;
use crate::data::GameData;

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
