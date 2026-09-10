use super::*;
use crate::data::GameData;

#[test]
fn hazard_resolver_keeps_the_tutorial_relay_stable() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("industrial_battery").unwrap();
    let report = resolve_extraction(
        7,
        15,
        &["decompression".to_owned(), "moving_debris".to_owned()],
        target,
        ModuleStats::default(),
        false,
        false,
    );
    assert_eq!(report.outcome, WorkspaceOutcome::Recovered);
    assert_eq!(exposure_label(report.exposure), "STABLE");
}

#[test]
fn stabilizer_reduces_exposure_for_a_dangerous_pull() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("shield_generator").unwrap();
    let hazards = vec!["automated_defenses".to_owned(), "moving_debris".to_owned()];
    let without = resolve_extraction(
        13,
        45,
        &hazards,
        target,
        ModuleStats::default(),
        false,
        false,
    );
    let with_stabilizer = resolve_extraction(
        13,
        45,
        &hazards,
        target,
        ModuleStats::default(),
        true,
        false,
    );
    assert!(with_stabilizer.exposure < without.exposure);
    assert_eq!(with_stabilizer.mitigation - without.mitigation, 24);
}

#[test]
fn identical_inputs_resolve_to_the_same_workspace_outcome() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("navigation_computer").unwrap();
    let hazards = vec!["electrical_arcs".to_owned()];
    let first = resolve_extraction(
        99,
        15,
        &hazards,
        target,
        ModuleStats::default(),
        false,
        false,
    );
    let second = resolve_extraction(
        99,
        15,
        &hazards,
        target,
        ModuleStats::default(),
        false,
        false,
    );
    assert_eq!(first, second);
}
