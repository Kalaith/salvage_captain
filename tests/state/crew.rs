//! Crew assignment, expertise, and training rules.

use super::*;
use crate::data::GameData;

#[test]
fn crew_roles_cycle_in_a_visible_operating_order() {
    assert_eq!(CrewRole::ALL.len(), 5);
    assert_eq!(CrewRole::Deckhand.next(), CrewRole::Navigator);
    assert_eq!(CrewRole::SafetyOfficer.next(), CrewRole::Broker);
    assert_eq!(CrewRole::Broker.next(), CrewRole::Deckhand);
}

#[test]
fn crew_assignment_changes_only_the_promised_operating_levers() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    assert_eq!(session.crew_role(), CrewRole::Deckhand);
    assert_eq!(session.crew_fuel_delta(), 0);
    assert_eq!(session.crew_adjusted_danger(45), 45);
    assert_eq!(session.crew_external_capacity(), 0);
    assert_eq!(session.crew_contract_bonus_percent(), 0);

    session.cycle_crew().unwrap();
    assert_eq!(session.crew_role(), CrewRole::Navigator);
    assert_eq!(session.crew_fuel_delta(), -1);
    assert_eq!(session.crew_adjusted_danger(45), 45);

    session.cycle_crew().unwrap();
    assert_eq!(session.crew_role(), CrewRole::Rigger);
    assert_eq!(session.crew_external_capacity(), 1);

    session.cycle_crew().unwrap();
    assert_eq!(session.crew_role(), CrewRole::SafetyOfficer);
    assert_eq!(session.crew_adjusted_danger(45), 37);

    session.cycle_crew().unwrap();
    assert_eq!(session.crew_role(), CrewRole::Broker);
    assert_eq!(session.crew_contract_bonus_percent(), 5);
}

#[test]
fn crew_expertise_grows_by_role_and_unlocks_tiered_operating_bonuses() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);

    assert_eq!(
        GameSession::crew_experience_gain(RiskOutcome::OrdinaryReturn, true),
        2
    );
    assert_eq!(
        session.record_crew_experience(RiskOutcome::OrdinaryReturn, false),
        1
    );
    assert_eq!(session.crew_experience(), 1);
    assert_eq!(session.crew_expertise_level(), 0);

    session.crew_role = CrewRole::Navigator;
    session.career.crew_experience[CrewRole::Navigator.index()] = 3;
    assert_eq!(session.crew_fuel_delta(), -2);

    session.crew_role = CrewRole::Rigger;
    session.career.crew_experience[CrewRole::Rigger.index()] = 6;
    assert_eq!(session.crew_external_capacity(), 2);

    session.crew_role = CrewRole::SafetyOfficer;
    session.career.crew_experience[CrewRole::SafetyOfficer.index()] = 3;
    assert_eq!(session.crew_adjusted_danger(45), 33);

    session.crew_role = CrewRole::Broker;
    session.career.crew_experience[CrewRole::Broker.index()] = 6;
    assert_eq!(session.crew_contract_bonus_percent(), 9);
}

#[test]
fn port_training_buys_expertise_for_the_active_role() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);

    assert_eq!(session.crew_training_cost(&data), Some(160));
    session.train_crew(&data).unwrap();
    assert_eq!(session.crew_experience(), 1);
    assert_eq!(session.economy.credits, 690);
    assert_eq!(session.crew_training_cost(&data), Some(260));
}

#[test]
fn port_training_reports_low_funds_and_the_certified_cap() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.economy.credits = 100;

    assert!(session.train_crew(&data).is_err());

    session.career.crew_experience[CrewRole::Deckhand.index()] = MAX_CREW_EXPERIENCE;
    assert_eq!(session.crew_training_cost(&data), None);
    assert!(session.train_crew(&data).is_err());
}

#[test]
fn veteran_deckhands_reduce_the_fatigue_of_a_completed_run() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.career.crew_experience[CrewRole::Deckhand.index()] = 3;

    let gain = session.register_crew_fatigue(RiskOutcome::OrdinaryReturn, 0, 0);

    assert_eq!(gain, 7);
    assert_eq!(session.crew_readiness(), 93);
}

#[test]
fn veteran_crew_experience_caps_without_overflow() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.crew_role = CrewRole::Broker;
    session.career.crew_experience[CrewRole::Broker.index()] = 5;
    let gain = session.record_crew_experience(RiskOutcome::ForcedAbandon, true);

    assert_eq!(gain, 1);
    assert_eq!(session.crew_experience(), 6);
    assert_eq!(
        session.record_crew_experience(RiskOutcome::OrdinaryReturn, true),
        0
    );
    assert_eq!(session.crew_experience(), 6);
}

#[test]
fn crew_cannot_be_reassigned_during_a_live_or_unresolved_run() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.expedition = Some(crate::state::ExpeditionState {
        workspace_transfer: None,
        site_id: "merchant_wreck".to_owned(),
        cargo: Vec::new(),
        risk: crate::engine::RiskResult {
            outcome: crate::engine::RiskOutcome::OrdinaryReturn,
            danger_score: 0,
            explanation: String::new(),
        },
        seed: 1,
        workspace_section: "cargo_bay".to_owned(),
        workspace_scanned: false,
        revealed_targets: Vec::new(),
        stabilized_targets: Vec::new(),
        scan_profile: crate::state::WorkspaceScanProfile::Standard,
        drones_deployed: false,
        drone_directive: crate::state::DroneDirective::Standby,
        workspace_energy: 1,
        workspace_energy_capacity: 1,
        power_cycles_used: 0,
        insured: false,
        contract_accepted: true,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
    });
    assert!(session.cycle_crew().is_err());
}
