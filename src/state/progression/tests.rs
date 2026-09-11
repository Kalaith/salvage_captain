use super::*;

#[test]
fn starting_credits_open_only_the_early_blueprint_tier() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert!(session.module_is_unlocked("battery_module", &data));
    assert!(!session.module_is_unlocked("scanner_module", &data));
    assert_eq!(session.next_module_unlock(&data).unwrap().id, "nav_module");
}

#[test]
fn credit_milestones_unlock_blueprints_permanently() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = 1_200;

    let newly_unlocked = session.refresh_module_unlocks(&data);

    assert!(newly_unlocked.contains(&"nav_module".to_owned()));
    assert!(newly_unlocked.contains(&"hull_plating".to_owned()));
    assert!(newly_unlocked.contains(&"scanner_module".to_owned()));
    assert!(session.module_is_unlocked("scanner_module", &data));
    session.economy.credits = 0;
    session.refresh_module_unlocks(&data);
    assert!(session.module_is_unlocked("scanner_module", &data));
}

#[test]
fn locked_blueprint_rejects_purchase_without_mutating_the_ship() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let layout_before = session.ship_layout.clone();
    let credits_before = session.economy.credits;

    let error = session
        .purchase_module("scanner_module", &data)
        .unwrap_err();

    assert!(error.contains("blueprint is locked"));
    assert_eq!(session.ship_layout, layout_before);
    assert_eq!(session.economy.credits, credits_before);
}

#[test]
fn progression_milestone_activates_after_credit_threshold_and_blueprint_unlock() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = data.config.progression_credit_threshold;

    session.refresh_module_unlocks(&data);

    assert!(session.milestone_reached);
    assert!(session.unlocked_module_count(&data) > data.config.starting_modules.len());
}
