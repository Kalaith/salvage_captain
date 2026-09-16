//! Session-owned wreck snapshots and an immutable resolved-content view.

use super::{GameSession, SiteProgress};
use crate::data::{GameData, SalvageObjectData, SiteData};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WreckInstance {
    pub serial: u64,
    pub template_id: String,
    pub seed: u64,
    pub site: SiteData,
    pub targets: Vec<SalvageInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalvageInstance {
    pub template_id: String,
    pub section_index: usize,
    pub slot: usize,
    pub object: SalvageObjectData,
}

pub fn wreck_id(template_id: &str, serial: u64) -> String {
    format!("wreck:{template_id}:{serial:08}")
}

pub fn item_id(wreck_id: &str, section: usize, slot: usize) -> String {
    format!("{wreck_id}/item:{section}:{slot}")
}

impl SiteProgress {
    pub fn fresh(condition: i32) -> Self {
        Self {
            condition,
            visits: 0,
            discovered_sections: Vec::new(),
            removed_targets: Vec::new(),
            operation_log: Vec::new(),
            surveyed_targets: Vec::new(),
            contract_completed: false,
            contract_failed: false,
            reconnaissance_level: 0,
            cleared_sections: Vec::new(),
        }
    }
}

impl GameData {
    /// Clear only the derived registry entries. Authored templates never change.
    pub fn clear_wreck_instances(&mut self) {
        let sites: Vec<_> = self
            .sites
            .ids()
            .filter(|id| id.starts_with("wreck:"))
            .cloned()
            .collect();
        let targets: Vec<_> = self
            .salvage_objects
            .ids()
            .filter(|id| id.starts_with("wreck:"))
            .cloned()
            .collect();
        for id in sites {
            self.sites.remove(&id);
        }
        for id in targets {
            self.salvage_objects.remove(&id);
        }
    }

    pub fn with_wreck_instances(&self, wrecks: &[WreckInstance]) -> Result<Self, String> {
        let mut resolved = self.clone();
        resolved.clear_wreck_instances();
        let mut serials = HashSet::new();
        for wreck in wrecks {
            if wreck.serial == 0
                || !serials.insert(wreck.serial)
                || wreck.site.id != wreck_id(&wreck.template_id, wreck.serial)
                || !resolved.sites.contains(&wreck.template_id)
                || wreck.template_id.starts_with("wreck:")
                || !(0..=100).contains(&wreck.site.condition)
            {
                return Err("save contains an invalid or duplicate wreck identity".to_owned());
            }
            let mut target_ids = HashSet::new();
            for target in &wreck.targets {
                let section = wreck
                    .site
                    .sections
                    .get(target.section_index)
                    .ok_or_else(|| "save target references an unknown section".to_owned())?;
                if target.slot >= 4
                    || target.object.id
                        != item_id(&wreck.site.id, target.section_index, target.slot)
                    || !target_ids.insert(target.object.id.clone())
                    || section.candidate_targets.get(target.slot) != Some(&target.object.id)
                    || target.template_id.starts_with("wreck:")
                    || !resolved.salvage_objects.contains(&target.template_id)
                {
                    return Err("save contains an invalid physical salvage identity".to_owned());
                }
                resolved
                    .salvage_objects
                    .insert(target.object.id.clone(), target.object.clone());
            }
            let roster: HashSet<_> = wreck
                .site
                .sections
                .iter()
                .flat_map(|section| section.candidate_targets.iter().cloned())
                .collect();
            let roster_count: usize = wreck
                .site
                .sections
                .iter()
                .map(|s| s.candidate_targets.len())
                .sum();
            if roster != target_ids || roster_count != target_ids.len() {
                return Err("save wreck roster does not match its physical salvage".to_owned());
            }
            resolved
                .sites
                .insert(wreck.site.id.clone(), wreck.site.clone());
        }
        resolved.validate()?;
        Ok(resolved)
    }
}

impl GameSession {
    pub fn resolved_data(&self, data: &GameData) -> Result<GameData, String> {
        if self.wrecks.iter().any(|wreck| {
            wreck.serial > self.discovery_serial || !self.site_progress.contains_key(&wreck.site.id)
        }) {
            return Err("save is missing wreck progress or its discovery sequence".to_owned());
        }
        data.with_wreck_instances(&self.wrecks)
    }
}
