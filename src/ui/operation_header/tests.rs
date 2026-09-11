use super::*;

#[test]
fn log_button_label_keeps_small_counts_readable() {
    assert_eq!(log_button_label(0), "LOG 00");
    assert_eq!(log_button_label(6), "LOG 06");
    assert_eq!(log_button_label(107), "LOG 107");
}

#[test]
fn hull_badge_carries_accumulated_ship_wear() {
    assert_eq!(hull_badge_label(8, 42), "HULL 8 // W42");
}

#[test]
fn power_badge_carries_field_cell_stock() {
    assert_eq!(field_power_cell_label(0), "CELL 0");
    assert_eq!(field_power_cell_label(2), "CELL 2");
}
