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

#[test]
fn port_crew_control_reports_expertise_progress() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = crate::state::GameSession::new(&data);

    assert_eq!(crew_expertise_status_label(&session), "NOVICE  //  XP 0/6");
    session.crew_role = crate::state::CrewRole::Navigator;
    session.career.crew_experience[crate::state::CrewRole::Navigator.index()] = 4;

    assert_eq!(
        crew_expertise_status_label(&session),
        "QUALIFIED  //  XP 4/6"
    );
}
