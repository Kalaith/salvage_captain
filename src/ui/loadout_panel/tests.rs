use super::*;

#[test]
fn slot_title_reports_empty_and_stored_states() {
    assert_eq!(slot_title(0, None), "SLOT 1 // EMPTY");
    assert_eq!(
        slot_title(
            2,
            Some(&LoadoutPreset {
                placements: Vec::new()
            })
        ),
        "SLOT 3 // STORED"
    );
}
