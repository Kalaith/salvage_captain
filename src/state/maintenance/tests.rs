use super::*;
use crate::data::GameData;
use crate::state::maintenance::ServicePlan;

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
fn service_clears_ship_wear() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_wear = 10;
    session.hull = session.max_hull_with_modules(&data);

    session.repair(&data).unwrap();

    assert_eq!(session.ship_wear(), 0);
}

#[test]
fn hull_patch_restores_hull_without_touching_systems_or_wear() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.hull = 6;
    session.ship_wear = 10;
    session.damaged_modules.push("engine_core".to_owned());

    let quote = session.service_quote(ServicePlan::Hull, &data);
    assert_eq!(quote.missing_hull, 4);
    assert_eq!(quote.offline_modules, 0);
    assert_eq!(quote.ship_wear, 0);
    assert_eq!(quote.total_cost, 140);

    session.service(ServicePlan::Hull, &data).unwrap();

    assert_eq!(session.hull, 10);
    assert_eq!(session.ship_wear(), 10);
    assert_eq!(session.damaged_modules, vec!["engine_core"]);
}

#[test]
fn systems_service_clears_modules_and_wear_without_repairing_hull() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.hull = 6;
    session.ship_wear = 10;
    session.damaged_modules.push("engine_core".to_owned());

    let quote = session.service_quote(ServicePlan::Systems, &data);
    assert_eq!(quote.missing_hull, 0);
    assert_eq!(quote.offline_modules, 1);
    assert_eq!(quote.ship_wear, 10);
    assert_eq!(quote.total_cost, 115);

    session.service(ServicePlan::Systems, &data).unwrap();

    assert_eq!(session.hull, 6);
    assert_eq!(session.ship_wear(), 0);
    assert!(session.damaged_modules.is_empty());
}
