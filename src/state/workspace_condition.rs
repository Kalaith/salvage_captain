//! Persistent wreck-frame condition derived from saved salvage progress.

use serde::{Deserialize, Serialize};

/// Persistent condition as seen from the currently selected wreck section.
///
/// The frame condition comes from the saved site progress. Section condition
/// also accounts for targets already removed from this frame, so revisiting a
/// partially salvaged wreck has a stable, data-backed visual state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceConditionStatus {
    pub frame_condition: i32,
    pub section_condition: i32,
    pub recovered_targets: usize,
    pub total_targets: usize,
    pub discovered: bool,
}

impl WorkspaceConditionStatus {
    pub fn unknown() -> Self {
        Self {
            frame_condition: 0,
            section_condition: 0,
            recovered_targets: 0,
            total_targets: 0,
            discovered: false,
        }
    }

    pub fn label(self) -> &'static str {
        if !self.discovered {
            "UNMAPPED"
        } else if self.section_condition <= 30 {
            "CRITICAL"
        } else if self.recovered_targets > 0 {
            "STRESSED"
        } else {
            "STABLE"
        }
    }

    pub fn structural_stress(self) -> i32 {
        (100 - self.section_condition).clamp(0, 100)
    }
}
