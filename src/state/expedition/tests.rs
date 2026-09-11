use super::*;
use crate::data::GameData;

#[test]
fn insured_departure_deducts_the_site_quote_and_marks_the_expedition() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let starting_credits = session.economy.credits;
    let starting_fuel = session.economy.fuel;
    let quote = session.insurance_quote("merchant_wreck", &data).unwrap();

    let message = session
        .begin_expedition_with_coverage("merchant_wreck", &data, true)
        .unwrap();

    assert_eq!(session.economy.credits, starting_credits - quote.premium);
    assert_eq!(session.economy.fuel, starting_fuel - 4);
    assert!(session.expedition.as_ref().unwrap().insured);
    assert!(message.contains("Coverage secured for 65 credits"));
}

#[test]
fn uninsured_departure_keeps_the_original_fuel_and_message_contract() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    let message = session.begin_expedition("merchant_wreck", &data).unwrap();

    assert_eq!(session.economy.credits, data.config.starting_credits);
    assert_eq!(session.economy.fuel, data.config.starting_fuel - 4);
    assert!(!session.expedition.as_ref().unwrap().insured);
    assert!(message.starts_with("Travelled to Merchant Wreck for 4 fuel."));
}
