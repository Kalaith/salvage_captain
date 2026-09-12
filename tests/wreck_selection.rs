//! Regression coverage for choosing a wreck without starting an expedition.

use salvage_captain::data::GameData;
use salvage_captain::engine::VoyagePlan;
use salvage_captain::state::GameSession;
use salvage_captain::ui::site_cards::{card_rect, SelectionAction, WreckSelection};
use salvage_captain::ui::UiAction;

#[test]
fn selection_changes_the_departure_destination() {
    let data = GameData::load().unwrap();
    let mut choice = WreckSelection::default();
    for site in data.ordered_sites() {
        choice.apply(&SelectionAction::Select(site.id.clone()), &data);
        assert_eq!(
            choice.departure_action(&data),
            Some(UiAction::Depart(site.id.clone()))
        );
    }
}

#[test]
fn private_haul_clears_coverage_and_cannot_enable_it() {
    let data = GameData::load().unwrap();
    let mut choice = WreckSelection::default();
    choice.apply(&SelectionAction::ToggleInsurance, &data);
    assert!(matches!(
        choice.departure_action(&data),
        Some(UiAction::DepartInsured(_))
    ));
    choice.apply(&SelectionAction::TogglePrivate, &data);
    choice.apply(&SelectionAction::ToggleInsurance, &data);
    assert!(!choice.insured);
    assert!(matches!(
        choice.departure_action(&data),
        Some(UiAction::DepartPrivate(_))
    ));
    choice.apply(&SelectionAction::TogglePrivate, &data);
    assert!(matches!(
        choice.departure_action(&data),
        Some(UiAction::Depart(_))
    ));
}

#[test]
fn invalid_selection_is_ignored_and_details_preserve_preparation() {
    let data = GameData::load().unwrap();
    let mut choice = WreckSelection::default();
    choice.apply(&SelectionAction::Select("military_wreck".to_owned()), &data);
    choice.apply(&SelectionAction::ToggleInsurance, &data);
    let departure = choice.departure_action(&data);
    choice.apply(&SelectionAction::Select("missing".to_owned()), &data);
    choice.apply(&SelectionAction::ToggleDetails, &data);
    assert!(choice.details_open);
    assert_eq!(choice.departure_action(&data), departure);
    choice.apply(
        &SelectionAction::Select("research_vessel".to_owned()),
        &data,
    );
    assert!(!choice.details_open);
    assert!(choice.insured);
}

#[test]
fn selected_insured_departure_preserves_authoritative_affordability_checks() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let mut choice = WreckSelection::default();
    choice.apply(
        &SelectionAction::Select("research_vessel".to_owned()),
        &data,
    );
    choice.apply(&SelectionAction::ToggleInsurance, &data);
    let Some(UiAction::DepartInsured(id)) = choice.departure_action(&data) else {
        panic!("insured action expected")
    };
    session.economy.fuel = 0;
    assert!(session
        .begin_expedition_with_plan_and_contract(&id, &data, true, VoyagePlan::Standard, true)
        .is_err());
    session.economy.fuel = 100;
    session.economy.credits = 0;
    assert!(session
        .begin_expedition_with_plan_and_contract(&id, &data, true, VoyagePlan::Standard, true)
        .is_err());
    session.economy.credits = 1000;
    assert!(session
        .begin_expedition_with_plan_and_contract(&id, &data, true, VoyagePlan::Standard, true)
        .is_ok());
    assert_eq!(session.selected_site.as_deref(), Some(id.as_str()));
}

#[test]
fn every_authored_card_has_a_separate_touch_area_above_the_inspector() {
    let data = GameData::load().unwrap();
    let sites = data.ordered_sites();
    for i in 0..sites.len() {
        let rect = card_rect(i);
        assert!(rect.x >= 24.0 && rect.right() <= 1256.0);
        assert!(rect.y >= 84.0 && rect.bottom() < 416.0);
        assert!(rect.w * 0.75 >= 44.0 && rect.h * 0.75 >= 44.0);
        for j in 0..i {
            assert!(!rect.overlaps(&card_rect(j)));
        }
    }
}
