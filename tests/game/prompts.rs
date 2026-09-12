//! Player-facing transition prompt rules.

use super::*;
use crate::state::GameState;

#[test]
fn site_selection_prompt_names_visible_touch_controls() {
    let prompt = state_prompt(GameState::SiteSelection);

    assert!(prompt.contains("Plan"));
    assert!(prompt.contains("Depart"));
    assert!(prompt.contains("Insurance"));
}

#[test]
fn workspace_prompt_names_the_power_recovery_control() {
    let prompt = state_prompt(GameState::SalvageWorkspace);

    assert!(prompt.contains("SCAN"));
    assert!(prompt.contains("POWER CYCLE"));
}
