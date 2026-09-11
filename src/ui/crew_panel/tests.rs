use super::*;

#[test]
fn crew_buttons_name_the_current_assignment() {
    assert_eq!(
        crew_button_label(crate::state::CrewRole::SafetyOfficer),
        "CREW  //  SAFETY"
    );
    assert_eq!(rest_button_label(100), "RESTED");
    assert_eq!(rest_button_label(79), "REST  //  79%");
}
