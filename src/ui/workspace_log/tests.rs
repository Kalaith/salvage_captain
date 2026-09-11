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
fn log_summary_counts_field_power_resets_as_field_events() {
    let entries = [WorkspaceLogEntry::new(
        1,
        WorkspaceLogEvent::PowerCycled,
        Some("cargo_bay"),
        None,
    )];

    assert_eq!(log_event_count(&entries, WorkspaceLogEvent::PowerCycled), 1);
}

#[test]
fn log_summary_counts_field_power_cells_as_field_events() {
    let entries = [WorkspaceLogEntry::new(
        1,
        WorkspaceLogEvent::FieldPowerCellUsed,
        Some("cargo_bay"),
        None,
    )];

    assert_eq!(
        log_event_count(&entries, WorkspaceLogEvent::FieldPowerCellUsed),
        1
    );
    assert_eq!(
        event_context_suffix(
            WorkspaceLogEvent::FieldPowerCellUsed,
            crate::state::DroneDirective::PullSupport,
        ),
        "  //  CELL -1  //  POWER +4"
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
fn log_summary_names_the_assigned_crew() {
    assert_eq!(
        crew_log_label(crate::state::CrewRole::SafetyOfficer, 79),
        "CREW SAFETY // READY 79%"
    );
}

#[test]
fn power_cycle_log_context_names_its_fuel_and_power_delta() {
    assert_eq!(
        event_context_suffix(
            WorkspaceLogEvent::PowerCycled,
            crate::state::DroneDirective::PullSupport,
        ),
        "  //  FUEL -1  //  POWER +4"
    );
    assert!(event_context_suffix(
        WorkspaceLogEvent::SectionScanned,
        crate::state::DroneDirective::PullSupport,
    )
    .is_empty());

    assert_eq!(
        event_context_suffix(
            WorkspaceLogEvent::DroneDirectiveChanged,
            crate::state::DroneDirective::Survey,
        ),
        "  //  ORDER SURVEY"
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
