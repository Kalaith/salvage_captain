use super::*;
use crate::data::GameData;

#[test]
fn crew_roles_cycle_in_a_visible_operating_order() {
    assert_eq!(CrewRole::ALL.len(), 5);
    assert_eq!(CrewRole::Deckhand.next(), CrewRole::Navigator);
    assert_eq!(CrewRole::SafetyOfficer.next(), CrewRole::Broker);
    assert_eq!(CrewRole::Broker.next(), CrewRole::Deckhand);
    assert_eq!(
        CrewRole::SafetyOfficer.description(),
        "-8 route danger // disciplined return watch"
    );
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

    assert!(session.cycle_crew().unwrap().contains("NAVIGATOR"));
    assert_eq!(session.crew_fuel_delta(), -1);
    assert_eq!(session.crew_adjusted_danger(45), 45);

    assert!(session.cycle_crew().unwrap().contains("RIGGER"));
    assert_eq!(session.crew_external_capacity(), 1);

    assert!(session.cycle_crew().unwrap().contains("SAFETY OFFICER"));
    assert_eq!(session.crew_adjusted_danger(45), 37);

    assert!(session.cycle_crew().unwrap().contains("BROKER"));
    assert_eq!(session.crew_contract_bonus_percent(), 5);
}

#[test]
fn crew_expertise_grows_by_role_and_unlocks_tiered_operating_bonuses() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);

    assert_eq!(session.crew_expertise_label(), "NOVICE");
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
    assert_eq!(session.crew_expertise_label(), "QUALIFIED");
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
fn veteran_deckhands_reduce_the_fatigue_of_a_completed_run() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.career.crew_experience[CrewRole::Deckhand.index()] = 3;

    let gain = session.register_crew_fatigue(RiskOutcome::OrdinaryReturn, 0, 0);

    assert_eq!(gain, 7);
    assert_eq!(session.crew_readiness(), 93);
}

#[test]
fn crew_experience_report_calls_out_a_new_qualification() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.career.crew_experience[CrewRole::Deckhand.index()] = 2;
    let previous_level = session.crew_expertise_level();
    session.record_crew_experience(RiskOutcome::OrdinaryReturn, false);

    assert_eq!(
        session.crew_experience_report(1, previous_level),
        "Crew DECKHAND expertise +1 // PROMOTED QUALIFIED."
    );
}

#[test]
fn veteran_crew_experience_caps_without_overflow() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.crew_role = CrewRole::Broker;
    session.career.crew_experience[CrewRole::Broker.index()] = 5;
    let previous_level = session.crew_expertise_level();

    let gain = session.record_crew_experience(RiskOutcome::ForcedAbandon, true);

    assert_eq!(gain, 1);
    assert_eq!(session.crew_experience(), 6);
    assert_eq!(session.crew_expertise_label(), "VETERAN");
    assert!(session
        .crew_experience_report(gain, previous_level)
        .contains("PROMOTED VETERAN"));
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
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
    });
    assert!(session.cycle_crew().unwrap_err().contains("safe port"));
}
