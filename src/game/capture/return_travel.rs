//! Return-transit fixtures for the capture harness.

use super::Game;
use crate::state::GameState;

pub(super) fn prepare(game: &mut Game, private_haul: bool) -> GameState {
    game.selected_voyage_plan = crate::engine::VoyagePlan::Cautious;
    game.session.briefing_voyage_plan = game.selected_voyage_plan;
    let _ = if private_haul {
        game.session.begin_expedition_with_plan_and_contract(
            "merchant_wreck",
            &game.data,
            false,
            game.selected_voyage_plan,
            false,
        )
    } else {
        game.session.begin_expedition_with_plan(
            "merchant_wreck",
            &game.data,
            true,
            game.selected_voyage_plan,
        )
    };
    super::recover_capture_cargo(game);
    let _ = game.session.finish_packing(&game.data);
    game.return_elapsed = 1.8;
    GameState::ReturnTravel
}
