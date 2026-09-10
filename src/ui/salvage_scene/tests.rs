use super::*;

#[test]
fn section_shift_window_is_short_and_positive() {
    assert!(SECTION_SHIFT_SECONDS > 0.0);
    assert!(SECTION_SHIFT_SECONDS < 1.0);
}

#[test]
fn section_switch_prompt_waits_for_arrival_when_camera_moves() {
    assert!(section_switch_prompt("Camera moved.", true).contains("wait for ARRIVAL"));
    assert!(section_switch_prompt("Already here.", false).contains("Tap SCAN"));
}
