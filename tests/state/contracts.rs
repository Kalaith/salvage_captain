//! Contract objective lifecycle and payout rules.

use super::*;

#[test]
fn contract_objective_status_tracks_the_run_from_open_to_complete() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    let open = session
        .contract_objective_status("merchant_wreck", &data)
        .unwrap();
    assert_eq!(open.target_id, "industrial_battery");
    assert_eq!(open.state, ContractObjectiveState::Open);

    session.begin_expedition("merchant_wreck", &data).unwrap();
    assert_eq!(
        session
            .contract_objective_status("merchant_wreck", &data)
            .unwrap()
            .state,
        ContractObjectiveState::Open
    );
    session.scan_workspace(&data).unwrap();
    super::begin_test_transfer(&mut session, "industrial_battery", &data);
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    assert_eq!(
        session
            .contract_objective_status("merchant_wreck", &data)
            .unwrap()
            .state,
        ContractObjectiveState::Recovered
    );

    let cargo = vec![CargoItem {
        object_id: "industrial_battery".to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];
    session.complete_site_contract("merchant_wreck", &cargo, &data);
    assert_eq!(
        session
            .contract_objective_status("merchant_wreck", &data)
            .unwrap()
            .state,
        ContractObjectiveState::Complete
    );
}

#[test]
fn contract_objective_status_marks_a_removed_target_failed() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .removed_targets
        .push("industrial_battery".to_owned());

    assert_eq!(
        session
            .contract_objective_status("merchant_wreck", &data)
            .unwrap()
            .state,
        ContractObjectiveState::Failed
    );
}

#[test]
fn packed_contract_target_pays_once() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let site = data.sites.get("merchant_wreck").unwrap();
    let target_id = site.contract_target.as_ref().unwrap().clone();
    let before = session.economy.credits;
    let cargo = vec![CargoItem {
        object_id: target_id,
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];

    session
        .complete_site_contract("merchant_wreck", &cargo, &data)
        .unwrap();
    let second = session.complete_site_contract("merchant_wreck", &cargo, &data);

    assert_eq!(session.economy.credits, before + 180);
    assert_eq!(session.career.contract_income, 180);
    assert!(second.is_none());
}

#[test]
fn consecutive_contracts_add_a_streak_bonus_and_track_the_best_run() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let first_cargo = vec![CargoItem {
        object_id: "industrial_battery".to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];
    let second_cargo = vec![CargoItem {
        object_id: "shield_generator".to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];

    session
        .complete_site_contract("merchant_wreck", &first_cargo, &data)
        .unwrap();
    let before_second = session.economy.credits;
    session
        .complete_site_contract("military_wreck", &second_cargo, &data)
        .unwrap();

    assert_eq!(session.contract_streak(), 2);
    assert_eq!(session.career.best_contract_streak, 2);
    assert_eq!(session.economy.credits, before_second + 345);
}

#[test]
fn contract_bonus_can_complete_the_credit_milestone() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = data.config.progression_credit_threshold - 180;
    session.unlocked_modules.push("nav_module".to_owned());
    let cargo = vec![CargoItem {
        object_id: "industrial_battery".to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];

    session.complete_site_contract("merchant_wreck", &cargo, &data);

    assert!(session.milestone_reached);
}

#[test]
fn lost_contract_target_is_terminal_and_unpaid() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let target_id = data
        .sites
        .get("merchant_wreck")
        .unwrap()
        .contract_target
        .as_ref()
        .unwrap()
        .clone();
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .removed_targets
        .push(target_id.clone());

    session
        .complete_site_contract("merchant_wreck", &[], &data)
        .unwrap();

    assert!(
        session
            .site_progress
            .get("merchant_wreck")
            .unwrap()
            .contract_failed
    );
    assert_eq!(session.economy.credits, data.config.starting_credits);
    assert_eq!(session.contract_streak(), 0);
}

#[test]
fn private_haul_does_not_fail_a_lost_client_target() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.career.contract_streak = 2;
    session.career.best_contract_streak = 2;
    session.reputation = 3;
    session
        .begin_expedition_with_plan_and_contract(
            "merchant_wreck",
            &data,
            false,
            crate::engine::VoyagePlan::Standard,
            false,
        )
        .unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .lose_workspace_target("industrial_battery", &data)
        .unwrap();
    session.leave_all_pending().unwrap();

    session.finish_packing(&data).unwrap();
    let progress = session.site_progress.get("merchant_wreck").unwrap();

    assert!(!progress.contract_completed);
    assert!(!progress.contract_failed);
    assert_eq!(session.reputation, 3);
    assert_eq!(session.contract_streak(), 2);
    assert!(!session.last_voyage().unwrap().contract_accepted);
}

#[test]
fn recovered_but_abandoned_contract_target_fails_on_return() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    super::begin_test_transfer(&mut session, "industrial_battery", &data);
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    session
        .set_cargo_status("industrial_battery", CargoStatus::LeftBehind)
        .unwrap();
    session.leave_all_pending().unwrap();

    session.finish_packing(&data).unwrap();

    assert!(
        session
            .site_progress
            .get("merchant_wreck")
            .unwrap()
            .contract_failed
    );
    assert!(session.returned.is_empty());
}
