use super::*;
use crate::data::{Footprint, GameData, GridPosition};
use crate::engine::Disposition;

#[test]
fn new_game_has_a_valid_starter_layout_and_safe_economy() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    assert_eq!(session.ship_layout.occupied_cells(), 5);
    assert_eq!(session.economy.credits, data.config.starting_credits);
    assert_eq!(session.economy.fuel, data.config.starting_fuel);
}

#[test]
fn expedition_spends_fuel_and_generates_five_objects() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    assert_eq!(session.economy.fuel, 8);
    assert_eq!(session.expedition.as_ref().unwrap().cargo.len(), 5);
}

#[test]
fn save_round_trip_preserves_layout_and_resources() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let save = session.to_save(&data.config.version);
    let json = serde_json::to_value(save).unwrap();
    let restored: SaveData = serde_json::from_value(json).unwrap();
    let restored = GameSession::from_save(restored, &data).unwrap();
    assert_eq!(restored.ship_layout, session.ship_layout);
    assert_eq!(restored.economy, session.economy);
}

#[test]
fn selling_returns_credits() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let first = session.expedition.as_ref().unwrap().cargo[0]
        .object_id
        .clone();
    session.auto_place(&first, &data).unwrap();
    let remaining: Vec<String> = session
        .expedition
        .as_ref()
        .unwrap()
        .cargo
        .iter()
        .skip(1)
        .map(|item| item.object_id.clone())
        .collect();
    for item in remaining {
        session
            .set_cargo_status(&item, CargoStatus::LeftBehind)
            .unwrap();
    }
    session.finish_packing(&data).unwrap();
    session.dispose(&first, Disposition::Sell, &data).unwrap();
    assert!(session.economy.credits > data.config.starting_credits);
}

#[test]
fn invalid_saved_layout_is_rejected() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_layout.placements[0].position = crate::data::GridPosition::new(4, 4);
    let save = session.to_save(&data.config.version);
    assert!(GameSession::from_save(save, &data).is_err());
}

#[test]
fn invalid_saved_cargo_phase_is_rejected() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let object_id = session.expedition.as_ref().unwrap().cargo[0]
        .object_id
        .clone();
    session.auto_place(&object_id, &data).unwrap();
    session
        .expedition
        .as_mut()
        .unwrap()
        .cargo
        .iter_mut()
        .find(|item| item.object_id == object_id)
        .unwrap()
        .status = CargoStatus::Pending;
    let save = session.to_save(&data.config.version);
    assert!(GameSession::from_save(save, &data).is_err());
}

#[test]
fn installed_module_can_be_removed_at_port() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    assert_eq!(session.max_fuel(&data), 24);
    session.remove_module("fuel_tank", &data).unwrap();
    assert_eq!(session.max_fuel(&data), 20);
    assert!(!session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.id == "fuel_tank"));
}

#[test]
fn starter_engine_provides_one_external_clamp() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    assert_eq!(session.external_capacity(&data), 1);
    assert_eq!(session.external_cargo_count(&data, None), 0);
}

#[test]
fn external_cargo_respects_clamp_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("sealed_container", &data).unwrap();
    let error = session.auto_place("trade_crate", &data).unwrap_err();
    assert!(error.contains("No external clamp is free"));
}

#[test]
fn heavy_salvage_modules_expand_the_external_rig() {
    let data = GameData::load().unwrap();
    assert_eq!(
        data.modules
            .get("reactor_module")
            .unwrap()
            .external_capacity,
        2
    );
    assert_eq!(
        data.modules.get("shield_module").unwrap().external_capacity,
        1
    );
}

#[test]
fn yard_can_buy_a_scanner_into_open_ship_space() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let before = session.economy.credits;
    let message = session.purchase_module("scanner_module", &data).unwrap();
    assert_eq!(session.economy.credits, before - 360);
    assert!(message.contains("Clamp capacity is now 2"));
    assert!(session.has_capability("scanner_array", &data));
    assert!(session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == "scanner_module"));
}

#[test]
fn yard_rejects_unaffordable_purchase_without_mutating_ship() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = 0;
    let layout_before = session.ship_layout.clone();
    let unlocked_before = session.unlocked_modules.clone();

    let error = session
        .purchase_module("scanner_module", &data)
        .unwrap_err();

    assert!(error.contains("requires 360 credits"));
    assert_eq!(session.ship_layout, layout_before);
    assert_eq!(session.unlocked_modules, unlocked_before);
    assert_eq!(session.economy.credits, 0);
}

#[test]
fn yard_rejects_purchase_when_the_grid_has_no_fit() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_layout.placements.clear();
    session
        .ship_layout
        .place(
            "test_blocker",
            Footprint {
                width: 5,
                height: 5,
            },
            GridPosition::new(0, 0),
            0,
            true,
        )
        .unwrap();
    let credits_before = session.economy.credits;

    let error = session
        .purchase_module("scanner_module", &data)
        .unwrap_err();

    assert!(error.contains("no open fit"));
    assert_eq!(session.economy.credits, credits_before);
    assert_eq!(session.ship_layout.placements.len(), 1);
}

#[test]
fn damaged_engine_goes_offline_until_repaired() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());

    assert_eq!(session.module_stats(&data).power, 0);
    assert_eq!(session.tractor_capacity_tons(&data), 0.0);
    assert!(!session.has_capability("basic_tractor", &data));

    let message = session.repair(&data).unwrap();

    assert!(session.damaged_modules.is_empty());
    assert!(message.contains("Restored: Engine Core"));
    assert_eq!(session.module_stats(&data).power, 2);
    assert!(session.has_capability("basic_tractor", &data));
}

#[test]
fn damaged_fuel_tank_clamps_fuel_to_new_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());
    session.economy.fuel = session.max_fuel(&data);

    session.apply_workspace_damage(&data);

    assert!(session.damaged_modules.contains(&"fuel_tank".to_owned()));
    assert_eq!(session.max_fuel(&data), 20);
    assert_eq!(session.economy.fuel, 20);
}

#[test]
fn save_rejects_duplicate_damaged_modules() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules = vec!["engine_core".to_owned(), "engine_core".to_owned()];

    let error = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

    assert!(error.contains("invalid damaged module 'engine_core'"));
}
