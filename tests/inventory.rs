//! Inventory filtering and navigation after cargo is dropped.

use salvage_captain::data::GameData;
use salvage_captain::state::{CargoItem, CargoStatus, GameSession};
use salvage_captain::ui::decision_panel::navigation::ManifestPage;

#[test]
fn inventory_excludes_cargo_no_longer_aboard_and_retains_legacy_unplaced_items() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    assert_eq!(session.inventory_cargo().count(), 0);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.expedition.as_mut().unwrap().cargo = [
        CargoStatus::Packed,
        CargoStatus::Pending,
        CargoStatus::LeftBehind,
        CargoStatus::Discarded,
        CargoStatus::Lost,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, status)| CargoItem {
        object_id: index.to_string(),
        status,
        position: None,
        rotation: 0,
    })
    .collect();
    let visible: Vec<_> = session
        .inventory_cargo()
        .map(|cargo| cargo.status)
        .collect();
    assert_eq!(visible, [CargoStatus::Packed, CargoStatus::Pending]);
    session.leave_all_pending().unwrap();
    assert_eq!(session.inventory_cargo().count(), 1);
}

#[test]
fn dropping_the_last_cargo_on_a_page_keeps_remaining_inventory_and_return_available() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.cargo_bay_level = 1;
    for object_id in [
        "industrial_battery",
        "navigation_computer",
        "medical_supplies",
        "trade_crate",
    ] {
        session.expedition.as_mut().unwrap().cargo.push(CargoItem {
            object_id: object_id.to_owned(),
            status: CargoStatus::Pending,
            position: None,
            rotation: 0,
        });
        session.auto_place(object_id, &data).unwrap();
    }
    let mut page = ManifestPage::default();
    page.turn(true, session.inventory_cargo().count());
    assert_eq!(page.current(session.inventory_cargo().count()), 1);
    session
        .set_cargo_status("trade_crate", CargoStatus::Discarded)
        .unwrap();
    assert_eq!(page.current(session.inventory_cargo().count()), 0);
    assert_eq!(session.inventory_cargo().count(), 3);
    assert_eq!(session.pending_count(), 0);
    session.finish_packing(&data).unwrap();
    assert!(session.expedition.is_none());
}
