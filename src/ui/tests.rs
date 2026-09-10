use super::*;

#[test]
fn screen_titles_are_operational_not_game_branding() {
    let titles = [
        screen_title(GameState::Port, GameState::Port),
        screen_title(GameState::SiteSelection, GameState::Port),
        screen_title(GameState::Travel, GameState::Port),
        screen_title(GameState::SalvageWorkspace, GameState::Port),
        screen_title(GameState::SalvagePacking, GameState::Port),
        screen_title(GameState::Results, GameState::Port),
    ];
    assert!(titles
        .iter()
        .all(|title| !title.eq_ignore_ascii_case("SALVAGE CAPTAIN")));
}

#[test]
fn labels_clip_predictably() {
    assert_eq!(short_label("industrial_battery"), "INDUSTRIAL ");
    assert_eq!(clipped("cargo", 8), "cargo");
    assert_eq!(clipped("navigation core", 10), "navigation...");
}

#[test]
fn hazard_tags_read_like_operator_labels() {
    assert_eq!(hazard_label("reactor_instability"), "REACTOR INSTABILITY");
}

#[test]
fn contract_statuses_have_one_player_facing_vocabulary() {
    assert_eq!(contract_status_label(false, false), "RECOVER");
    assert_eq!(contract_status_label(true, false), "COMPLETE");
    assert_eq!(contract_status_label(false, true), "FAILED");
    assert_eq!(contract_status_label(true, true), "COMPLETE");
}

#[test]
fn power_badge_warns_at_a_third_reserve() {
    let normal = power_badge_color(Some((8, 12)));
    let low = power_badge_color(Some((4, 12)));

    assert!(low.r > normal.r);
    assert!(low.g < normal.g);
}
