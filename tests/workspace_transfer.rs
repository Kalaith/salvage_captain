//! Placement and recovery are one transaction with no unbounded salvage queue.

use salvage_captain::data::{GameData, GridPosition};
use salvage_captain::state::{CargoItem, CargoStatus, GameSession};

fn scanned() -> (GameData, GameSession) {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    (data, session)
}

#[test]
fn recovery_requires_a_destination_and_only_completed_pulls_fill_it() {
    let (data, mut session) = scanned();
    let initial = session.ship_layout.clone();
    assert!(session.expedition.as_ref().unwrap().cargo.is_empty());
    assert!(session
        .recover_workspace_target("industrial_battery", &data)
        .is_err());
    session
        .begin_workspace_transfer("industrial_battery", GridPosition::new(3, 2), 0, &data)
        .unwrap();
    assert_eq!(session.ship_layout, initial);
    assert!(!session.target_is_removed("industrial_battery"));
    assert!(session.expedition.as_ref().unwrap().cargo.is_empty());
    assert!(session
        .begin_workspace_transfer("navigation_computer", GridPosition::new(5, 2), 0, &data)
        .is_err());
    assert!(session.finish_packing(&data).is_err());
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    let item = &session.expedition.as_ref().unwrap().cargo[0];
    assert_eq!(item.status, CargoStatus::Packed);
    assert_eq!(item.position, Some(GridPosition::new(3, 2)));
    assert!(session.target_is_removed("industrial_battery"));
    assert_eq!(session.pending_count(), 0);
    assert!(session.workspace_transfer().is_none());
    assert!(session
        .recover_workspace_target("industrial_battery", &data)
        .is_err());
}

#[test]
fn invalid_footprints_leave_power_cargo_and_wreck_untouched() {
    let (mut data, session) = scanned();
    let mut target = data.salvage_objects.remove("industrial_battery").unwrap();
    target.rotatable = false;
    data.salvage_objects
        .insert("industrial_battery".to_owned(), target);
    for (position, rotation) in [
        (GridPosition::new(0, 0), 0),
        (GridPosition::new(-1, 2), 0),
        (GridPosition::new(99, 99), 0),
        (GridPosition::new(3, 2), 1),
    ] {
        let mut attempt = session.clone();
        assert!(attempt
            .begin_workspace_transfer("industrial_battery", position, rotation, &data)
            .is_err());
        assert_eq!(attempt.workspace_energy(), session.workspace_energy());
        assert_eq!(attempt.ship_layout, session.ship_layout);
        assert!(!attempt.target_is_removed("industrial_battery"));
        assert!(attempt.workspace_transfer().is_none());
        assert!(attempt.expedition.as_ref().unwrap().cargo.is_empty());
    }
}

#[test]
fn full_berths_and_clamps_reject_pulls_even_with_free_grid_cells() {
    let (mut data, session) = scanned();
    for external in [false, true] {
        let mut attempt = session.clone();
        let mut target = data.salvage_objects.remove("industrial_battery").unwrap();
        target.transfer_mode = if external {
            "external_clamp"
        } else {
            "internal_cargo"
        }
        .to_owned();
        data.salvage_objects
            .insert("industrial_battery".to_owned(), target);
        let (count, object_id) = if external {
            (attempt.external_capacity(&data), "titanium_plating")
        } else {
            (attempt.internal_cargo_capacity(), "navigation_computer")
        };
        attempt.expedition.as_mut().unwrap().cargo = (0..count)
            .map(|_| CargoItem {
                object_id: object_id.to_owned(),
                status: CargoStatus::Packed,
                position: Some(GridPosition::new(1, 1)),
                rotation: 0,
            })
            .collect();
        assert!(attempt
            .begin_workspace_transfer("industrial_battery", GridPosition::new(3, 2), 0, &data)
            .is_err());
        assert_eq!(attempt.workspace_energy(), session.workspace_energy());
        assert!(attempt.workspace_transfer().is_none());
    }
}

#[test]
fn cancellation_and_target_loss_release_destination_without_adding_cargo() {
    let (data, session) = scanned();
    for lost in [false, true] {
        let mut attempt = session.clone();
        attempt
            .begin_workspace_transfer("industrial_battery", GridPosition::new(3, 2), 0, &data)
            .unwrap();
        let spent = attempt.workspace_energy();
        if lost {
            attempt
                .lose_workspace_target("industrial_battery", &data)
                .unwrap();
        } else {
            attempt.cancel_workspace_transfer();
        }
        assert!(attempt.workspace_transfer().is_none());
        assert_eq!(attempt.ship_layout, session.ship_layout);
        assert_eq!(attempt.workspace_energy(), spent);
        assert!(attempt.expedition.as_ref().unwrap().cargo.is_empty());
        assert_eq!(attempt.target_is_removed("industrial_battery"), lost);
        if !lost {
            attempt
                .begin_workspace_transfer("industrial_battery", GridPosition::new(3, 2), 0, &data)
                .unwrap();
        }
    }
}

#[test]
fn rotated_recovery_uses_last_power_once_and_survives_save_and_return() {
    let mut data = GameData::load().unwrap();
    data.config.risk.safe_danger_threshold = 100;
    let mut session = GameSession::new(&data);
    session.begin_expedition("military_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    let target = data.salvage_objects.get("titanium_plating").unwrap();
    assert!(target.rotatable);
    session.expedition.as_mut().unwrap().workspace_energy = target.energy_cost;
    session
        .begin_workspace_transfer("titanium_plating", GridPosition::new(3, 2), 1, &data)
        .unwrap();
    assert_eq!(session.workspace_energy().unwrap().0, 0);
    session
        .recover_workspace_target("titanium_plating", &data)
        .unwrap();
    assert_eq!(session.workspace_energy().unwrap().0, 0);
    let mut restored =
        GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();
    let item = &restored.expedition.as_ref().unwrap().cargo[0];
    assert_eq!(item.rotation, 1);
    assert_eq!(item.position, Some(GridPosition::new(3, 2)));
    restored.finish_packing(&data).unwrap();
    assert_eq!(restored.returned[0].position, GridPosition::new(3, 2));
    assert_eq!(restored.returned[0].rotation, 1);
}
