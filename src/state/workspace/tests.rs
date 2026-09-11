use super::*;
use crate::data::GameData;

#[test]
fn extraction_phase_schedule_stays_deterministic() {
    let mut extraction = ExtractionRuntime::new("industrial_battery", 100.0);
    for (progress, expected) in [
        (0.00, ExtractionPhase::Alignment),
        (0.12, ExtractionPhase::Connection),
        (0.28, ExtractionPhase::Strain),
        (0.55, ExtractionPhase::Separation),
        (0.68, ExtractionPhase::Retrieval),
        (0.92, ExtractionPhase::Capture),
    ] {
        extraction.elapsed = progress * extraction.duration;
        assert_eq!(extraction.phase(), expected);
    }
}

#[test]
fn scan_reveals_the_authored_merchant_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let message = session.scan_workspace(&data).unwrap();
    let expedition = session.expedition.as_ref().unwrap();
    assert!(expedition.workspace_scanned);
    assert!(message.contains("Site recovery is 0/"));
    assert!(message.contains("4 remain"));
    assert!(expedition
        .revealed_targets
        .contains(&"industrial_battery".to_owned()));
    assert!(expedition
        .revealed_targets
        .contains(&"engine_assembly".to_owned()));
}

#[test]
fn wreck_status_tracks_explored_frames_and_recovered_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();

    let initial = session.site_recovery_status("merchant_wreck", &data);
    assert_eq!(initial.explored_sections, 0);
    assert_eq!(initial.exploration_percent, 0);
    assert_eq!(initial.recovered_targets, 0);

    session.scan_workspace(&data).unwrap();
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    let first_frame = session.site_recovery_status("merchant_wreck", &data);
    assert_eq!(first_frame.explored_sections, 1);
    assert_eq!(first_frame.exploration_percent, 50);
    assert_eq!(first_frame.recovered_targets, 1);
    assert_eq!(first_frame.total_targets, 4);
    assert_eq!(first_frame.remaining_targets, 3);
}

#[test]
fn workspace_condition_reflects_persistent_frame_wear() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();

    let unread = session.workspace_condition_status(&data).unwrap();
    assert_eq!(unread.frame_condition, 82);
    assert_eq!(unread.section_condition, 82);
    assert_eq!(unread.label(), "UNMAPPED");

    session.scan_workspace(&data).unwrap();
    let scanned = session.workspace_condition_status(&data).unwrap();
    assert_eq!(scanned.label(), "STABLE");

    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    let salvaged = session.workspace_condition_status(&data).unwrap();
    assert_eq!(salvaged.frame_condition, 82);
    assert_eq!(salvaged.section_condition, 74);
    assert_eq!(salvaged.recovered_targets, 1);
    assert_eq!(salvaged.total_targets, 3);
    assert_eq!(salvaged.structural_stress(), 26);
    assert_eq!(salvaged.label(), "STRESSED");

    let restored = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();
    let restored_status = restored.workspace_condition_status(&data).unwrap();
    assert_eq!(restored_status, salvaged);
}

#[test]
fn scanning_and_extraction_spend_the_expedition_power_reserve() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    assert_eq!(session.workspace_energy(), Some((12, 12)));

    session.scan_workspace(&data).unwrap();
    assert_eq!(session.workspace_energy(), Some((11, 12)));

    let repeat_message = session.scan_workspace(&data).unwrap();
    assert!(repeat_message.contains("already scanned"));
    assert_eq!(session.workspace_energy(), Some((11, 12)));

    let message = session
        .reserve_workspace_energy("industrial_battery", &data)
        .unwrap();
    assert!(message.contains("Power reserve -1"));
    assert_eq!(session.workspace_energy(), Some((10, 12)));
}

#[test]
fn stabilizing_a_revealed_hazard_spends_power_and_survives_a_save() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.purchase_module("shield_module", &data).unwrap();
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();

    let message = session
        .stabilize_workspace_target("navigation_computer", &data)
        .unwrap();

    assert!(message.contains("Exposure -20"));
    assert!(message.contains("Tap LOAD CARGO"));
    assert_eq!(session.workspace_energy(), Some((9, 12)));
    assert!(session.target_is_stabilized("navigation_computer"));
    let restored = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();
    assert!(restored.target_is_stabilized("navigation_computer"));
}

#[test]
fn stabilization_lock_clears_when_the_target_leaves_the_wreck() {
    let data = GameData::load().unwrap();
    let mut recovered_session = GameSession::new(&data);
    recovered_session
        .purchase_module("shield_module", &data)
        .unwrap();
    recovered_session
        .begin_expedition("merchant_wreck", &data)
        .unwrap();
    recovered_session.scan_workspace(&data).unwrap();
    recovered_session
        .stabilize_workspace_target("navigation_computer", &data)
        .unwrap();
    recovered_session
        .recover_workspace_target("navigation_computer", &data)
        .unwrap();
    assert!(!recovered_session.target_is_stabilized("navigation_computer"));

    let mut lost_session = GameSession::new(&data);
    lost_session
        .purchase_module("shield_module", &data)
        .unwrap();
    lost_session
        .begin_expedition("merchant_wreck", &data)
        .unwrap();
    lost_session.scan_workspace(&data).unwrap();
    lost_session
        .stabilize_workspace_target("navigation_computer", &data)
        .unwrap();
    lost_session
        .lose_workspace_target("navigation_computer", &data)
        .unwrap();
    assert!(!lost_session.target_is_stabilized("navigation_computer"));
}

#[test]
fn operation_log_records_workspace_actions_and_survives_a_save() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.purchase_module("shield_module", &data).unwrap();
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .stabilize_workspace_target("navigation_computer", &data)
        .unwrap();
    session
        .reserve_workspace_energy("industrial_battery", &data)
        .unwrap();
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    session
        .switch_workspace_section("engineering_access", &data)
        .unwrap();
    session.scan_workspace(&data).unwrap();

    let log = session.workspace_log().unwrap();
    assert_eq!(log.len(), 7);
    assert_eq!(log[0].event, WorkspaceLogEvent::Departed);
    assert_eq!(log[1].event, WorkspaceLogEvent::SectionScanned);
    assert_eq!(log[2].event, WorkspaceLogEvent::TargetStabilized);
    assert_eq!(log[2].target_id.as_deref(), Some("navigation_computer"));
    assert_eq!(log[3].event, WorkspaceLogEvent::ExtractionStarted);
    assert_eq!(log[4].event, WorkspaceLogEvent::TargetRecovered);
    assert_eq!(log[4].target_id.as_deref(), Some("industrial_battery"));
    assert_eq!(log[5].event, WorkspaceLogEvent::EnteredSection);
    assert_eq!(log[6].event, WorkspaceLogEvent::SectionScanned);
    assert!(log
        .windows(2)
        .all(|pair| pair[0].sequence < pair[1].sequence));

    let restored = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();
    assert_eq!(restored.workspace_log().unwrap(), log);
}

#[test]
fn cancelled_pull_is_recorded_as_a_field_event() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .reserve_workspace_energy("industrial_battery", &data)
        .unwrap();
    session.record_workspace_event(
        WorkspaceLogEvent::ExtractionCancelled,
        Some("industrial_battery"),
    );

    let log = session.workspace_log().unwrap();
    assert_eq!(
        log.last().map(|entry| entry.event),
        Some(WorkspaceLogEvent::ExtractionCancelled)
    );
    assert_eq!(
        log.last().and_then(|entry| entry.target_id.as_deref()),
        Some("industrial_battery")
    );
}

#[test]
fn extraction_explains_when_the_power_reserve_is_empty() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session.expedition.as_mut().unwrap().workspace_energy = 0;

    let reason = session
        .extraction_block_reason("industrial_battery", &data)
        .unwrap()
        .unwrap();

    assert!(reason.contains("Power reserve insufficient"));
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
fn recovery_message_names_external_clamp_destination() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("military_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();

    let message = session
        .recover_workspace_target("titanium_plating", &data)
        .unwrap();

    assert!(message.contains("external clamp queue"));
}

#[test]
fn transfer_modes_keep_commands_and_destinations_distinct() {
    let data = GameData::load().unwrap();
    let cases = [
        (
            "industrial_battery",
            TransferMode::InternalCargo,
            "LOAD CARGO",
            "SALVAGE HOLD",
        ),
        (
            "titanium_plating",
            TransferMode::ExternalClamp,
            "LOCK CLAMP",
            "EXTERNAL CLAMP",
        ),
        (
            "engine_assembly",
            TransferMode::Tow,
            "ENGAGE TOW",
            "TOW RIG",
        ),
    ];
    for (target_id, expected_mode, command, destination) in cases {
        let target = data.salvage_objects.get(target_id).unwrap();
        let mode = TransferMode::from_target(target);
        assert_eq!(mode, expected_mode);
        assert_eq!(mode.command_label(), command);
        assert_eq!(mode.destination_label(), destination);
        assert!(mode.cancel_label().starts_with("CANCEL "));
    }
}

#[test]
fn transfer_modes_only_external_loads_use_the_rig() {
    assert!(!TransferMode::InternalCargo.uses_external_rig());
    assert!(TransferMode::ExternalClamp.uses_external_rig());
    assert!(TransferMode::Tow.uses_external_rig());
}

#[test]
fn recovery_message_names_tow_destination() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.purchase_module("reactor_module", &data).unwrap();
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();

    let message = session
        .recover_workspace_target("engine_assembly", &data)
        .unwrap();

    assert!(message.contains("tow rig queue"));
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
