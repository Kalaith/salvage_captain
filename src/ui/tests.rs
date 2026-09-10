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
