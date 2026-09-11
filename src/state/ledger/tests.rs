use crate::data::{GameData, GridPosition};
use crate::engine::{RiskOutcome, RiskResult};
use crate::state::{CargoStatus, GameSession, ReturnedItem, VoyageRecord, WorkspaceLogEvent};

#[test]
fn completed_voyage_records_returned_value_and_outcome() {
    let data = GameData::load().expect("valid game data");
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let target_id = session.expedition.as_ref().unwrap().cargo[0]
        .object_id
        .clone();
    let position = GridPosition::new(3, 3);
    session.expedition.as_mut().unwrap().cargo[0].status = CargoStatus::Packed;
    session.expedition.as_mut().unwrap().cargo[0].position = Some(position);
    session
        .ship_layout
        .place(
            format!("cargo:{target_id}"),
            data.salvage_objects.get(&target_id).unwrap().footprint,
            position,
            0,
            false,
        )
        .unwrap();
    session.returned.push(ReturnedItem {
        object_id: target_id.clone(),
        position,
        rotation: 0,
    });

    session.record_voyage(
        "merchant_wreck",
        &RiskResult {
            outcome: RiskOutcome::OrdinaryReturn,
            danger_score: 12,
            explanation: "clear".to_owned(),
        },
        1,
        true,
        false,
        64,
        &data,
    );

    let record = session.last_voyage().expect("voyage record");
    assert_eq!(record.site_id, "merchant_wreck");
    assert_eq!(record.recovered_count, 1);
    assert_eq!(
        record.recovered_value,
        data.salvage_objects.get(&target_id).unwrap().sale_value
    );
    assert_eq!(record.external_load, 1);
    assert_eq!(record.risk_outcome, RiskOutcome::OrdinaryReturn);
    assert!(record.contract_completed);
    assert!(!record.contract_failed);
    assert_eq!(record.condition_after, 64);
}

#[test]
fn closing_a_run_appends_a_ledger_entry() {
    let data = GameData::load().expect("valid game data");
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.leave_all_pending().unwrap();

    session.finish_packing(&data).unwrap();

    let record = session.last_voyage().expect("completed run is logged");
    assert_eq!(record.site_id, "merchant_wreck");
    assert_eq!(record.recovered_count, 0);
    assert_eq!(record.external_load, 0);
    assert_eq!(record.condition_after, 64);
    assert!(!record.contract_completed);
}

#[test]
fn operation_log_remains_available_after_returning_to_debrief() {
    let data = GameData::load().expect("valid game data");
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session.leave_all_pending().unwrap();

    session.finish_packing(&data).unwrap();

    let log = session
        .site_progress
        .get("merchant_wreck")
        .expect("site progress")
        .operation_log
        .as_slice();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].event, WorkspaceLogEvent::Departed);
    assert_eq!(log[1].event, WorkspaceLogEvent::SectionScanned);
}

#[test]
fn save_rejects_impossible_voyage_log_entries() {
    let data = GameData::load().expect("valid game data");
    let mut save = GameSession::new(&data).to_save(&data.config.version);
    save.session.voyage_log.push(VoyageRecord {
        site_id: "missing_wreck".to_owned(),
        recovered_count: 1,
        recovered_value: 20,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 10,
        contract_completed: false,
        contract_failed: false,
        condition_after: 80,
    });

    let error = GameSession::from_save(save, &data).unwrap_err();
    assert!(error.contains("voyage log references unknown site"));
}

#[test]
fn old_saves_default_contract_failure_to_false() {
    let data = GameData::load().expect("valid game data");
    let session = GameSession::new(&data);
    let save = session.to_save(&data.config.version);
    let mut value = serde_json::to_value(save).unwrap();
    for progress in value["session"]["site_progress"]
        .as_object_mut()
        .unwrap()
        .values_mut()
    {
        progress.as_object_mut().unwrap().remove("contract_failed");
    }

    let restored: crate::state::SaveData = serde_json::from_value(value).unwrap();
    assert!(restored
        .session
        .site_progress
        .values()
        .all(|progress| !progress.contract_failed));
}

#[test]
fn save_rejects_a_contract_marked_complete_and_failed() {
    let data = GameData::load().expect("valid game data");
    let mut save = GameSession::new(&data).to_save(&data.config.version);
    let progress = save
        .session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap();
    progress.contract_completed = true;
    progress.contract_failed = true;

    let error = GameSession::from_save(save, &data).unwrap_err();
    assert!(error.contains("marked complete and failed"));
}
