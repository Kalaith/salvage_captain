use super::*;
use crate::state::WorkspaceLogEvent;

#[test]
fn section_lock_count_only_marks_stabilized_targets_in_that_frame() {
    let candidates = vec![
        "navigation_computer".to_owned(),
        "industrial_battery".to_owned(),
    ];
    let stabilized = vec![
        "navigation_computer".to_owned(),
        "engine_assembly".to_owned(),
    ];

    assert_eq!(stabilized_target_count(&candidates, Some(&stabilized)), 1);
    assert_eq!(stabilized_target_count(&candidates, None), 0);
}

#[test]
fn section_log_count_only_marks_events_in_that_frame() {
    let entries = vec![
        WorkspaceLogEntry::new(1, WorkspaceLogEvent::Departed, Some("cargo_bay"), None),
        WorkspaceLogEntry::new(
            2,
            WorkspaceLogEvent::SectionScanned,
            Some("cargo_bay"),
            None,
        ),
        WorkspaceLogEntry::new(
            3,
            WorkspaceLogEvent::EnteredSection,
            Some("engineering_access"),
            None,
        ),
    ];

    assert_eq!(section_log_count(&entries, "cargo_bay"), 2);
    assert_eq!(section_log_count(&entries, "engineering_access"), 1);
    assert_eq!(section_log_count(&entries, "reactor_spine"), 0);
}
