use super::*;
use crate::data::GameData;

#[test]
fn route_familiarity_grows_from_persistent_visits_and_caps() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    assert_eq!(session.route_familiarity("merchant_wreck"), 0);
    assert_eq!(session.route_familiarity_label("merchant_wreck"), "NEW");
    assert_eq!(
        session.route_familiarity_danger_reduction("merchant_wreck"),
        0
    );

    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .visits = 9;

    assert_eq!(session.route_familiarity("merchant_wreck"), 3);
    assert_eq!(
        session.route_familiarity_label("merchant_wreck"),
        "FAMILIAR"
    );
    assert_eq!(
        session.route_familiarity_danger_reduction("merchant_wreck"),
        12
    );
    assert_eq!(
        session.route_familiarity_readout("merchant_wreck"),
        "ROUTE FAMILIAR // DANGER -12"
    );
}

#[test]
fn unknown_routes_keep_a_first_pass_readout() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        session.route_familiarity_readout("merchant_wreck"),
        "ROUTE NEW // FIRST PASS"
    );
}
