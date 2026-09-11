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
        0,
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
        0,
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
        0,
        false,
    );
    assert!(with_stabilizer.exposure < without.exposure);
    assert_eq!(with_stabilizer.mitigation - without.mitigation, 24);
}

#[test]
fn active_stabilization_adds_a_distinct_exposure_reduction() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("engine_assembly").unwrap();
    let without_lock = resolve_extraction(
        7,
        45,
        &["moving_debris".to_owned()],
        target,
        ModuleStats::default(),
        false,
        false,
        0,
        false,
    );
    let with_lock = resolve_extraction(
        7,
        45,
        &["moving_debris".to_owned()],
        target,
        ModuleStats::default(),
        false,
        false,
        0,
        true,
    );

    assert_eq!(with_lock.mitigation - without_lock.mitigation, 20);
    assert!(with_lock.exposure < without_lock.exposure);
}

#[test]
fn survey_drones_reduce_extraction_exposure() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("shield_generator").unwrap();
    let hazards = vec!["automated_defenses".to_owned(), "moving_debris".to_owned()];
    let without_drones = resolve_extraction(
        13,
        45,
        &hazards,
        target,
        ModuleStats::default(),
        false,
        false,
        0,
        false,
    );
    let with_drones = resolve_extraction(
        13,
        45,
        &hazards,
        target,
        ModuleStats::default(),
        false,
        false,
        1,
        false,
    );

    assert_eq!(with_drones.mitigation - without_drones.mitigation, 8);
    assert!(with_drones.exposure < without_drones.exposure);
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
        0,
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
        0,
        false,
    );
    assert_eq!(first, second);
}

#[test]
fn authored_target_hazards_have_typed_response_signals() {
    let data = GameData::load().unwrap();
    for (target_id, expected) in [
        ("damaged_reactor", WorkspaceHazard::ReactorInstability),
        ("navigation_computer", WorkspaceHazard::ElectricalArcs),
        ("shield_generator", WorkspaceHazard::AutomatedDefenses),
        ("military_crate", WorkspaceHazard::UnexplodedAmmunition),
        ("quantum_lens", WorkspaceHazard::MagneticInterference),
        ("engine_assembly", WorkspaceHazard::StructuralCollapse),
    ] {
        let target = data.salvage_objects.get(target_id).unwrap();
        assert_eq!(
            target
                .hazard
                .as_deref()
                .and_then(WorkspaceHazard::from_value),
            Some(expected)
        );
        assert!(!expected.response_label().is_empty());
    }
}

#[test]
fn extraction_report_carries_the_authored_hazard_signal() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("navigation_computer").unwrap();
    let report = resolve_extraction(
        99,
        15,
        &["electrical_arcs".to_owned()],
        target,
        ModuleStats::default(),
        false,
        false,
        0,
        false,
    );

    assert_eq!(report.hazard, Some(WorkspaceHazard::ElectricalArcs));
}

#[test]
fn authored_hazards_add_their_specific_exposure_load() {
    let data = GameData::load().unwrap();
    let target = data.salvage_objects.get("engine_assembly").unwrap();
    let mut hazardous_target = target.clone();
    hazardous_target.extraction_difficulty = 0;
    let mut plain_target = hazardous_target.clone();
    plain_target.hazard = None;
    let hazardous = resolve_extraction(
        7,
        15,
        &[],
        &hazardous_target,
        ModuleStats::default(),
        false,
        false,
        0,
        false,
    );
    let plain = resolve_extraction(
        7,
        15,
        &[],
        &plain_target,
        ModuleStats::default(),
        false,
        false,
        0,
        false,
    );

    assert!(hazardous.exposure > plain.exposure);
    assert_eq!(
        hazardous.exposure - plain.exposure,
        WorkspaceHazard::StructuralCollapse.exposure_modifier()
    );
}
