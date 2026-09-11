use super::*;

#[test]
fn scanner_capability_selects_the_deeper_array_profile() {
    assert_eq!(
        WorkspaceScanProfile::from_capability(false),
        WorkspaceScanProfile::Standard
    );
    assert_eq!(
        WorkspaceScanProfile::from_capability(true),
        WorkspaceScanProfile::Array
    );
    assert_eq!(
        WorkspaceScanProfile::Array.result_label(),
        "ARRAY SCAN // DEEP RESOLVE"
    );
}

#[test]
fn array_profile_adds_a_second_scan_pulse() {
    assert_eq!(WorkspaceScanProfile::Standard.pulse_count(), 1);
    assert_eq!(WorkspaceScanProfile::Array.pulse_count(), 2);
}
