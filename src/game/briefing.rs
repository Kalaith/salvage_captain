//! Game-layer handling for planning and operating-policy controls.

use super::Game;
use crate::state::GameState;
use crate::ui::UiAction;

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
    if game.state != GameState::CargoInventory {
        return;
    }
    match game.session.cycle_return_policy() {
        Ok(message) => game.note(message),
        Err(error) => game.note(error),
    }
}

pub(super) fn depart(game: &mut Game, action: UiAction) {
    let (site_id, insured, contract_accepted) = match action {
        UiAction::Depart(site_id) => (site_id, false, true),
        UiAction::DepartPrivate(site_id) => (site_id, false, false),
        UiAction::DepartInsured(site_id) => (site_id, true, true),
        _ => unreachable!("departure action matched above"),
    };
    match game.session.begin_expedition_with_plan_and_contract(
        &site_id,
        &game.data,
        insured,
        game.selected_voyage_plan,
        contract_accepted,
    ) {
        Ok(_message) => {
            game.transition(crate::state::StateTransition::ToTravel);
            if insured {
                game.note(
                    "Transit underway under coverage. Tap ARRIVE to enter the wreck workspace.",
                );
            } else if contract_accepted {
                game.note("Transit underway. Tap ARRIVE to enter the wreck workspace.");
            } else {
                game.note(
                    "Private haul underway; client contract declined. Tap ARRIVE to enter the wreck workspace.",
                );
            }
        }
        Err(error) => game.note(error),
    }
}
