//! Game-layer handling for the safe-port crew assignment control.

use super::Game;
use crate::state::GameState;

pub(super) fn cycle(game: &mut Game) {
    if game.state != GameState::SiteSelection && game.state != GameState::Port {
        return;
    }
    match game.session.cycle_crew() {
        Ok(message) => game.note(message),
        Err(error) => game.note(error),
    }
}
