//! One-time port bounties for completely recovered wreck sections.

use super::{GameData, GameSession, SiteProgress, WorkspaceLogEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionClearanceStatus {
    pub section_id: String,
    pub recovered_targets: usize,
    pub total_targets: usize,
    pub reward: i64,
    pub cleared: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SectionClearanceReport {
    pub section_ids: Vec<String>,
    pub payout: i64,
}

impl SectionClearanceReport {
    pub fn message(&self, data: &GameData) -> String {
        if self.section_ids.is_empty() {
            return String::new();
        }
        let names = self
            .section_ids
            .iter()
            .filter_map(|section_id| {
                data.sites
                    .iter()
                    .flat_map(|(_, site)| site.sections.iter())
                    .find(|section| section.id == *section_id)
                    .map(|section| section.display_name.as_str())
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            " Section clearance filed: {names}. Bounty +{} credits.",
            self.payout
        )
    }
}

impl GameSession {
    pub fn section_clearance_status(
        &self,
        site_id: &str,
        section_id: &str,
        data: &GameData,
    ) -> Option<SectionClearanceStatus> {
        let site = data.sites.get(site_id)?;
        let section = site
            .sections
            .iter()
            .find(|section| section.id == section_id)?;
        let progress = self.site_progress.get(site_id);
        let recovered_targets = progress.map_or(0, |progress| {
            section
                .candidate_targets
                .iter()
                .filter(|target_id| target_recovered_in_section(progress, section_id, target_id))
                .count()
        });
        let cleared = progress.is_some_and(|progress| {
            progress
                .cleared_sections
                .iter()
                .any(|cleared_id| cleared_id == section_id)
        });
        let ready = !cleared
            && !section.candidate_targets.is_empty()
            && recovered_targets == section.candidate_targets.len();
        Some(SectionClearanceStatus {
            section_id: section_id.to_owned(),
            recovered_targets,
            total_targets: section.candidate_targets.len(),
            reward: section.clearance_reward,
            cleared,
            ready,
        })
    }

    pub fn site_clearance_summary(&self, site_id: &str, data: &GameData) -> (usize, usize) {
        let Some(site) = data.sites.get(site_id) else {
            return (0, 0);
        };
        let cleared = self.site_progress.get(site_id).map_or(0, |progress| {
            site.sections
                .iter()
                .filter(|section| {
                    progress
                        .cleared_sections
                        .iter()
                        .any(|cleared_id| cleared_id == &section.id)
                })
                .count()
        });
        (cleared, site.sections.len())
    }

    pub fn site_clearance_rewards(&self, site_id: &str, data: &GameData) -> (i64, i64) {
        let Some(site) = data.sites.get(site_id) else {
            return (0, 0);
        };
        let cleared_sections = self
            .site_progress
            .get(site_id)
            .map_or(&[] as &[String], |progress| {
                progress.cleared_sections.as_slice()
            });
        site.sections
            .iter()
            .fold((0, 0), |(paid, remaining), section| {
                if cleared_sections.iter().any(|id| id == &section.id) {
                    (paid + section.clearance_reward, remaining)
                } else {
                    (paid, remaining + section.clearance_reward)
                }
            })
    }

    pub fn site_clearance_ready_summary(&self, site_id: &str, data: &GameData) -> (usize, i64) {
        let Some(site) = data.sites.get(site_id) else {
            return (0, 0);
        };
        site.sections
            .iter()
            .fold((0, 0), |(count, payout), section| {
                let Some(status) = self.section_clearance_status(site_id, &section.id, data) else {
                    return (count, payout);
                };
                if status.ready {
                    (count + 1, payout + status.reward)
                } else {
                    (count, payout)
                }
            })
    }

    pub(crate) fn resolve_section_clearance(
        &mut self,
        site_id: &str,
        data: &GameData,
    ) -> SectionClearanceReport {
        let Some(site) = data.sites.get(site_id) else {
            return SectionClearanceReport::default();
        };
        let ready_sections: Vec<_> = site
            .sections
            .iter()
            .filter(|section| {
                self.site_progress.get(site_id).is_some_and(|progress| {
                    !progress
                        .cleared_sections
                        .iter()
                        .any(|cleared_id| cleared_id == &section.id)
                        && !section.candidate_targets.is_empty()
                        && section.candidate_targets.iter().all(|target_id| {
                            target_recovered_in_section(progress, &section.id, target_id)
                        })
                })
            })
            .map(|section| (section.id.clone(), section.clearance_reward))
            .collect();
        if ready_sections.is_empty() {
            return SectionClearanceReport::default();
        }
        let report = SectionClearanceReport {
            section_ids: ready_sections
                .iter()
                .map(|(section_id, _)| section_id.clone())
                .collect(),
            payout: ready_sections.iter().map(|(_, reward)| *reward).sum(),
        };
        if let Some(progress) = self.site_progress.get_mut(site_id) {
            progress
                .cleared_sections
                .extend(report.section_ids.iter().cloned());
        }
        for section_id in &report.section_ids {
            self.append_workspace_log(
                site_id,
                WorkspaceLogEvent::SectionCleared,
                Some(section_id.as_str()),
                None,
            );
        }
        self.economy.credits += report.payout;
        report
    }
}

fn target_recovered_in_section(progress: &SiteProgress, section_id: &str, target_id: &str) -> bool {
    progress.operation_log.iter().any(|entry| {
        entry.event == WorkspaceLogEvent::TargetRecovered
            && entry.section_id == section_id
            && entry.target_id.as_deref() == Some(target_id)
    })
}

#[cfg(test)]
mod tests;
