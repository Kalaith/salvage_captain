use crate::data::{GameData, GridPosition};
use crate::engine::{RiskOutcome, RiskResult};
use crate::state::{
    CargoStatus, GameSession, ReturnedItem, VoyageRecord, WorkspaceLogEvent, WorkspaceScanProfile,
};

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
        market_cycle: 0,
    });

    session.record_voyage(
        "merchant_wreck",
        &RiskResult {
            outcome: RiskOutcome::OrdinaryReturn,
            danger_score: 12,
            explanation: "clear".to_owned(),
        },
        crate::engine::VoyagePlan::Standard,
        0,
        1,
        true,
        false,
        WorkspaceScanProfile::Array,
        crate::state::DroneDirective::Survey,
        64,
        2,
        &[],
        0,
        false,
        0,
        0,
        &data,
    );

    let record = session.last_voyage().expect("voyage record");
    assert_eq!(record.site_id, "merchant_wreck");
    assert_eq!(record.recovered_count, 1);
    assert_eq!(
        record.recovered_value,
        crate::engine::market::quote_for(
            data.salvage_objects.get(&target_id).unwrap(),
            0,
            &data.config.market,
        )
        .sale_value
    );
    assert_eq!(record.external_load, 1);
    assert_eq!(record.risk_outcome, RiskOutcome::OrdinaryReturn);
    assert!(record.contract_completed);
    assert!(!record.contract_failed);
    assert_eq!(record.scan_profile, WorkspaceScanProfile::Array);
    assert_eq!(record.drone_directive, crate::state::DroneDirective::Survey);
    assert_eq!(record.condition_after, 64);
    assert_eq!(record.return_fuel, 2);
    assert_eq!(session.career.voyages_completed, 1);
    assert_eq!(session.career.safe_returns, 1);
    assert_eq!(session.career.targets_recovered, 1);
    assert_eq!(session.career.gross_haul_value, record.recovered_value);
    assert_eq!(session.career.highest_haul_value, record.recovered_value);
    assert_eq!(session.career.contracts_completed, 1);
    assert_eq!(session.career.sections_cleared, 0);
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
    assert_eq!(record.scan_profile, WorkspaceScanProfile::Standard);
}

#[test]
fn closing_a_briefed_run_files_its_route_intelligence_level() {
    let data = GameData::load().expect("valid game data");
    let mut session = GameSession::new(&data);
    session.buy_reconnaissance("merchant_wreck", &data).unwrap();
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.leave_all_pending().unwrap();

    session.finish_packing(&data).unwrap();

    assert_eq!(session.last_voyage().unwrap().reconnaissance_level, 1);
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
        recovered_alloy: 0,
        recovered_electronics: 0,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 10,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        contract_completed: false,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 80,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    });

    let error = GameSession::from_save(save, &data).unwrap_err();
    assert!(error.contains("voyage log references unknown site"));
}

#[test]
fn save_rejects_uninsured_claim_records() {
    let data = GameData::load().expect("valid game data");
    let mut save = GameSession::new(&data).to_save(&data.config.version);
    save.session.voyage_log.push(VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 0,
        recovered_value: 0,
        recovered_alloy: 0,
        recovered_electronics: 0,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 10,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        contract_completed: false,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 80,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 1,
    });

    let error = GameSession::from_save(save, &data).unwrap_err();
    assert!(error.contains("invalid voyage log measurement"));
}

#[test]
fn old_voyage_records_default_to_standard_scan() {
    let value = serde_json::json!({
        "site_id": "merchant_wreck",
        "recovered_count": 1,
        "recovered_value": 20,
        "external_load": 0,
        "risk_outcome": "OrdinaryReturn",
        "danger_score": 10,
        "contract_completed": false,
        "contract_failed": false,
        "condition_after": 80
    });
    let record: VoyageRecord = serde_json::from_value(value).unwrap();
    assert_eq!(record.scan_profile, WorkspaceScanProfile::Standard);
    assert_eq!(record.voyage_plan, crate::engine::VoyagePlan::Standard);
    assert_eq!(record.return_fuel, 0);
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
