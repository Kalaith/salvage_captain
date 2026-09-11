//! Game-layer handling for planning and operating-policy controls.

use super::Game;
use crate::state::GameState;

pub(super) fn cycle_plan(game: &mut Game) {
    if game.state != GameState::SiteSelection {
        return;
    }
    game.selected_voyage_plan = game.selected_voyage_plan.next();
    game.session.briefing_voyage_plan = game.selected_voyage_plan;
    game.note(format!(
        "Operating plan: {}. {}.",
        game.selected_voyage_plan.label(),
        game.selected_voyage_plan.description()
    ));
}

pub(super) fn cycle_crew(game: &mut Game) {
    if game.state != GameState::SiteSelection && game.state != GameState::Port {
        return;
    }
    match game.session.cycle_crew() {
        Ok(message) => game.note(message),
        Err(error) => game.note(error),
    }
}

pub(super) fn cycle_return_policy(game: &mut Game) {
    if game.state != GameState::SalvagePacking {
        return;
    }
    match game.session.cycle_return_policy() {
        Ok(message) => game.note(message),
        Err(error) => game.note(error),
    }
}
