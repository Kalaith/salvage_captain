//! Recovery budgets, fitted-gear previews and renewable early-game work.

use salvage_captain::data::{discovery::LeadKind, GameData};
use salvage_captain::engine::discovery::generate_wreck;
use salvage_captain::state::GameSession;
use salvage_captain::ui::scene_layout::salvage_layout;

#[test]
fn every_class_keeps_its_starter_budget_across_seeds_and_distinct_mounts() {
    let data = GameData::load().unwrap();
    for (index, pool) in data.discovery.pools.iter().enumerate() {
        let (total, accessible) = [(4..=6, 2..=4), (5..=6, 2..=3), (6..=8, 1..=2)][index].clone();
        let mut totals = std::collections::HashSet::new();
        let mut counts = std::collections::HashSet::new();
        for seed in 0..150 {
            let wreck = generate_wreck(pool, seed + 1, seed, &data).unwrap();
            let resolved = data
                .with_wreck_instances(std::slice::from_ref(&wreck))
                .unwrap();
            let mut session = GameSession::new(&resolved);
            let ready = session.wreck_readiness(&wreck.site, &resolved);
            assert!(
                total.contains(&ready.remaining),
                "{index}/{seed}: {ready:?}"
            );
            assert!(
                accessible.contains(&ready.accessible),
                "{index}/{seed}: {ready:?}"
            );
            totals.insert(ready.remaining);
            counts.insert(ready.accessible);
            session.economy.fuel = 100;
            session.begin_expedition(&wreck.site.id, &resolved).unwrap();
            let mut actual = 0;
            for section in &wreck.site.sections {
                let mounts: Vec<_> = section
                    .candidate_targets
                    .iter()
                    .map(|id| salvage_layout().target_rect(id).unwrap())
                    .collect();
                for (i, mount) in mounts.iter().enumerate() {
                    assert!(mount.w >= 44.0 && mount.h >= 44.0);
                    assert!(mounts
                        .iter()
                        .skip(i + 1)
                        .all(|other| !mount.overlaps(other)));
                }
                if session
                    .switch_workspace_section(&section.id, &resolved)
                    .is_err()
                {
                    continue;
                }
                session.scan_workspace(&resolved).unwrap();
                actual += section
                    .candidate_targets
                    .iter()
                    .filter(|id| {
                        session
                            .extraction_block_reason(id, &resolved)
                            .unwrap()
                            .is_none()
                    })
                    .count();
            }
            assert_eq!(actual, ready.accessible);
        }
        assert_eq!(totals.len(), total.count());
        assert_eq!(counts.len(), accessible.count());
    }
}

#[test]
fn fitted_upgrades_open_saved_salvage_and_damage_closes_the_same_gates() {
    let data = GameData::load().unwrap();
    for pool in &data.discovery.pools {
        let wreck = generate_wreck(pool, 1, 7, &data).unwrap();
        let resolved = data
            .with_wreck_instances(std::slice::from_ref(&wreck))
            .unwrap();
        let mut session = GameSession::new(&resolved);
        let starter = session.wreck_readiness(&wreck.site, &resolved);
        session.economy.credits = 10000;
        session.refresh_module_unlocks(&resolved);
        let mut previous = starter.accessible;
        for module in ["reactor_module", "shield_module", "scanner_module"] {
            session.purchase_module(module, &resolved).unwrap();
            let ready = session.wreck_readiness(&wreck.site, &resolved);
            assert!(ready.accessible >= previous);
            previous = ready.accessible;
        }
        assert_eq!(previous, starter.remaining);
        session.damaged_modules = vec![
            "reactor_module".into(),
            "shield_module".into(),
            "scanner_module".into(),
        ];
        assert_eq!(session.wreck_readiness(&wreck.site, &resolved), starter);
    }
}

#[test]
fn warnings_track_remaining_salvage_and_survive_reload_without_rerolling() {
    let mut data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let id = session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    let site = data.sites.get(&id).unwrap();
    session.site_progress.get_mut(&id).unwrap().removed_targets = site
        .candidate_salvage
        .iter()
        .filter(|id| {
            data.salvage_objects
                .get(id)
                .unwrap()
                .required_capability
                .is_none()
        })
        .cloned()
        .collect();
    let ready = session.wreck_readiness(site, &data);
    assert_eq!(ready.accessible, 0);
    assert!(ready.remaining > 0);
    assert!(!ready.contract_accessible);
    assert_eq!(ready.label(&data), data.discovery.copy.wasted_trip);
    // The warning informs a deliberate visit; it is not a departure prohibition.
    assert!(session.can_depart(&id, &data));
    let snapshot = serde_json::to_value(&session.wrecks).unwrap();
    let fresh = GameData::load().unwrap();
    let mut restored =
        GameSession::from_save(session.to_save(&data.config.version), &fresh).unwrap();
    let resolved = restored.resolved_data(&fresh).unwrap();
    assert_eq!(serde_json::to_value(&restored.wrecks).unwrap(), snapshot);
    assert_eq!(
        restored.wreck_readiness(resolved.sites.get(&id).unwrap(), &resolved),
        ready
    );
    restored.site_progress.get_mut(&id).unwrap().removed_targets = site.candidate_salvage.clone();
    assert_eq!(
        restored.wreck_readiness(site, &resolved).label(&resolved),
        resolved.discovery.copy.depleted
    );
}

#[test]
fn upgrade_only_wrecks_cannot_crowd_out_free_work_on_a_full_board() {
    let mut data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    while session.listed_wrecks(&data, false).len() < data.discovery.active_limit {
        session.discover_wreck(LeadKind::Local, &mut data).unwrap();
    }
    for site in session.listed_wrecks(&data, false) {
        if site.visual_theme != "merchant" {
            continue;
        }
        session
            .site_progress
            .get_mut(&site.id)
            .unwrap()
            .removed_targets = site
            .candidate_salvage
            .iter()
            .filter(|id| {
                data.salvage_objects
                    .get(id)
                    .unwrap()
                    .required_capability
                    .is_none()
            })
            .cloned()
            .collect();
    }
    let old = serde_json::to_value(&session.wrecks).unwrap();
    session.economy.credits = 0;
    let id = session.replenish_wrecks(&mut data).unwrap().unwrap();
    assert!(
        session
            .wreck_readiness(data.sites.get(&id).unwrap(), &data)
            .accessible
            >= 2
    );
    assert_eq!(
        session.listed_wrecks(&data, false).len(),
        data.discovery.active_limit + 1
    );
    assert_eq!(
        serde_json::to_value(&session.wrecks[..session.wrecks.len() - 1]).unwrap(),
        old
    );
    assert!(session.replenish_wrecks(&mut data).unwrap().is_none());
    assert!(session.discover_wreck(LeadKind::Local, &mut data).is_err());
}

#[test]
fn invalid_budgets_and_ineligible_loot_are_rejected_before_discovery() {
    let data = GameData::load().unwrap();
    for case in 0..5 {
        let mut broken = data.clone();
        let pool = &mut broken.discovery.pools[0];
        match case {
            0 => pool.maximum_targets = 9,
            1 => pool.minimum_accessible = 0,
            2 => pool.maximum_accessible = 7,
            3 => pool
                .loot
                .retain(|entry| entry.object_id == "industrial_battery"),
            _ => pool
                .loot
                .retain(|entry| entry.object_id == "damaged_reactor"),
        }
        assert!(broken.discovery.validate(&broken).is_err());
    }
}
