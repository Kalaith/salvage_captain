use super::*;

#[test]
fn hazard_readout_pairs_compact_label_with_response_signal() {
    assert_eq!(
        hazard_readout("structural_collapse"),
        "HAZARD  STRUCTURAL // HULL COLLAPSE"
    );
    assert_eq!(
        hazard_readout("electrical_arcs"),
        "HAZARD  ELECTRICAL // ARC FLASH"
    );
}

#[test]
fn unknown_hazard_readout_keeps_operator_label() {
    assert_eq!(hazard_readout("unknown_hazard"), "HAZARD  UNKNOWN HAZARD");
}
