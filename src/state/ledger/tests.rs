use crate::data::{GameData, GridPosition};
use crate::engine::{RiskOutcome, RiskResult};
use crate::state::{CargoStatus, GameSession, ReturnedItem};

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
