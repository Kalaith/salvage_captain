use super::*;

#[test]
fn section_shift_window_is_short_and_positive() {
    assert!(SECTION_SHIFT_SECONDS > 0.0);
    assert!(SECTION_SHIFT_SECONDS < 1.0);
    assert!(SECTION_ARRIVAL_FLASH_SECONDS > 0.0);
}

#[test]
fn section_switch_prompt_waits_for_arrival_when_camera_moves() {
    assert!(section_switch_prompt("Camera moved.", true).contains("wait for ARRIVAL"));
    assert!(section_switch_prompt("Already here.", false).contains("Tap SCAN"));
}

#[test]
fn section_shift_ease_stays_inside_the_transition_window() {
    assert_eq!(section_shift_ease(-0.2), 0.0);
    assert!(section_shift_ease(0.5) > 0.4);
    assert_eq!(section_shift_ease(1.2), 1.0);
}

#[test]
fn section_arrival_waits_for_camera_and_breathing_room() {
    assert!(!section_arrival_ready(0.99, 2.0));
    assert!(!section_arrival_ready(1.0, 0.79));
    assert!(section_arrival_ready(1.0, 0.8));
}

#[test]
fn settled_prompt_names_the_scan_control() {
    assert!(SECTION_SETTLED_PROMPT.contains("Tap SCAN"));
}

#[test]
fn drone_status_names_active_pull_support() {
    assert_eq!(
        drone_status_label(true),
        "DRONE MESH ACTIVE // PULL SUPPORT ONLINE"
    );
    assert_eq!(drone_status_label(false), "");
}

#[test]
fn hazard_notice_names_the_typed_response() {
    assert_eq!(
        hazard_response_suffix(Some(crate::engine::WorkspaceHazard::StructuralCollapse)),
        " // RESPONSE HULL COLLAPSE"
    );
    assert!(hazard_response_suffix(None).is_empty());
}
