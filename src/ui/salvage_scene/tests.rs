use super::*;

#[test]
fn section_shift_window_is_short_and_positive() {
    assert!(SECTION_SHIFT_SECONDS > 0.0);
    assert!(SECTION_SHIFT_SECONDS < 1.0);
}
