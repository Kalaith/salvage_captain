//! Deterministic hazard resolution for close-range salvage operations.

use crate::data::SalvageObjectData;
use crate::engine::ModuleStats;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceOutcome {
    Recovered,
    DamagedHull,
    LostTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRiskReport {
    pub outcome: WorkspaceOutcome,
    pub exposure: i32,
    pub mitigation: i32,
    pub roll: i32,
    pub explanation: String,
}

pub fn resolve_extraction(
    seed: u64,
    site_danger: i32,
    section_hazards: &[String],
    target: &SalvageObjectData,
    stats: ModuleStats,
    has_stabilizer: bool,
    has_scanner_array: bool,
    drone_support: i32,
) -> WorkspaceRiskReport {
    let section_load = (section_hazards.len() as i32 * 7).min(28);
    let extraction_load = if target.hazard.is_some() {
        target.extraction_difficulty / 3
    } else {
        target.extraction_difficulty / 8
    };
    let exposure = (site_danger / 4 + section_load + extraction_load).clamp(0, 100);
    let mitigation = (stats.shielding
        + stats.scanning / 3
        + if has_stabilizer { 24 } else { 0 }
        + if has_scanner_array { 10 } else { 0 }
        + drone_support.clamp(0, 3) * 8)
        .min(100);
    let adjusted_exposure = (exposure - mitigation).clamp(0, 100);
    let roll = stable_roll(seed, &target.id);
    let outcome = if adjusted_exposure <= 20 || roll >= adjusted_exposure {
        WorkspaceOutcome::Recovered
    } else if target.hazard.as_deref() == Some("structural_collapse") || roll % 3 == 0 {
        WorkspaceOutcome::LostTarget
    } else {
        WorkspaceOutcome::DamagedHull
    };
    let explanation = match outcome {
        WorkspaceOutcome::Recovered => {
            "The hazard stayed inside the predicted envelope. Pull is clean.".to_owned()
        }
        WorkspaceOutcome::DamagedHull => {
            "The mount slipped under load. The target is aboard, but the hull took a hit."
                .to_owned()
        }
        WorkspaceOutcome::LostTarget => {
            "The mount failed during separation. The target is gone in the wreckage.".to_owned()
        }
    };
    WorkspaceRiskReport {
        outcome,
        exposure: adjusted_exposure,
        mitigation,
        roll,
        explanation,
    }
}

pub fn exposure_label(exposure: i32) -> &'static str {
    match exposure {
        0..=20 => "STABLE",
        21..=45 => "CAUTION",
        _ => "HIGH RISK",
    }
}

fn stable_roll(seed: u64, target_id: &str) -> i32 {
    let mixed = target_id
        .bytes()
        .fold(seed ^ 0xA5A5_5A5A_1F2E_3D4C, |value, byte| {
            value.rotate_left(7) ^ u64::from(byte).wrapping_mul(0x9E37)
        });
    (mixed.wrapping_mul(0x9E37_79B9_7F4A_7C15) % 100) as i32
}

#[cfg(test)]
mod tests;
