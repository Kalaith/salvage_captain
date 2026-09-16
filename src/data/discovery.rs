//! Authored generation rules; runtime wreck snapshots live in the session.

use crate::data::GameData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryTuning {
    pub active_limit: usize,
    pub specialist_cost: i64,
    pub condition_min: i32,
    pub condition_max: i32,
    pub danger_variation: i32,
    pub integrity_variation: i32,
    pub reward_base: i64,
    pub reward_per_danger: i64,
    pub reward_per_difficulty: i64,
    pub contract_brief: String,
    pub name_pattern: String,
    pub pools: Vec<WreckPool>,
    pub copy: DiscoveryCopy,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WreckPool {
    pub template_id: String,
    pub minimum_reputation: i32,
    pub specialist: bool,
    pub minimum_targets: usize,
    pub maximum_targets: usize,
    pub minimum_accessible: usize,
    pub maximum_accessible: usize,
    pub loot: Vec<LootWeight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LootWeight {
    pub object_id: String,
    pub weight: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryCopy {
    pub find: String,
    pub specialist: String,
    pub active: String,
    pub archive: String,
    pub previous: String,
    pub next: String,
    pub unexplored: String,
    pub partial: String,
    pub depleted: String,
    pub empty: String,
    pub archive_empty: String,
    pub tutorial: String,
    pub busy: String,
    pub full: String,
    pub locked: String,
    pub unaffordable: String,
    pub found: String,
    pub requirements: String,
    pub starter_ready: String,
    pub depleted_departure: String,
    pub gear_ready: String,
    pub wasted_trip: String,
    pub readiness_hint: String,
    pub contract_blocked: String,
    pub no_access_hint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeadKind {
    Local,
    Specialist,
}

impl DiscoveryTuning {
    pub fn validate(&self, data: &GameData) -> Result<(), String> {
        if !(3..=12).contains(&self.active_limit)
            || self.specialist_cost <= 0
            || self.condition_min < 40
            || self.condition_max > 100
            || self.condition_min > self.condition_max
            || !(0..=20).contains(&self.danger_variation)
            || !(0..=30).contains(&self.integrity_variation)
            || self.reward_base <= 0
            || self.reward_per_danger < 0
            || self.reward_per_difficulty < 0
        {
            return Err("discovery.json: invalid generation limits or rewards".to_owned());
        }
        if !self
            .pools
            .iter()
            .any(|pool| !pool.specialist && pool.minimum_reputation == 0)
        {
            return Err("discovery.json: a free starter pool is required".to_owned());
        }
        let mut ids = std::collections::HashSet::new();
        for pool in &self.pools {
            if !ids.insert(&pool.template_id)
                || !data.sites.contains(&pool.template_id)
                || pool.minimum_reputation < 0
                || pool.minimum_targets < 4
                || pool.maximum_targets > 8
                || pool.minimum_targets > pool.maximum_targets
                || pool.minimum_accessible == 0
                || pool.minimum_accessible > pool.maximum_accessible
                || pool.maximum_accessible > pool.minimum_targets
                || pool.loot.is_empty()
                || pool.loot.iter().any(|entry| {
                    entry.weight == 0
                        || entry.weight > 100
                        || !data.salvage_objects.contains(&entry.object_id)
                })
            {
                return Err(format!(
                    "discovery.json: invalid pool '{}'",
                    pool.template_id
                ));
            }
            let site = data
                .sites
                .get(&pool.template_id)
                .expect("validated pool template");
            let open_slots = site
                .sections
                .iter()
                .enumerate()
                .filter(|(_, section)| section.required_capability.is_none())
                .map(|(index, _)| {
                    pool.minimum_targets / 2 + usize::from(index < pool.minimum_targets % 2)
                })
                .sum::<usize>();
            if site.sections.len() != 2
                || site.sections[0].required_capability.is_some()
                || open_slots < pool.maximum_accessible
                || !pool.loot.iter().any(|entry| {
                    data.salvage_objects
                        .get(&entry.object_id)
                        .is_some_and(|target| !starter_equipment_target(target, data))
                })
            {
                return Err(format!(
                    "discovery.json: '{}' needs open mounts and upgrade salvage",
                    pool.template_id
                ));
            }
            if !pool.loot.iter().any(|entry| {
                data.salvage_objects
                    .get(&entry.object_id)
                    .is_some_and(|target| {
                        starter_target(target) && starter_equipment_target(target, data)
                    })
            }) {
                return Err(format!(
                    "discovery.json: '{}' needs an accessible contract target",
                    pool.template_id
                ));
            }
        }
        Ok(())
    }
}

pub const TRACTOR_TONS_PER_POWER: i32 = 4;

/// Content classification against the authored starter fit, independent of the captain's upgrades.
pub fn starter_equipment_target(target: &crate::data::SalvageObjectData, data: &GameData) -> bool {
    let modules: Vec<_> = data
        .config
        .starting_modules
        .iter()
        .filter_map(|starting| data.modules.get(&starting.module_id))
        .collect();
    let power: i32 = modules
        .iter()
        .map(|module| match module.effect {
            crate::data::ModuleEffect::Power(value) => value,
            _ => 0,
        })
        .sum();
    target.mass_tons <= (power * TRACTOR_TONS_PER_POWER) as f32
        && target
            .required_capability
            .as_deref()
            .is_none_or(|required| {
                modules
                    .iter()
                    .any(|module| module.capability.as_deref() == Some(required))
            })
}

pub fn starter_target(target: &crate::data::SalvageObjectData) -> bool {
    target.required_capability.is_none()
        && target.hazard.is_none()
        && target.transfer_mode == "internal_cargo"
        && target.footprint.width <= 2
        && target.footprint.height <= 2
}
