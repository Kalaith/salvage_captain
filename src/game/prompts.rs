//! Short state prompts shared by the coordinator's transition messages.

use crate::state::GameState;

pub(super) fn state_prompt(state: GameState) -> &'static str {
    match state {
        GameState::MainMenu => "Choose an operation.",
        GameState::Port => "Shipyard online. Select equipment or browse a wreck.",
        GameState::SiteSelection => "Tap PLAN to cycle the route, then tap DEPART or COVER.",
        GameState::Travel => "Tap ARRIVE to enter the wreck workspace.",
        GameState::SalvageWorkspace => "Tap SCAN, then select a bracketed target.",
        GameState::SalvagePacking => "Place or leave every recovered object.",
        GameState::ReturnTravel => "Tap DOCK NOW to enter the yard debrief.",
        GameState::Results => "Choose SELL, INSTALL, or BREAK DOWN.",
        GameState::Pause => "Tap RESUME to continue.",
    }
}

#[cfg(test)]
mod tests;
