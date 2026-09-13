//! Deterministic debrief fixtures for settlement, affordability, and full holds.

use super::{begin_capture_transfer, recover_capture_cargo};
use crate::game::Game;
use crate::state::GameState;

pub(super) fn prepare(game: &mut Game, scene: &str) -> GameState {
    game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
    game.session.briefing_voyage_plan = game.selected_voyage_plan;
    let private = scene == "results_private_haul";
    if !private {
        game.session
            .buy_reconnaissance("merchant_wreck", &game.data)
            .unwrap();
    }
    game.session
        .begin_expedition_with_plan_and_contract(
            "merchant_wreck",
            &game.data,
            !private,
            game.selected_voyage_plan,
            !private,
        )
        .unwrap();
    game.session.scan_workspace(&game.data).unwrap();
    match scene {
        "results_empty" => {}
        "results_open" | "results_failed" => {
            if scene == "results_failed" {
                game.session
                    .lose_workspace_target("industrial_battery", &game.data)
                    .unwrap();
            }
            begin_capture_transfer(game, "navigation_computer");
            game.session
                .recover_workspace_target("navigation_computer", &game.data)
                .unwrap();
        }
        _ => recover_capture_cargo(game),
    }
    game.session.leave_all_pending().unwrap();
    game.session.finish_packing(&game.data).unwrap();
    if scene == "results_low_funds" {
        game.session.economy.credits = 0;
    }
    if matches!(scene, "results_many" | "results_last") {
        let fixture = game.session.returned[0].clone();
        for id in [
            "shield_generator",
            "experimental_sensor",
            "titanium_plating",
            "sealed_container",
        ] {
            assert!(
                game.data.salvage_objects.contains(id),
                "capture object exists"
            );
            let mut item = fixture.clone();
            item.object_id = id.to_owned();
            game.session.returned.push(item);
        }
        if scene == "results_last" {
            game.manifest_page.turn(true, game.session.returned.len());
        }
        let value = game
            .session
            .returned
            .iter()
            .filter_map(|item| game.session.returned_market_quote(item, &game.data))
            .map(|quote| quote.sale_value)
            .sum();
        let record = game.session.voyage_log.last_mut().unwrap();
        record.recovered_count = game.session.returned.len() as u32;
        record.recovered_value = value;
    }
    if scene == "results_journal" {
        verify_journal_actions(game);
    }
    GameState::Results
}

fn verify_journal_actions(game: &mut Game) {
    use crate::ui::{voyage_archive::ArchiveAction, UiAction};

    game.state = GameState::Results;
    let cargo_before = game.session.returned.len();
    game.apply_action(UiAction::ToggleVoyageArchive);
    assert!(game.voyage_archive_open);
    game.apply_action(UiAction::Archive(ArchiveAction::Page(1)));
    assert_eq!(game.voyage_archive.page, 1);
    game.apply_action(UiAction::ToggleVoyageArchive);
    assert!(!game.voyage_archive_open);
    assert_eq!(game.session.returned.len(), cargo_before);
    assert_eq!(game.state, GameState::Results);
    game.apply_action(UiAction::ToggleVoyageArchive);
}
