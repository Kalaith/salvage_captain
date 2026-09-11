use super::*;

#[test]
fn crew_buttons_name_the_current_assignment() {
    assert_eq!(
        crew_button_label(crate::state::CrewRole::SafetyOfficer),
        "CREW  //  SAFETY"
    );
}
