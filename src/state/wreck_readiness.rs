//! Preflight salvage access using the same fitted-equipment gates as extraction.

use super::GameSession;
use crate::data::{GameData, SalvageObjectData, SiteData};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WreckReadiness {
    pub accessible: usize,
    pub remaining: usize,
    pub contract_accessible: bool,
}

impl WreckReadiness {
    pub fn label(self, data: &GameData) -> String {
        let copy = &data.discovery.copy;
        if self.remaining == 0 {
            copy.depleted.clone()
        } else if self.accessible == 0 {
            copy.wasted_trip.clone()
        } else {
            copy.gear_ready
                .replace("{accessible}", &self.accessible.to_string())
                .replace("{remaining}", &self.remaining.to_string())
        }
    }
}

impl GameSession {
    pub fn target_equipment_block_reason(
        &self,
        target: &SalvageObjectData,
        data: &GameData,
    ) -> Option<String> {
        if let Some(required) = &target.required_capability {
            if !self.has_capability(required, data) {
                return Some(format!(
                    "Requires {} capability.",
                    super::workspace::capability_label(required)
                ));
            }
        }
        if target.mass_tons > self.tractor_capacity_tons(data) {
            return Some(format!(
                "Tractor capacity insufficient: {:.0} / {:.0}t.",
                self.tractor_capacity_tons(data),
                target.mass_tons
            ));
        }
        None
    }

    /// Counts individual equipment-compatible pulls, not a promise of a full haul.
    pub fn wreck_readiness(&self, site: &SiteData, data: &GameData) -> WreckReadiness {
        let mut reachable = HashSet::new();
        let mut pending: Vec<_> = site
            .sections
            .first()
            .map(|s| s.id.as_str())
            .into_iter()
            .collect();
        while let Some(id) = pending.pop() {
            let Some(section) = site.sections.iter().find(|s| s.id == id) else {
                continue;
            };
            if reachable.contains(id)
                || section
                    .required_capability
                    .as_deref()
                    .is_some_and(|capability| !self.has_capability(capability, data))
            {
                continue;
            }
            reachable.insert(id);
            pending.extend(section.connected_sections.iter().map(String::as_str));
        }
        let progress = self.site_progress.get(&site.id);
        let mut remaining = HashSet::new();
        let mut accessible = HashSet::new();
        for section in &site.sections {
            for id in &section.candidate_targets {
                if progress.is_some_and(|p| p.removed_targets.contains(id)) {
                    continue;
                }
                remaining.insert(id);
                if reachable.contains(section.id.as_str())
                    && data.salvage_objects.get(id).is_some_and(|target| {
                        self.target_equipment_block_reason(target, data).is_none()
                    })
                {
                    accessible.insert(id);
                }
            }
        }
        WreckReadiness {
            accessible: accessible.len(),
            remaining: remaining.len(),
            contract_accessible: site
                .contract_target
                .as_ref()
                .is_some_and(|id| accessible.contains(id)),
        }
    }
}
