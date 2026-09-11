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

#[test]
fn survey_memory_label_distinguishes_new_and_repeated_passes() {
    assert_eq!(survey_memory_label(None), "SURVEY NEW");
    let note = crate::state::TargetSurveyNote {
        target_id: "navigation_computer".to_owned(),
        section_id: "cargo_bay".to_owned(),
        scan_count: 2,
        hazard: Some("electrical_arcs".to_owned()),
        transfer_mode: "internal_cargo".to_owned(),
        integrity: 74,
        extraction_difficulty: 46,
        mass_tons: 4.5,
    };
    assert_eq!(survey_memory_label(Some(&note)), "SURVEY MEMORY // PASS 02");
}
