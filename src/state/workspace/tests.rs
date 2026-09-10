use super::*;
use crate::data::GameData;

#[test]
fn scan_reveals_the_authored_merchant_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    let expedition = session.expedition.as_ref().unwrap();
    assert!(expedition.workspace_scanned);
    assert!(expedition
        .revealed_targets
        .contains(&"industrial_battery".to_owned()));
    assert!(expedition
        .revealed_targets
        .contains(&"engine_assembly".to_owned()));
}

#[test]
fn starter_tractor_explains_the_engine_gate() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    let reason = session
        .extraction_block_reason("engine_assembly", &data)
        .unwrap()
        .unwrap();
    assert!(reason.contains("Heavy Tractor"));
    assert_eq!(session.tractor_capacity_tons(&data), 8.0);
}

#[test]
fn recovered_target_persists_as_an_empty_mount() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    assert!(session.target_is_removed("industrial_battery"));
    assert!(session
        .expedition
        .as_ref()
        .unwrap()
        .cargo
        .iter()
        .any(|item| item.object_id == "industrial_battery"));
    assert!(!session
        .expedition
        .as_ref()
        .unwrap()
        .revealed_targets
        .contains(&"industrial_battery".to_owned()));
    let save = session.to_save(&data.config.version);
    let restored = GameSession::from_save(save, &data).unwrap();
    assert!(restored.target_is_removed("industrial_battery"));
}

#[test]
fn gated_reactor_section_explains_missing_stabilizer() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("military_wreck", &data).unwrap();
    let error = session
        .switch_workspace_section("reactor_spine", &data)
        .unwrap_err();
    assert!(error.contains("Stabilizer"));
}

#[test]
fn section_change_returns_the_authored_arrival_briefing() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let message = session
        .switch_workspace_section("engineering_access", &data)
        .unwrap();
    assert!(message.contains("narrow service run"));
}
