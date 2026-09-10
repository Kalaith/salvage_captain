//! Site-selection screen identity.

use super::{GameData, GameSession};
use std::collections::HashSet;

pub const TITLE: &str = "SELECT A WRECK";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiteRecoveryStatus {
    pub explored_sections: usize,
    pub total_sections: usize,
    pub recovered_targets: usize,
    pub total_targets: usize,
    pub remaining_targets: usize,
    pub exploration_percent: i32,
}

impl GameSession {
    pub fn site_recovery_summary(&self, site_id: &str, data: &GameData) -> (usize, usize) {
        let status = self.site_recovery_status(site_id, data);
        (status.recovered_targets, status.total_targets)
    }

    pub fn site_recovery_status(&self, site_id: &str, data: &GameData) -> SiteRecoveryStatus {
        let Some(site) = data.sites.get(site_id) else {
            return SiteRecoveryStatus {
                explored_sections: 0,
                total_sections: 0,
                recovered_targets: 0,
                total_targets: 0,
                remaining_targets: 0,
                exploration_percent: 0,
            };
        };
        let targets: HashSet<&str> = site
            .sections
            .iter()
            .flat_map(|section| section.candidate_targets.iter().map(String::as_str))
            .collect();
        let progress = self.site_progress.get(site_id);
        let explored_sections = progress.map_or(0, |value| {
            value
                .discovered_sections
                .iter()
                .filter(|section_id| {
                    site.sections
                        .iter()
                        .any(|section| section.id == **section_id)
                })
                .count()
        });
        let recovered_targets = progress.map_or(0, |value| {
            value
                .removed_targets
                .iter()
                .filter(|target| targets.contains(target.as_str()))
                .count()
        });
        let exploration_percent = if site.sections.is_empty() {
            0
        } else {
            (explored_sections * 100 / site.sections.len()) as i32
        };
        SiteRecoveryStatus {
            explored_sections,
            total_sections: site.sections.len(),
            recovered_targets,
            total_targets: targets.len(),
            remaining_targets: targets.len().saturating_sub(recovered_targets),
            exploration_percent,
        }
    }
}
