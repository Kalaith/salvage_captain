use super::*;
use crate::engine::VoyagePlan;

#[test]
fn scanned_log_rows_show_persistent_survey_count() {
    assert_eq!(
        survey_log_suffix(WorkspaceLogEvent::SectionScanned, 3),
        "  //  SURV 03"
    );
}

#[test]
fn non_scan_rows_keep_their_original_context_shape() {
    assert_eq!(survey_log_suffix(WorkspaceLogEvent::TargetRecovered, 3), "");
    assert_eq!(survey_log_suffix(WorkspaceLogEvent::SectionScanned, 0), "");
    assert_eq!(
        scan_log_suffix(
            WorkspaceLogEvent::TargetRecovered,
            WorkspaceScanProfile::Array
        ),
        ""
    );
}

#[test]
fn log_summary_counts_drone_deployments_as_field_events() {
    let entries = [WorkspaceLogEntry::new(
        1,
        WorkspaceLogEvent::DronesDeployed,
        Some("cargo_bay"),
        None,
    )];
    assert_eq!(
        log_event_count(&entries, WorkspaceLogEvent::DronesDeployed),
        1
    );
    assert_eq!(
        log_event_count(&entries, WorkspaceLogEvent::SectionScanned),
        0
    );
}

#[test]
fn log_summary_names_the_active_scan_profile() {
    assert_eq!(
        scan_log_label(WorkspaceScanProfile::Standard),
        "SCAN STANDARD"
    );
    assert_eq!(scan_log_label(WorkspaceScanProfile::Array), "SCAN ARRAY");
}

#[test]
fn log_summary_names_the_active_voyage_plan() {
    assert_eq!(
        voyage_plan_log_label(VoyagePlan::Expedited),
        "PLAN EXPEDITED"
    );
}

#[test]
fn scanned_log_rows_repeat_the_active_scan_profile() {
    assert_eq!(
        scan_log_suffix(
            WorkspaceLogEvent::SectionScanned,
            WorkspaceScanProfile::Standard
        ),
        "  //  SCAN STANDARD"
    );
    assert_eq!(
        scan_log_suffix(
            WorkspaceLogEvent::SectionScanned,
            WorkspaceScanProfile::Array
        ),
        "  //  SCAN ARRAY"
    );
}
