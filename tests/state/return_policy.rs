//! Forced-return casualty priority rules.

use super::*;
use crate::state::return_policy::select_casualty;

fn packed(object_id: &str) -> CargoItem {
    CargoItem {
        object_id: object_id.to_owned(),
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(0, 0)),
        rotation: 0,
    }
}

#[test]
fn return_policies_cycle_through_clear_priorities() {
    assert_eq!(
        ReturnPolicy::Standard.next(),
        ReturnPolicy::ProtectObjective
    );
    assert_eq!(
        ReturnPolicy::ProtectObjective.next(),
        ReturnPolicy::ProtectValue
    );
    assert_eq!(ReturnPolicy::ProtectValue.next(), ReturnPolicy::Standard);
}

#[test]
fn protect_value_changes_the_default_high_value_loss_choice() {
    let data = GameData::load().expect("game data");
    let mut cargo = vec![packed("medical_supplies"), packed("sealed_container")];
    let selected = select_casualty(
        &mut cargo,
        &data,
        ReturnPolicy::Standard,
        None,
        RiskOutcome::LostSalvage,
    )
    .expect("casualty");
    assert_eq!(selected.object_id, "sealed_container");

    let mut cargo = vec![packed("medical_supplies"), packed("sealed_container")];
    let selected = select_casualty(
        &mut cargo,
        &data,
        ReturnPolicy::ProtectValue,
        None,
        RiskOutcome::LostSalvage,
    )
    .expect("casualty");
    assert_eq!(selected.object_id, "medical_supplies");
}

#[test]
fn protect_objective_spends_another_packed_load_first() {
    let data = GameData::load().expect("game data");
    let mut cargo = vec![packed("industrial_battery"), packed("sealed_container")];
    let selected = select_casualty(
        &mut cargo,
        &data,
        ReturnPolicy::ProtectObjective,
        Some("industrial_battery"),
        RiskOutcome::ForcedAbandon,
    )
    .expect("casualty");
    assert_eq!(selected.object_id, "sealed_container");
}
