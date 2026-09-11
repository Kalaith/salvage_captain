use super::*;
use crate::state::WorkspaceLogEntry;

#[test]
fn fully_recovered_section_pays_once_and_files_its_clearance() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let progress = session.site_progress.get_mut("merchant_wreck").unwrap();
    progress.operation_log = vec![
        WorkspaceLogEntry::new(
            1,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("industrial_battery"),
        ),
        WorkspaceLogEntry::new(
            2,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("navigation_computer"),
        ),
        WorkspaceLogEntry::new(
            3,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("engine_assembly"),
        ),
    ];
    let credits_before = session.economy.credits;

    let report = session.resolve_section_clearance("merchant_wreck", &data);

    assert_eq!(report.section_ids, vec!["cargo_bay"]);
    assert_eq!(report.payout, 140);
    assert_eq!(session.economy.credits, credits_before + 140);
    assert_eq!(
        session.site_clearance_summary("merchant_wreck", &data),
        (1, 2)
    );
    assert_eq!(
        session
            .site_progress
            .get("merchant_wreck")
            .unwrap()
            .operation_log
            .last()
            .unwrap()
            .event,
        WorkspaceLogEvent::SectionCleared
    );
    assert!(
        session
            .section_clearance_status("merchant_wreck", "cargo_bay", &data)
            .unwrap()
            .cleared
    );

    let second = session.resolve_section_clearance("merchant_wreck", &data);
    assert!(second.section_ids.is_empty());
    assert_eq!(session.economy.credits, credits_before + 140);
}

#[test]
fn lost_target_does_not_qualify_a_section_for_clearance() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let progress = session.site_progress.get_mut("merchant_wreck").unwrap();
    progress.operation_log = vec![
        WorkspaceLogEntry::new(
            1,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("industrial_battery"),
        ),
        WorkspaceLogEntry::new(
            2,
            WorkspaceLogEvent::TargetLost,
            Some("cargo_bay"),
            Some("navigation_computer"),
        ),
        WorkspaceLogEntry::new(
            3,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("engine_assembly"),
        ),
    ];

    let report = session.resolve_section_clearance("merchant_wreck", &data);

    assert!(report.section_ids.is_empty());
    assert_eq!(
        session.site_clearance_summary("merchant_wreck", &data),
        (0, 2)
    );
}

#[test]
fn clearance_status_exposes_the_bounty_before_it_is_paid() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let progress = session.site_progress.get_mut("merchant_wreck").unwrap();
    progress.operation_log = vec![
        WorkspaceLogEntry::new(
            1,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("industrial_battery"),
        ),
        WorkspaceLogEntry::new(
            2,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("navigation_computer"),
        ),
        WorkspaceLogEntry::new(
            3,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("engine_assembly"),
        ),
    ];

    let status = session
        .section_clearance_status("merchant_wreck", "cargo_bay", &data)
        .unwrap();

    assert_eq!(status.recovered_targets, 3);
    assert_eq!(status.total_targets, 3);
    assert_eq!(status.reward, 140);
    assert!(status.ready);
    assert!(!status.cleared);
}

#[test]
fn closing_a_run_pays_new_section_clearance_and_records_it() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let progress = session.site_progress.get_mut("merchant_wreck").unwrap();
    progress.removed_targets = vec![
        "industrial_battery".to_owned(),
        "navigation_computer".to_owned(),
        "engine_assembly".to_owned(),
    ];
    progress.operation_log.extend([
        WorkspaceLogEntry::new(
            2,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("industrial_battery"),
        ),
        WorkspaceLogEntry::new(
            3,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("navigation_computer"),
        ),
        WorkspaceLogEntry::new(
            4,
            WorkspaceLogEvent::TargetRecovered,
            Some("cargo_bay"),
            Some("engine_assembly"),
        ),
    ]);
    session.leave_all_pending().unwrap();
    let credits_before = session.economy.credits;

    session.finish_packing(&data).unwrap();

    assert_eq!(session.economy.credits, credits_before + 140);
    let record = session.last_voyage().unwrap();
    assert_eq!(record.cleared_sections, vec!["cargo_bay"]);
    assert_eq!(record.clearance_payout, 140);
}
