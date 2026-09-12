//! Workspace extraction risk preview with route-plan adjustments.

use super::super::GameSession;
use crate::data::GameData;
use crate::engine::{
    danger_after_intel, resolve_extraction, ExtractionRequest, VoyagePlan, WorkspaceRiskReport,
};

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
        )
        .saturating_sub(self.route_familiarity_danger_reduction(&site.id));
        let voyage_plan = self
            .expedition
            .as_ref()
            .map_or(VoyagePlan::Standard, |expedition| expedition.voyage_plan);
        Ok(resolve_extraction(ExtractionRequest {
            seed: self
                .expedition
                .as_ref()
                .map_or(7, |expedition| expedition.seed),
            site_danger: voyage_plan.adjust_danger(departure_danger, &data.config.voyage_plan),
            section_hazards: &section.hazard_tags,
            target,
            stats,
            has_stabilizer: self.has_capability("stabilizer", data),
            has_scanner_array: self.has_capability("scanner_array", data),
            drone_support: self.workspace_drone_support(data),
            stabilized: self.target_is_stabilized(target_id),
        }))
    }
}
