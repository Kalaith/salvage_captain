use super::*;
use crate::engine::refinery::RefineryResource;

#[test]
fn refining_a_material_batch_returns_shipyard_credits() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.alloy = 7;
    let before = session.economy.credits;

    let message = session
        .refine_resource(RefineryResource::Alloy, &data)
        .unwrap();

    assert_eq!(session.economy.alloy, 2);
    assert_eq!(session.economy.credits, before + 100);
    assert!(message.contains("Refined 5 ALLOY for 100 credits"));
}

#[test]
fn refining_rejects_an_incomplete_batch_without_mutating_stock() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.electronics = 2;
    let before = session.economy.credits;

    let error = session
        .refine_resource(RefineryResource::Electronics, &data)
        .unwrap_err();

    assert!(error.contains("Need 3 ELEC"));
    assert_eq!(session.economy.electronics, 2);
    assert_eq!(session.economy.credits, before);
}
