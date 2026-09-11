use super::*;
use crate::state::GameState;

#[test]
fn site_selection_prompt_names_visible_touch_controls() {
    let prompt = state_prompt(GameState::SiteSelection);

    assert!(prompt.contains("PLAN"));
    assert!(prompt.contains("DEPART"));
    assert!(prompt.contains("COVER"));
}
