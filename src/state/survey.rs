//! Persistent target intelligence gathered during workspace scans.

use super::{GameSession, TargetSurveyNote};
use crate::data::GameData;

impl GameSession {
    pub fn workspace_survey_note<'a>(
        &'a self,
        target_id: &str,
        data: &GameData,
    ) -> Option<&'a TargetSurveyNote> {
        let expedition = self.expedition.as_ref()?;
        let section = data
            .sites
            .get(&expedition.site_id)?
            .sections
            .iter()
            .find(|section| section.id == expedition.workspace_section)?;
        self.site_progress
            .get(&expedition.site_id)?
            .surveyed_targets
            .iter()
            .find(|note| note.target_id == target_id && note.section_id == section.id)
    }

    pub fn site_survey_count(&self, site_id: &str) -> usize {
        self.site_progress
            .get(site_id)
            .map_or(0, |progress| progress.surveyed_targets.len())
    }

    pub fn section_survey_count(&self, site_id: &str, section_id: &str) -> usize {
        self.site_progress.get(site_id).map_or(0, |progress| {
            progress
                .surveyed_targets
                .iter()
                .filter(|note| note.section_id == section_id)
                .count()
        })
    }

    pub(crate) fn record_survey_notes(
        &mut self,
        site_id: &str,
        section_id: &str,
        target_ids: &[String],
        data: &GameData,
    ) {
        let Some(progress) = self.site_progress.get_mut(site_id) else {
            return;
        };
        for target_id in target_ids {
            let Some(target) = data.salvage_objects.get(target_id) else {
                continue;
            };
            if let Some(note) = progress
                .surveyed_targets
                .iter_mut()
                .find(|note| note.target_id == *target_id && note.section_id == section_id)
            {
                note.scan_count = note.scan_count.saturating_add(1);
            } else {
                progress
                    .surveyed_targets
                    .push(TargetSurveyNote::from_target(target_id, section_id, target));
            }
        }
    }
}
