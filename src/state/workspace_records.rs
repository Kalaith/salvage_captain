//! Serialized field records and target intelligence retained between runs.

use crate::data::SalvageObjectData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceLogEvent {
    Departed,
    EnteredSection,
    SectionScanned,
    SectionCleared,
    DronesDeployed,
    PowerCycled,
    TargetStabilized,
    ExtractionStarted,
    ExtractionCancelled,
    TargetRecovered,
    TargetLost,
}

impl WorkspaceLogEvent {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Departed => "DEPARTED",
            Self::EnteredSection => "ENTERED",
            Self::SectionScanned => "SCANNED",
            Self::SectionCleared => "SECTION CLEARED",
            Self::DronesDeployed => "DRONES DEPLOYED",
            Self::PowerCycled => "POWER CYCLED",
            Self::TargetStabilized => "STABILIZED",
            Self::ExtractionStarted => "PULL STARTED",
            Self::ExtractionCancelled => "PULL CANCELLED",
            Self::TargetRecovered => "RECOVERED",
            Self::TargetLost => "LOST",
        }
    }

    pub const fn is_target_event(self) -> bool {
        matches!(
            self,
            Self::TargetStabilized
                | Self::ExtractionStarted
                | Self::ExtractionCancelled
                | Self::TargetRecovered
                | Self::TargetLost
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLogEntry {
    pub sequence: u32,
    pub event: WorkspaceLogEvent,
    #[serde(default)]
    pub section_id: String,
    #[serde(default)]
    pub target_id: Option<String>,
}

impl WorkspaceLogEntry {
    pub fn new(
        sequence: u32,
        event: WorkspaceLogEvent,
        section_id: Option<&str>,
        target_id: Option<&str>,
    ) -> Self {
        Self {
            sequence,
            event,
            section_id: section_id.unwrap_or_default().to_owned(),
            target_id: target_id.map(str::to_owned),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetSurveyNote {
    pub target_id: String,
    pub section_id: String,
    pub scan_count: u32,
    pub hazard: Option<String>,
    pub transfer_mode: String,
    pub integrity: i32,
    pub extraction_difficulty: i32,
    pub mass_tons: f32,
}

impl TargetSurveyNote {
    pub fn from_target(target_id: &str, section_id: &str, target: &SalvageObjectData) -> Self {
        Self {
            target_id: target_id.to_owned(),
            section_id: section_id.to_owned(),
            scan_count: 1,
            hazard: target.hazard.clone(),
            transfer_mode: target.transfer_mode.clone(),
            integrity: target.integrity,
            extraction_difficulty: target.extraction_difficulty,
            mass_tons: target.mass_tons,
        }
    }
}
