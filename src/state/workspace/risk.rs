//! Workspace extraction risk preview with route-plan adjustments.

use super::super::GameSession;
use crate::data::GameData;
use crate::engine::{danger_after_intel, resolve_extraction, VoyagePlan, WorkspaceRiskReport};

impl GameSession {
    pub fn workspace_risk_preview(
        &self,
        target_id: &str,
        data: &GameData,
    ) -> Result<WorkspaceRiskReport, String> {
        let site = self.workspace_site(data)?;
        let section = self.workspace_section(data)?;
        let target = self.workspace_target(target_id, data)?;
        let stats = self.module_stats(data);
        let departure_danger = danger_after_intel(
            site.danger,
            self.reconnaissance_level(&site.id),
            &data.config.reconnaissance,
        );
        let voyage_plan = self
            .expedition
            .as_ref()
            .map_or(VoyagePlan::Standard, |expedition| expedition.voyage_plan);
        Ok(resolve_extraction(
            self.expedition
                .as_ref()
                .map_or(7, |expedition| expedition.seed),
            voyage_plan.adjust_danger(departure_danger, &data.config.voyage_plan),
            &section.hazard_tags,
            target,
            stats,
            self.has_capability("stabilizer", data),
            self.has_capability("scanner_array", data),
            stats.drone_support,
            self.target_is_stabilized(target_id),
        ))
    }
}
