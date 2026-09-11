use super::*;
use crate::data::GameData;
use crate::state::SaveData;

#[test]
fn loadout_slots_store_and_restore_permanent_module_geometry() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.store_loadout(0).unwrap();
    assert_eq!(session.loadout_capacity(0, &data), Some((24, 10)));
    session.remove_module("fuel_tank", &data).unwrap();

    assert!(!session.loadout_matches_current(0));
    let message = session.apply_loadout(0, &data).unwrap();

    assert!(message.contains("slot 1 applied"));
    assert!(session.loadout_matches_current(0));
    assert!(session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.id == "fuel_tank"));
}

#[test]
fn loadout_slots_reject_live_expeditions_and_empty_slots() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);

    assert!(session
        .apply_loadout(1, &data)
        .unwrap_err()
        .contains("empty"));
    session.begin_expedition("merchant_wreck", &data).unwrap();
    assert!(session.store_loadout(0).unwrap_err().contains("safe port"));
}

#[test]
fn saved_loadouts_reject_unknown_or_malformed_modules() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.store_loadout(0).unwrap();
    session.loadout_slots[0].as_mut().unwrap().placements[0].id = "ghost_module".to_owned();

    let error = validate_saved_loadouts(&session.loadout_slots, &session, &data).unwrap_err();

    assert!(error.contains("slot 1") && error.contains("unknown module"));
}

#[test]
fn loadout_slots_reject_a_ship_over_the_restored_capacity() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.store_loadout(0).unwrap();
    session.economy.fuel = session.max_fuel(&data) + 1;

    let error = session.apply_loadout(0, &data).unwrap_err();

    assert!(error.contains("caps fuel"));
}

#[test]
fn loadout_slots_survive_save_roundtrips_and_old_saves_default_empty() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.store_loadout(0).unwrap();

    let restored = GameSession::from_save(session.to_save("2.30.0"), &data).unwrap();
    assert!(restored.loadout_slot(0).is_some());

    let mut old_value = serde_json::to_value(restored.to_save("2.29.0")).unwrap();
    old_value
        .get_mut("session")
        .and_then(serde_json::Value::as_object_mut)
        .expect("saved session")
        .remove("loadout_slots");
    let old_save: SaveData = serde_json::from_value(old_value).unwrap();
    let migrated = GameSession::from_save(old_save, &data).unwrap();

    assert!(migrated.loadout_slots.iter().all(Option::is_none));
}
