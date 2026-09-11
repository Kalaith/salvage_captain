use super::*;
use crate::data::GameData;
use crate::state::CargoItem;

#[test]
fn new_ships_have_three_internal_cargo_berths() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(session.cargo_bay_level(), 0);
    assert_eq!(session.internal_cargo_capacity(), 3);
    assert_eq!(session.internal_cargo_count(&data, None), 0);
    assert_eq!(session.cargo_bay_upgrade_label(), "UPGRADE ¢420");
}

#[test]
fn cargo_bay_upgrades_expand_berths_and_charge_the_yard() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let starting_credits = session.economy.credits;

    let first = session.purchase_cargo_bay_upgrade().unwrap();
    assert!(first.contains("Expanded the cargo bay to 5 internal berths"));
    assert_eq!(session.cargo_bay_level(), 1);
    assert_eq!(session.internal_cargo_capacity(), 5);
    assert_eq!(session.economy.credits, starting_credits - 420);

    session.economy.credits = 1_000;
    session.purchase_cargo_bay_upgrade().unwrap();
    assert_eq!(session.cargo_bay_level(), 2);
    assert_eq!(session.internal_cargo_capacity(), 7);
    assert_eq!(session.cargo_bay_upgrade_label(), "HOLD MAX");
    assert!(session.purchase_cargo_bay_upgrade().is_err());
}

#[test]
fn cargo_bay_level_survives_a_save_and_legacy_saves_start_at_level_zero() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.purchase_cargo_bay_upgrade().unwrap();

    let restored = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();
    assert_eq!(restored.cargo_bay_level(), 1);
    assert_eq!(restored.internal_cargo_capacity(), 5);

    let mut legacy = serde_json::to_value(GameSession::new(&data).to_save("2.37.0")).unwrap();
    legacy["session"]
        .as_object_mut()
        .unwrap()
        .remove("cargo_bay_level");
    let restored_legacy: crate::state::SaveData = serde_json::from_value(legacy).unwrap();
    let restored_legacy = GameSession::from_save(restored_legacy, &data).unwrap();
    assert_eq!(restored_legacy.cargo_bay_level(), 0);
}

#[test]
fn internal_cargo_berths_block_a_fourth_packed_item_until_upgrade() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.expedition.as_mut().unwrap().cargo = [
        "industrial_battery",
        "navigation_computer",
        "medical_supplies",
    ]
    .into_iter()
    .map(|object_id| CargoItem {
        object_id: object_id.to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    })
    .collect();

    let error = session
        .place_cargo(
            "quantum_lens",
            crate::data::GridPosition::new(2, 2),
            0,
            &data,
        )
        .unwrap_err();
    assert!(error.contains("No internal cargo berth is free (3/3)"));

    session.expedition = None;
    session.purchase_cargo_bay_upgrade().unwrap();
    assert_eq!(session.internal_cargo_capacity(), 5);
}
