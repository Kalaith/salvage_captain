use super::*;

#[test]
fn mission_briefing_names_the_next_ship_blueprint() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let label = blueprint_progress_label(&session, &data);

    assert!(label.contains("SHIP BLUEPRINTS 3/9"));
    assert!(label.contains("NEXT NAV MODULE @ ¢900"));
}
