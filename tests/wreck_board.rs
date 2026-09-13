//! Wreck-board navigation and renewable end-of-content regression coverage.

use salvage_captain::data::discovery::LeadKind;
use salvage_captain::data::GameData;
use salvage_captain::engine::VoyagePlan;
use salvage_captain::state::GameSession;
use salvage_captain::ui::site_cards::{BoardAction, WreckSelection, BOARD_PAGE_SIZE};
use salvage_captain::ui::voyage_archive::ArchiveFilter;

fn setup() -> (GameData, GameSession) {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    (data, session)
}

#[test]
fn paging_keeps_selection_visible_and_preserves_departure_preferences() {
    let (mut data, mut session) = setup();
    for _ in 0..3 {
        session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    }
    let mut choice = WreckSelection {
        insured: true,
        ..Default::default()
    };
    choice.normalize_board(&session, &data);
    let first: Vec<_> = choice
        .visible_sites(&session, &data)
        .iter()
        .map(|s| s.id.clone())
        .collect();
    assert_eq!(first.len(), BOARD_PAGE_SIZE);
    assert_eq!(choice.page_count(&session, &data), 2);
    choice.apply_board(BoardAction::Next, &session, &data);
    let second = choice.visible_sites(&session, &data);
    assert_eq!(second.len(), BOARD_PAGE_SIZE);
    assert!(second.iter().all(|s| !first.contains(&s.id)));
    assert!(second
        .iter()
        .any(|s| Some(&s.id) == choice.site_id.as_ref()));
    assert!(choice.insured);
    choice.apply_board(BoardAction::Next, &session, &data);
    assert_eq!(choice.page, 1);
    choice.apply_board(BoardAction::Previous, &session, &data);
    choice.apply_board(BoardAction::Previous, &session, &data);
    assert_eq!(choice.page, 0);
}

#[test]
fn archive_shows_depleted_sites_and_authority_refuses_all_departure_modes() {
    let (data, mut session) = setup();
    let id = "merchant_wreck";
    session.site_progress.get_mut(id).unwrap().removed_targets =
        data.sites.get(id).unwrap().candidate_salvage.clone();
    let mut choice = WreckSelection::default();
    choice.normalize_board(&session, &data);
    assert_ne!(choice.site_id.as_deref(), Some(id));
    choice.apply_board(BoardAction::ToggleArchive, &session, &data);
    assert_eq!(choice.site_id.as_deref(), Some(id));
    let fuel = session.economy.fuel;
    let credits = session.economy.credits;
    for insured in [true, false] {
        for contract in [true, false] {
            assert!(session
                .begin_expedition_with_plan_and_contract(
                    id,
                    &data,
                    insured,
                    VoyagePlan::Standard,
                    contract
                )
                .is_err());
        }
    }
    assert!(!session.can_depart(id, &data));
    assert_eq!(session.economy.fuel, fuel);
    assert_eq!(session.economy.credits, credits);
}

#[test]
fn empty_archive_has_no_selected_destination_and_switches_back_cleanly() {
    let (data, session) = setup();
    let mut choice = WreckSelection::default();
    choice.apply_board(BoardAction::ToggleArchive, &session, &data);
    assert!(choice.selected_on_board(&session, &data).is_none());
    assert_eq!(choice.page_count(&session, &data), 1);
    choice.apply_board(BoardAction::ToggleArchive, &session, &data);
    assert!(choice.selected_on_board(&session, &data).is_some());
}

#[test]
fn exhausted_starter_save_can_continue_through_many_distinct_wrecks() {
    let (mut data, mut session) = setup();
    for site in data.ordered_sites() {
        session
            .site_progress
            .get_mut(&site.id)
            .unwrap()
            .removed_targets = site.candidate_salvage.clone();
    }
    let mut ids = std::collections::HashSet::new();
    for _ in 0..12 {
        let id = session.replenish_wrecks(&mut data).unwrap().unwrap();
        assert!(ids.insert(id.clone()));
        assert_eq!(session.listed_wrecks(&data, false).len(), 1);
        session.site_progress.get_mut(&id).unwrap().removed_targets =
            data.sites.get(&id).unwrap().candidate_salvage.clone();
    }
    assert_eq!(session.listed_wrecks(&data, true).len(), 15);
    let save = session.to_save(&data.config.version);
    let restored = GameSession::from_save(save, &GameData::load().unwrap()).unwrap();
    assert_eq!(restored.wrecks.len(), 12);
}

#[test]
fn journal_class_filters_include_generated_wrecks_without_cross_matching() {
    assert!(ArchiveFilter::Merchant.matches("wreck:merchant_wreck:00000001"));
    assert!(ArchiveFilter::Military.matches("wreck:military_wreck:00000002"));
    assert!(ArchiveFilter::Research.matches("wreck:research_vessel:00000003"));
    assert!(!ArchiveFilter::Merchant.matches("wreck:military_wreck:00000001"));
    assert!(ArchiveFilter::All.matches("wreck:research_vessel:00000003"));
}

#[test]
fn specialist_objectives_vary_and_research_opportunities_open_with_standing() {
    let data = GameData::load().unwrap();
    let mut objectives = std::collections::HashSet::new();
    for seed in 0..40 {
        let wreck = salvage_captain::engine::discovery::generate_wreck(
            &data.discovery.pools[1],
            seed + 1,
            seed,
            &data,
        )
        .unwrap();
        let id = wreck.site.contract_target.as_ref().unwrap();
        let target = wreck.targets.iter().find(|t| t.object.id == *id).unwrap();
        objectives.insert(target.template_id.clone());
    }
    assert!(objectives.len() >= 3);
    let mut found_research = false;
    for seed in 0..20 {
        let mut data = data.clone();
        let mut session = GameSession::new(&data);
        session.reputation = 4;
        session.discovery_seed = seed;
        let id = session
            .discover_wreck(LeadKind::Specialist, &mut data)
            .unwrap();
        found_research |= data.sites.get(&id).unwrap().visual_theme == "research";
    }
    assert!(found_research);
}
