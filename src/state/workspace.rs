//! Authoritative workspace discovery, capability gates, and wreck condition changes.

use super::{
    CargoItem, CargoStatus, GameSession, WorkspaceLogEntry, WorkspaceLogEvent, WorkspaceScanProfile,
};
use crate::data::{GameData, SalvageObjectData, WreckSectionData};
use crate::state::DroneDirective;

pub use super::workspace_condition::WorkspaceConditionStatus;

mod operations;
mod risk;
mod transfer;

#[derive(Debug, Clone)]
pub struct WorkspaceTransfer {
    pub target_id: String,
    pub position: crate::data::GridPosition,
    pub rotation: u8,
}

pub const WORKSPACE_STABILIZATION_ENERGY_COST: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractionPhase {
    Alignment,
    Connection,
    Strain,
    Separation,
    Retrieval,
    Capture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    InternalCargo,
    ExternalClamp,
    Tow,
}

impl TransferMode {
    pub fn from_value(value: &str) -> Self {
        match value {
            "external_clamp" => Self::ExternalClamp,
            "tow" => Self::Tow,
            _ => Self::InternalCargo,
        }
    }

    pub fn from_target(target: &SalvageObjectData) -> Self {
        Self::from_value(&target.transfer_mode)
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "CARGO",
            Self::ExternalClamp => "CLAMP",
            Self::Tow => "TOW",
        }
    }

    pub const fn uses_external_rig(self) -> bool {
        !matches!(self, Self::InternalCargo)
    }

    pub const fn destination_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "SALVAGE HOLD",
            Self::ExternalClamp => "EXTERNAL CLAMP",
            Self::Tow => "TOW RIG",
        }
    }

    pub const fn command_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "LOAD CARGO",
            Self::ExternalClamp => "LOCK CLAMP",
            Self::Tow => "ENGAGE TOW",
        }
    }

    pub const fn cancel_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "CANCEL CARGO",
            Self::ExternalClamp => "CANCEL CLAMP",
            Self::Tow => "CANCEL TOW",
        }
    }

    pub const fn engaged_message(self) -> &'static str {
        match self {
            Self::InternalCargo => "Cargo intake engaged.",
            Self::ExternalClamp => "External clamp engaged.",
            Self::Tow => "Tow rig engaged.",
        }
    }

    pub const fn phase_label(self, phase: ExtractionPhase) -> &'static str {
        match (self, phase) {
            (Self::InternalCargo, ExtractionPhase::Alignment) => "ALIGNING INTAKE",
            (Self::InternalCargo, ExtractionPhase::Connection) => "CARGO INTAKE LIVE",
            (Self::InternalCargo, ExtractionPhase::Strain) => "LOADING UNDER STRAIN",
            (Self::InternalCargo, ExtractionPhase::Separation) => "BREAKING CARGO FREE",
            (Self::InternalCargo, ExtractionPhase::Retrieval) => "RETRIEVING TO HOLD",
            (Self::InternalCargo, ExtractionPhase::Capture) => "CARGO LOCKED",
            (Self::ExternalClamp, ExtractionPhase::Alignment) => "ALIGNING CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Connection) => "CLAMP CONTACT",
            (Self::ExternalClamp, ExtractionPhase::Strain) => "CLAMP UNDER LOAD",
            (Self::ExternalClamp, ExtractionPhase::Separation) => "SEATING CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Retrieval) => "RETRIEVING TO CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Capture) => "CLAMP SEALED",
            (Self::Tow, ExtractionPhase::Alignment) => "ALIGNING TOW HEAD",
            (Self::Tow, ExtractionPhase::Connection) => "TOW LINE CONNECTED",
            (Self::Tow, ExtractionPhase::Strain) => "TOW LINE UNDER LOAD",
            (Self::Tow, ExtractionPhase::Separation) => "BREAKING TOW FREE",
            (Self::Tow, ExtractionPhase::Retrieval) => "RETRIEVING TO RIG",
            (Self::Tow, ExtractionPhase::Capture) => "TOW COUPLED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtractionRuntime {
    pub target_id: String,
    pub elapsed: f32,
    pub duration: f32,
    pub resolved: bool,
}

impl ExtractionRuntime {
    pub fn new(target_id: impl Into<String>, duration: f32) -> Self {
        Self {
            target_id: target_id.into(),
            elapsed: 0.0,
            duration: duration.max(1.0),
            resolved: false,
        }
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    pub fn phase(&self) -> ExtractionPhase {
        match self.progress() {
            value if value < 0.12 => ExtractionPhase::Alignment,
            value if value < 0.28 => ExtractionPhase::Connection,
            value if value < 0.55 => ExtractionPhase::Strain,
            value if value < 0.68 => ExtractionPhase::Separation,
            value if value < 0.92 => ExtractionPhase::Retrieval,
            _ => ExtractionPhase::Capture,
        }
    }
}

impl GameSession {
    pub fn workspace_condition_status(
        &self,
        data: &GameData,
    ) -> Result<WorkspaceConditionStatus, String> {
        let site = self.workspace_site(data)?;
        let section = self.workspace_section(data)?;
        self.site_section_condition_status(&site.id, &section.id, data)
            .ok_or_else(|| format!("unknown wreck section '{}'", section.id))
    }

    pub fn site_section_condition_status(
        &self,
        site_id: &str,
        section_id: &str,
        data: &GameData,
    ) -> Option<WorkspaceConditionStatus> {
        let site = data.sites.get(site_id)?;
        let section = site
            .sections
            .iter()
            .find(|section| section.id == section_id)?;
        let frame_condition = self
            .site_progress
            .get(site_id)
            .map_or(site.condition, |progress| progress.condition)
            .clamp(0, 100);
        let (recovered_targets, discovered) =
            self.site_progress
                .get(site_id)
                .map_or((0, false), |progress| {
                    (
                        section
                            .candidate_targets
                            .iter()
                            .filter(|target_id| {
                                progress
                                    .removed_targets
                                    .iter()
                                    .any(|removed_id| removed_id == *target_id)
                            })
                            .count(),
                        progress
                            .discovered_sections
                            .iter()
                            .any(|section_id| section_id == &section.id),
                    )
                });
        let section_condition = (frame_condition - recovered_targets as i32 * 8).max(0);
        Some(WorkspaceConditionStatus {
            frame_condition,
            section_condition,
            recovered_targets,
            total_targets: section.candidate_targets.len(),
            discovered,
        })
    }

    pub fn workspace_energy(&self) -> Option<(i32, i32)> {
        self.expedition.as_ref().map(|expedition| {
            (
                expedition.workspace_energy,
                expedition.workspace_energy_capacity,
            )
        })
    }

    pub fn workspace_site<'a>(
        &self,
        data: &'a GameData,
    ) -> Result<&'a crate::data::SiteData, String> {
        let site_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.site_id.as_str())
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        data.sites
            .get(site_id)
            .ok_or_else(|| format!("unknown salvage site '{site_id}'"))
    }

    pub fn workspace_section<'a>(
        &self,
        data: &'a GameData,
    ) -> Result<&'a WreckSectionData, String> {
        let site = self.workspace_site(data)?;
        let section_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.workspace_section.as_str())
            .unwrap_or_default();
        site.sections
            .iter()
            .find(|section| section.id == section_id)
            .or_else(|| site.sections.first())
            .ok_or_else(|| format!("site '{}' has no workspace sections", site.id))
    }

    pub fn workspace_log(&self) -> Option<&[WorkspaceLogEntry]> {
        let site_id = self.expedition.as_ref()?.site_id.as_str();
        self.site_progress
            .get(site_id)
            .map(|progress| progress.operation_log.as_slice())
    }

    pub fn record_workspace_event(&mut self, event: WorkspaceLogEvent, target_id: Option<&str>) {
        let Some(expedition) = self.expedition.as_ref() else {
            return;
        };
        let site_id = expedition.site_id.clone();
        let section_id = expedition.workspace_section.clone();
        self.append_workspace_log(&site_id, event, Some(section_id.as_str()), target_id);
    }
}

fn workspace_name(target: &SalvageObjectData) -> &str {
    if target.workspace_name.is_empty() {
        target.display_name.as_str()
    } else {
        target.workspace_name.as_str()
    }
}

fn capability_label(value: &str) -> String {
    value
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
