//! Crew fatigue, readiness, and rest rules.

use super::*;
use crate::data::GameData;
use crate::state::crew_readiness::*;

#[test]
fn hard_runs_leave_a_bounded_readiness_load() {
    assert_eq!(fatigue_gain(RiskOutcome::OrdinaryReturn, 0, 0), 8);
    assert_eq!(fatigue_gain(RiskOutcome::EmergencyRepair, 3, 1), 27);
    assert_eq!(fatigue_gain(RiskOutcome::ForcedAbandon, 9, 1), 37);
    assert_eq!(fatigue_gain(RiskOutcome::OrdinaryReturn, 99, 9), 21);
}

#[test]
fn crew_readiness_adds_small_route_pressure_until_rest() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);

    assert_eq!(session.crew_fatigue(), 0);
    assert_eq!(session.crew_readiness(), 100);
    assert_eq!(session.crew_fatigue_danger_delta(), 0);
    session.register_crew_fatigue(RiskOutcome::DamagedModule, 2, 1);
    assert_eq!(session.crew_fatigue(), 21);
    assert_eq!(session.crew_readiness(), 79);
    assert_eq!(session.crew_fatigue_danger_delta(), 2);
    assert_eq!(session.crew_adjusted_danger(45), 47);
    session.rest_crew().unwrap();
    assert_eq!(session.crew_fatigue(), 0);
}

#[test]
fn crew_cannot_rest_during_an_unresolved_expedition() {
    let data = GameData::load().expect("game data");
    let mut session = GameSession::new(&data);
    session.expedition = Some(crate::state::ExpeditionState {
        site_id: "merchant_wreck".to_owned(),
        cargo: Vec::new(),
        risk: crate::engine::RiskResult {
            outcome: RiskOutcome::OrdinaryReturn,
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

    assert!(session.rest_crew().is_err());
}
