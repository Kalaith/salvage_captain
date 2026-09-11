use super::*;

#[test]
fn hazard_readout_pairs_compact_label_with_response_signal() {
    assert_eq!(
        hazard_readout("structural_collapse", false),
        "HAZARD  STRUCTURAL // HULL COLLAPSE"
    );
    assert_eq!(
        hazard_readout("electrical_arcs", false),
        "HAZARD  ELECTRICAL // ARC FLASH"
    );
    assert_eq!(
        hazard_readout("structural_collapse", true),
        "HAZARD  STRUCTURAL // STABILIZED"
    );
}

#[test]
fn unknown_hazard_readout_keeps_operator_label() {
    assert_eq!(
        hazard_readout("unknown_hazard", false),
        "HAZARD  UNKNOWN HAZARD"
    );
}

#[test]
fn stabilization_command_names_its_power_cost() {
    assert_eq!(STABILIZE_COMMAND_LABEL, "STABILIZE  -2P");
}
