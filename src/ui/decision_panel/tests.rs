use super::*;

#[test]
fn debrief_run_label_keeps_survey_memory_visible() {
    let label = debrief_run_label(
        2,
        "MERCHANT WRECK",
        1,
        0,
        280,
        6,
        3,
        WorkspaceScanProfile::Array,
        5,
        9,
    );
    assert!(label.contains("MERCHANT WRECK  //  SCAN ARRAY"));
    assert!(label.contains("SCAN ARRAY  //  BP 05/09"));
    assert!(label.ends_with("FIELD LOG 06  //  SURV 03"));
}
