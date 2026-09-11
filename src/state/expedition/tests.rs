use super::*;
use crate::data::GameData;
use crate::engine::VoyagePlan;

#[test]
fn insured_departure_deducts_the_site_quote_and_marks_the_expedition() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let starting_credits = session.economy.credits;
    let starting_fuel = session.economy.fuel;
    let quote = session.insurance_quote("merchant_wreck", &data).unwrap();

    session
        .begin_expedition_with_coverage("merchant_wreck", &data, true)
        .unwrap();

    assert_eq!(session.economy.credits, starting_credits - quote.premium);
    assert_eq!(session.economy.fuel, starting_fuel - 4);
    assert!(session.expedition.as_ref().unwrap().insured);
}

#[test]
fn uninsured_departure_keeps_the_original_fuel_without_coverage() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    session.begin_expedition("merchant_wreck", &data).unwrap();

    assert_eq!(session.economy.credits, data.config.starting_credits);
    assert_eq!(session.economy.fuel, data.config.starting_fuel - 4);
    assert!(!session.expedition.as_ref().unwrap().insured);
}

#[test]
fn cautious_and_expedited_plans_trade_fuel_for_route_danger() {
    let data = GameData::load().unwrap();
    let mut cautious = GameSession::new(&data);
    cautious
        .begin_expedition_with_plan("merchant_wreck", &data, false, VoyagePlan::Cautious)
        .unwrap();
    let mut expedited = GameSession::new(&data);
    expedited
        .begin_expedition_with_plan("merchant_wreck", &data, false, VoyagePlan::Expedited)
        .unwrap();

    assert_eq!(cautious.economy.fuel, data.config.starting_fuel - 5);
    assert_eq!(expedited.economy.fuel, data.config.starting_fuel - 3);
    assert_eq!(
        cautious.expedition.as_ref().unwrap().voyage_plan,
        VoyagePlan::Cautious
    );
    assert!(
        cautious.expedition.as_ref().unwrap().risk.danger_score
            < expedited.expedition.as_ref().unwrap().risk.danger_score
    );
}

#[test]
fn worked_routes_lower_the_next_departure_risk() {
    let data = GameData::load().unwrap();
    let mut fresh = GameSession::new(&data);
    fresh.begin_expedition("merchant_wreck", &data).unwrap();

    let mut familiar = GameSession::new(&data);
    familiar
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .visits = 2;
    familiar.begin_expedition("merchant_wreck", &data).unwrap();

    assert_eq!(familiar.route_familiarity("merchant_wreck"), 2);
    assert!(
        familiar.expedition.as_ref().unwrap().risk.danger_score
            < fresh.expedition.as_ref().unwrap().risk.danger_score
    );
}
