use super::*;
use crate::data::GameData;

#[test]
fn ship_wear_scales_with_setbacks_and_operational_strain() {
    let data = GameData::load().unwrap();

    assert_eq!(
        wear_gain(RiskOutcome::OrdinaryReturn, 0, 0, &data.config.maintenance),
        4
    );
    assert_eq!(
        wear_gain(RiskOutcome::EmergencyRepair, 1, 1, &data.config.maintenance),
        27
    );
}

#[test]
fn ship_wear_adds_route_pressure_until_serviced() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_wear = 41;

    assert_eq!(session.maintenance_danger_delta(&data), 8);
    assert_eq!(session.maintenance_adjusted_danger(97, &data), 100);
    assert_eq!(session.maintenance_cost(&data), 246);
}
