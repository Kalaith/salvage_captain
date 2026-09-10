use super::*;
use crate::data::GameData;

#[test]
fn scan_reveals_the_authored_merchant_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let message = session.scan_workspace(&data).unwrap();
    let expedition = session.expedition.as_ref().unwrap();
    assert!(expedition.workspace_scanned);
    assert!(message.contains("Site recovery is 0/"));
    assert!(expedition
        .revealed_targets
        .contains(&"industrial_battery".to_owned()));
    assert!(expedition
        .revealed_targets
        .contains(&"engine_assembly".to_owned()));
}

#[test]
fn losing_the_contract_target_marks_the_briefing_failed() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();

    let message = session
        .lose_workspace_target("industrial_battery", &data)
        .unwrap();

    assert!(message.contains("Contract failed"));
    assert!(
        session
            .site_progress
            .get("merchant_wreck")
            .unwrap()
            .contract_failed
    );
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
    assert!(session.has_capability("basic_tractor", &data));
    assert!(!session.has_capability("stabilizer", &data));
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

#[test]
fn returning_to_a_known_section_restores_its_scan() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .switch_workspace_section("engineering_access", &data)
        .unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .switch_workspace_section("cargo_bay", &data)
        .unwrap();
    let expedition = session.expedition.as_ref().unwrap();
    assert!(expedition.workspace_scanned);
    assert!(expedition
        .revealed_targets
        .contains(&"navigation_computer".to_owned()));
}

#[test]
fn drone_bay_shortens_extraction_time() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let baseline = session
        .extraction_duration("industrial_battery", &data)
        .unwrap();

    session.purchase_module("drone_bay", &data).unwrap();

    let supported = session
        .extraction_duration("industrial_battery", &data)
        .unwrap();
    assert_eq!(session.module_stats(&data).drone_support, 1);
    assert!(supported < baseline);
}
