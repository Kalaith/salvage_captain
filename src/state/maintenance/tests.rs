use super::*;
use crate::data::GameData;

#[test]
fn repair_quote_prices_accumulated_ship_wear() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_wear = 10;
    session.hull = session.max_hull_with_modules(&data);

    let quote = session.repair_quote(&data);

    assert_eq!(quote.ship_wear, 10);
    assert_eq!(quote.wear_cost, 60);
    assert_eq!(quote.total_cost, 60);
}

#[test]
fn service_clears_ship_wear_and_reports_the_work() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_wear = 10;
    session.hull = session.max_hull_with_modules(&data);

    let message = session.repair(&data).unwrap();

    assert_eq!(session.ship_wear(), 0);
    assert!(message.starts_with("Serviced ship systems"));
}
