use super::*;

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
}
