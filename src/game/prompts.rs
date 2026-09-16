//! Short state prompts shared by the coordinator's transition messages.

use crate::state::GameState;

pub fn state_prompt(state: GameState) -> &'static str {
    match state {
        GameState::MainMenu => "Choose an operation.",
        GameState::Port => "Shipyard online. Select equipment or browse a wreck.",
        GameState::SiteSelection => {
            "Select a wreck. Tap Plan or Crew to cycle assignments, choose Insurance or Private haul, then Depart."
        }
        GameState::Travel => "Tap ARRIVE to enter the wreck workspace.",
        GameState::SalvageWorkspace => {
            "Tap SCAN or POWER CYCLE, select a target, then choose its hold position before the pull. Tap INVENTORY to view cargo or RETURN WITH HAUL when done."
        }
        GameState::CargoInventory => {
            "Review secured cargo. Tap BACK TO WRECK to continue salvaging or RETURN WITH HAUL."
        }
        GameState::ReturnTravel => "Tap DOCK NOW to enter the yard debrief.",
        GameState::Results => "Choose SELL, INSTALL, or BREAK DOWN.",
        GameState::Pause => "Tap RESUME to continue.",
    }
}
