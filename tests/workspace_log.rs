//! Pure pagination rules for the touch-accessible field log.

use salvage_captain::ui::workspace_log::{page_count, turn_page};

#[test]
fn field_log_keeps_newest_page_first_and_clamps_navigation() {
    assert_eq!(page_count(0), 1);
    assert_eq!(page_count(6), 1);
    assert_eq!(page_count(7), 2);
    assert_eq!(turn_page(0, false, 7), 0);
    assert_eq!(turn_page(0, true, 7), 1);
    assert_eq!(turn_page(1, true, 7), 1);
    assert_eq!(turn_page(1, false, 7), 0);
}
