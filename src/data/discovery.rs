//! Authored generation rules; runtime wreck snapshots live in the session.

use crate::data::GameData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryTuning {
    pub active_limit: usize,
    pub specialist_cost: i64,
    pub minimum_targets: usize,
    pub maximum_targets: usize,
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
            || self.minimum_targets == 0
            || self.maximum_targets > 3
            || self.minimum_targets > self.maximum_targets
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
            if !pool.loot.iter().any(|entry| {
                data.salvage_objects
                    .get(&entry.object_id)
                    .is_some_and(starter_target)
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

pub fn starter_target(target: &crate::data::SalvageObjectData) -> bool {
    target.required_capability.is_none()
        && target.hazard.is_none()
        && target.transfer_mode == "internal_cargo"
        && target.footprint.width <= 2
        && target.footprint.height <= 2
}
