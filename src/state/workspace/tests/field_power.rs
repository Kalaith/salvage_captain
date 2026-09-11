use super::*;
use crate::state::workspace_energy::{FIELD_POWER_CELL_PRICE, MAX_FIELD_POWER_CELLS};

#[test]
fn field_power_cells_are_bought_at_port_and_spent_in_the_wreck() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    assert!(session.can_buy_field_power_cell());
    session.buy_field_power_cell().unwrap();
    assert_eq!(session.field_power_cells, 1);
    assert_eq!(
        session.economy.credits,
        data.config.starting_credits - FIELD_POWER_CELL_PRICE
    );

    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.expedition.as_mut().unwrap().workspace_energy = 2;
    assert!(session.can_use_field_power_cell());

    let message = session.use_field_power_cell().unwrap();

    assert!(message.contains("+4 power"));
    assert_eq!(session.field_power_cells, 0);
    assert_eq!(session.workspace_energy(), Some((6, 12)));
    assert!(session
        .site_progress
        .get("merchant_wreck")
        .unwrap()
        .operation_log
        .iter()
        .any(|entry| entry.event == WorkspaceLogEvent::FieldPowerCellUsed));
    assert!(!session.can_use_field_power_cell());
}

#[test]
fn field_power_cell_stock_stops_at_the_rack_limit() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = FIELD_POWER_CELL_PRICE * i64::from(MAX_FIELD_POWER_CELLS);

    for _ in 0..MAX_FIELD_POWER_CELLS {
        session.buy_field_power_cell().unwrap();
    }

    assert_eq!(session.field_power_cells, MAX_FIELD_POWER_CELLS);
    assert!(!session.can_buy_field_power_cell());
    assert_eq!(
        session.buy_field_power_cell().unwrap_err(),
        "field power cell rack is full"
    );
}

#[test]
fn field_power_cell_can_be_fabricated_from_salvage_at_port() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.alloy = 1;
    session.economy.electronics = 1;
    let starting_credits = session.economy.credits;

    let message = session.fabricate_field_power_cell().unwrap();

    assert!(message.contains("from salvage"));
    assert_eq!(session.field_power_cells, 1);
    assert_eq!(session.economy.alloy, 0);
    assert_eq!(session.economy.electronics, 0);
    assert_eq!(session.economy.credits, starting_credits);
    assert_eq!(session.career.field_power_cells_fabricated, 1);
    assert!(session.can_buy_field_power_cell());
}

#[test]
fn field_power_cell_fabrication_requires_both_salvage_materials() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.alloy = 1;

    let error = session.fabricate_field_power_cell().unwrap_err();

    assert!(error.contains("Alloy") && error.contains("Electronics"));
    assert_eq!(session.field_power_cells, 0);
    assert_eq!(session.economy.alloy, 1);
    assert_eq!(session.career.field_power_cells_fabricated, 0);
}
