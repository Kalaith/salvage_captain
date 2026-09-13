//! Persistent, renewable salvage opportunities through the public game API.

use salvage_captain::data::discovery::{LeadKind, LootWeight};
use salvage_captain::data::GameData;
use salvage_captain::engine::{discovery::generate_wreck, Disposition};
use salvage_captain::state::{GameSession, SaveData};
use salvage_captain::ui::scene_layout::salvage_layout;

fn local() -> (GameData, GameSession, String) {
    let mut data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let id = session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    (data, session, id)
}

fn deplete(session: &mut GameSession, data: &GameData, id: &str) {
    session.site_progress.get_mut(id).unwrap().removed_targets =
        data.sites.get(id).unwrap().candidate_salvage.clone();
}

#[test]
fn generation_is_reproducible_but_varies_across_discoveries() {
    let data = GameData::load().unwrap();
    let pool = &data.discovery.pools[0];
    let first = generate_wreck(pool, 1, 123, &data).unwrap();
    let repeated = generate_wreck(pool, 1, 123, &data).unwrap();
    let other = generate_wreck(pool, 2, 321, &data).unwrap();
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(&repeated).unwrap()
    );
    assert_ne!(first.site.id, other.site.id);
    assert_ne!(
        first
            .targets
            .iter()
            .map(|t| (&t.template_id, t.object.integrity))
            .collect::<Vec<_>>(),
        other
            .targets
            .iter()
            .map(|t| (&t.template_id, t.object.integrity))
            .collect::<Vec<_>>()
    );
}

#[test]
fn every_generated_contract_exists_in_the_accessible_entry_section() {
    let data = GameData::load().unwrap();
    for pool in &data.discovery.pools {
        for seed in 0..50 {
            let wreck = generate_wreck(pool, seed + 1, seed, &data).unwrap();
            let resolved = data
                .with_wreck_instances(std::slice::from_ref(&wreck))
                .unwrap();
            let objective = wreck.site.contract_target.as_ref().unwrap();
            assert!(wreck.site.sections[0].required_capability.is_none());
            assert!(wreck.site.sections[0].candidate_targets.contains(objective));
            let target = resolved.salvage_objects.get(objective).unwrap();
            assert!(salvage_captain::data::discovery::starter_target(target));
            assert!(
                wreck.site.contract_reward
                    > i64::from(wreck.site.fuel_cost * data.config.refuel_price_per_unit)
            );
        }
    }
}

#[test]
fn generated_targets_have_distinct_touch_mounts_even_when_types_repeat() {
    let (data, session, _) = local();
    for section in &session.wrecks[0].site.sections {
        let mounts: Vec<_> = section
            .candidate_targets
            .iter()
            .map(|id| salvage_layout().target_rect(id).unwrap())
            .collect();
        for (index, mount) in mounts.iter().enumerate() {
            assert!(mount.w >= 44.0 && mount.h >= 44.0);
            for other in &mounts[index + 1..] {
                assert!(!mount.overlaps(other));
            }
        }
    }
    assert_eq!(data.sites.len(), 4);
}

#[test]
fn duplicate_batteries_survive_independent_recovery_packing_and_sale() {
    let mut data = GameData::load().unwrap();
    data.discovery.pools[0].loot = vec![LootWeight {
        object_id: "industrial_battery".to_owned(),
        weight: 1,
    }];
    let mut session = GameSession::new(&data);
    let id = session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    let section = data.sites.get(&id).unwrap().sections[0].clone();
    session.begin_expedition(&id, &data).unwrap();
    session.scan_workspace(&data).unwrap();
    for target in section.candidate_targets.iter().take(2) {
        let object = data.salvage_objects.get(target).unwrap();
        let (position, _) = session
            .ship_layout
            .first_fit(&format!("cargo:{target}"), object.footprint, false)
            .unwrap();
        session
            .begin_workspace_transfer(target, position, 0, &data)
            .unwrap();
        session.recover_workspace_target(target, &data).unwrap();
    }
    assert_eq!(session.expedition.as_ref().unwrap().cargo.len(), 2);
    let untouched = data.sites.get(&id).unwrap().sections[1].candidate_targets[0].clone();
    assert!(!session.site_progress[&id]
        .removed_targets
        .contains(&untouched));
    session.finish_packing(&data).unwrap();
    assert_eq!(session.returned.len(), 2);
    assert!(session.site_progress[&id].contract_completed);
    let first = session.returned[0].object_id.clone();
    let second = session.returned[1].object_id.clone();
    session.dispose(&first, Disposition::Sell, &data).unwrap();
    assert_eq!(session.returned[0].object_id, second);
    session.dispose(&second, Disposition::Sell, &data).unwrap();
    assert!(session.returned.is_empty());
}

#[test]
fn revisiting_does_not_regenerate_loot_or_restore_removed_items() {
    let (data, mut session, id) = local();
    let target = data
        .sites
        .get(&id)
        .unwrap()
        .contract_target
        .clone()
        .unwrap();
    let snapshot = serde_json::to_value(&session.wrecks).unwrap();
    session
        .site_progress
        .get_mut(&id)
        .unwrap()
        .removed_targets
        .push(target.clone());
    session.begin_expedition(&id, &data).unwrap();
    session.scan_workspace(&data).unwrap();
    assert!(!session.target_is_revealed(&target));
    assert_eq!(serde_json::to_value(&session.wrecks).unwrap(), snapshot);
}

#[test]
fn free_leads_work_without_credits_and_specialists_check_standing_and_cost() {
    let mut data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.credits = 0;
    assert!(session.discover_wreck(LeadKind::Local, &mut data).is_ok());
    assert!(session
        .discover_wreck(LeadKind::Specialist, &mut data)
        .is_err());
    session.reputation = 2;
    assert!(session
        .discover_wreck(LeadKind::Specialist, &mut data)
        .is_err());
    let serial = session.discovery_serial;
    session.economy.credits = data.discovery.specialist_cost;
    let id = session
        .discover_wreck(LeadKind::Specialist, &mut data)
        .unwrap();
    assert_eq!(session.discovery_serial, serial + 1);
    assert_eq!(session.economy.credits, 0);
    assert_eq!(data.sites.get(&id).unwrap().visual_theme, "military");
}

#[test]
fn depleted_wrecks_archive_without_erasing_progress_and_can_be_replaced() {
    let (mut data, mut session, id) = local();
    deplete(&mut session, &data, &id);
    assert!(session
        .listed_wrecks(&data, true)
        .iter()
        .any(|s| s.id == id));
    assert!(!session
        .listed_wrecks(&data, false)
        .iter()
        .any(|s| s.id == id));
    let new_id = session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    assert_ne!(new_id, id);
    assert!(!session.site_progress[&id].removed_targets.is_empty());
    assert!(session.site_progress[&new_id].removed_targets.is_empty());
}

#[test]
fn discovery_is_transactional_when_board_full_or_during_a_voyage() {
    let (mut data, mut session, id) = local();
    session.begin_expedition(&id, &data).unwrap();
    let before = serde_json::to_value(&session).unwrap();
    assert!(session.discover_wreck(LeadKind::Local, &mut data).is_err());
    assert_eq!(serde_json::to_value(&session).unwrap(), before);
    session.finish_packing(&data).unwrap();
    while session.listed_wrecks(&data, false).len() < data.discovery.active_limit {
        session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    }
    let before = serde_json::to_value(&session).unwrap();
    assert!(session.discover_wreck(LeadKind::Local, &mut data).is_err());
    assert_eq!(serde_json::to_value(&session).unwrap(), before);
}

#[test]
fn replenishment_preserves_unfinished_wrecks_and_adds_a_local_contract() {
    let (mut data, mut session, id) = local();
    assert!(session.replenish_wrecks(&mut data).unwrap().is_none());
    let merchant = data
        .sites
        .get("merchant_wreck")
        .unwrap()
        .contract_target
        .clone()
        .unwrap();
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .removed_targets
        .push(merchant);
    let target = data
        .sites
        .get(&id)
        .unwrap()
        .contract_target
        .clone()
        .unwrap();
    session
        .site_progress
        .get_mut(&id)
        .unwrap()
        .removed_targets
        .push(target);
    let fresh = session.replenish_wrecks(&mut data).unwrap().unwrap();
    assert!(session.site_recovery_status(&id, &data).remaining_targets > 0);
    assert_ne!(fresh, id);
    assert!(session.replenish_wrecks(&mut data).unwrap().is_none());
}

#[test]
fn save_load_restores_exact_contents_and_the_next_discovery() {
    let (mut data, mut session, _) = local();
    let fresh_data = GameData::load().unwrap();
    let save: SaveData = serde_json::from_value(
        serde_json::to_value(session.to_save(&data.config.version)).unwrap(),
    )
    .unwrap();
    let mut restored = GameSession::from_save(save, &fresh_data).unwrap();
    let mut restored_data = restored.resolved_data(&fresh_data).unwrap();
    assert_eq!(
        serde_json::to_value(&restored.wrecks).unwrap(),
        serde_json::to_value(&session.wrecks).unwrap()
    );
    assert_eq!(
        session.discover_wreck(LeadKind::Local, &mut data).unwrap(),
        restored
            .discover_wreck(LeadKind::Local, &mut restored_data)
            .unwrap()
    );
    assert_eq!(
        serde_json::to_value(&restored.wrecks).unwrap(),
        serde_json::to_value(&session.wrecks).unwrap()
    );
}

#[test]
fn legacy_save_keeps_starter_progress_and_gains_discovery_defaults() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .removed_targets
        .push("industrial_battery".to_owned());
    let mut value = serde_json::to_value(session.to_save("2.40.0")).unwrap();
    for field in ["wrecks", "discovery_seed", "discovery_serial"] {
        value["session"].as_object_mut().unwrap().remove(field);
    }
    let save: SaveData = serde_json::from_value(value).unwrap();
    let restored = GameSession::from_save(save, &data).unwrap();
    assert_eq!(restored.site_progress, session.site_progress);
    assert_eq!(restored.discovery_serial, 0);
    assert!(restored.wrecks.is_empty());
}

#[test]
fn saves_reject_duplicate_wrecks_invalid_item_ids_and_missing_progress() {
    let (data, session, id) = local();
    let mut duplicate = session.clone();
    duplicate.wrecks.push(duplicate.wrecks[0].clone());
    assert!(GameSession::from_save(duplicate.to_save(&data.config.version), &data).is_err());
    let mut invalid = session.clone();
    invalid.wrecks[0].targets[0].object.id = "industrial_battery".to_owned();
    assert!(GameSession::from_save(invalid.to_save(&data.config.version), &data).is_err());
    let mut missing = session.clone();
    missing.site_progress.remove(&id);
    assert!(GameSession::from_save(missing.to_save(&data.config.version), &data).is_err());
}

#[test]
fn resolving_a_different_save_drops_previous_dynamic_registry_entries() {
    let (data, session, id) = local();
    let fresh = GameData::load().unwrap();
    let clean_session = GameSession::new(&fresh);
    let clean = clean_session.resolved_data(&data).unwrap();
    assert!(!clean.sites.contains(&id));
    assert!(session.wrecks[0]
        .targets
        .iter()
        .all(|t| !clean.salvage_objects.contains(&t.object.id)));
    assert_eq!(clean.sites.len(), fresh.sites.len());
}
