use super::*;

#[test]
fn scanned_hazards_use_typed_response_initials() {
    assert_eq!(hazard_marker("reactor_instability"), "T");
    assert_eq!(hazard_marker("electrical_arcs"), "A");
    assert_eq!(hazard_marker("automated_defenses"), "D");
    assert_eq!(hazard_marker("unexploded_ammunition"), "O");
    assert_eq!(hazard_marker("magnetic_interference"), "M");
    assert_eq!(hazard_marker("structural_collapse"), "H");
}

#[test]
fn unknown_scanned_hazards_keep_the_generic_warning_marker() {
    assert_eq!(hazard_marker("unmapped"), "!");
}
