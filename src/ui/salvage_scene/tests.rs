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
fn power_reset_status_names_ready_and_spent_states() {
    assert_eq!(
        power_cycle::status_label(true, 0),
        Some("POWER RESET READY // FUEL 1")
    );
    assert_eq!(
        power_cycle::status_label(false, 1),
        Some("POWER RESET SPENT")
    );
    assert_eq!(power_cycle::status_label(false, 0), None);
}

#[test]
fn workspace_console_names_blueprint_progress() {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(workspace_blueprint_label(&session, &data), "BP 03/09");
}

#[test]
fn workspace_console_names_contract_standing_progress() {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(workspace_standing_label(&session), "REP 0/2");
}

#[test]
fn workspace_console_repeats_the_active_coverage_terms() {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data);
    let quote = session.insurance_quote("merchant_wreck", &data);

    assert_eq!(
        workspace_coverage_label(true, quote, 75),
        "COVER ¢65 // 75% CLAIM"
    );
    assert_eq!(workspace_coverage_label(false, quote, 75), "COVER NONE");
}

#[test]
fn hazard_notice_names_the_typed_response() {
    assert_eq!(
        hazard_response_suffix(Some(crate::engine::WorkspaceHazard::StructuralCollapse)),
        " // RESPONSE HULL COLLAPSE"
    );
    assert!(hazard_response_suffix(None).is_empty());
}
