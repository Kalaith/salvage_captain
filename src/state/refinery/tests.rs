use super::*;
use crate::engine::refinery::RefineryResource;

#[test]
fn refining_a_material_batch_returns_shipyard_credits() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.alloy = 7;
    let before = session.economy.credits;

    session
        .refine_resource(RefineryResource::Alloy, &data)
        .unwrap();

    assert_eq!(session.economy.alloy, 2);
    assert_eq!(session.economy.credits, before + 100);
}

#[test]
fn refining_rejects_an_incomplete_batch_without_mutating_stock() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.electronics = 2;
    let before = session.economy.credits;

    assert!(session
        .refine_resource(RefineryResource::Electronics, &data)
        .is_err());
    assert_eq!(session.economy.electronics, 2);
    assert_eq!(session.economy.credits, before);
}
