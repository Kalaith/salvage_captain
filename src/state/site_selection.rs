//! Site-selection screen identity.

use super::{GameData, GameSession};
use std::collections::HashSet;

pub const TITLE: &str = "SELECT A WRECK";

impl GameSession {
    pub fn site_recovery_summary(&self, site_id: &str, data: &GameData) -> (usize, usize) {
        let Some(site) = data.sites.get(site_id) else {
            return (0, 0);
        };
        let targets: HashSet<&str> = site
            .sections
            .iter()
            .flat_map(|section| section.candidate_targets.iter().map(String::as_str))
            .collect();
        let recovered = self.site_progress.get(site_id).map_or(0, |progress| {
            progress
                .removed_targets
                .iter()
                .filter(|target| targets.contains(target.as_str()))
                .count()
        });
        (recovered, targets.len())
    }
}
