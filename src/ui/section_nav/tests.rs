use super::*;

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
