use super::*;
use crate::state::WorkspaceScanProfile;

#[test]
fn mission_briefing_names_the_next_ship_blueprint() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let label = blueprint_progress_label(&session, &data);

    assert!(label.contains("SHIP BLUEPRINTS 3/9"));
    assert!(label.contains("NEXT NAV MODULE @ ¢900"));
    assert!(label.contains("STAND INDEPENDENT // REP 0/2"));
}

#[test]
fn mission_briefing_keeps_blueprints_on_empty_last_run_line() {
    let label = site_last_run_label(None, 0, 0, 3, 9);

    assert_eq!(
        label,
        "LAST RUN  NONE  //  LOG 00  //  SURV 00  //  BP 03/09"
    );
}

#[test]
fn mission_briefing_keeps_blueprints_on_completed_last_run_line() {
    let record = VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 2,
        recovered_value: 250,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        contract_completed: true,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        condition_after: 80,
    };
    let label = site_last_run_label(Some(&record), 4, 3, 5, 9);

    assert_eq!(
        label,
        "LAST ORDINARY RETURN  //  TGT 2  //  ¢250  //  LOG 04  //  SURV 03  //  BP 05/09"
    );
}
