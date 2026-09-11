//! Deterministic hazard resolution for close-range salvage operations.

use crate::data::SalvageObjectData;
use crate::engine::ModuleStats;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceOutcome {
    Recovered,
    DamagedHull,
    LostTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceHazard {
    ReactorInstability,
    ElectricalArcs,
    AutomatedDefenses,
    UnexplodedAmmunition,
    MagneticInterference,
    StructuralCollapse,
}

impl WorkspaceHazard {
    pub fn from_value(value: &str) -> Option<Self> {
        match value {
            "reactor_instability" => Some(Self::ReactorInstability),
            "electrical_arcs" => Some(Self::ElectricalArcs),
            "automated_defenses" => Some(Self::AutomatedDefenses),
            "unexploded_ammunition" => Some(Self::UnexplodedAmmunition),
            "magnetic_interference" => Some(Self::MagneticInterference),
            "structural_collapse" => Some(Self::StructuralCollapse),
            _ => None,
        }
    }

    pub const fn response_label(self) -> &'static str {
        match self {
            Self::ReactorInstability => "THERMAL SPIKE",
            Self::ElectricalArcs => "ARC FLASH",
            Self::AutomatedDefenses => "DEFENSE WAKE",
            Self::UnexplodedAmmunition => "ORDNANCE SHIFT",
            Self::MagneticInterference => "MAGNETIC DRIFT",
            Self::StructuralCollapse => "HULL COLLAPSE",
        }
    }

    pub const fn exposure_modifier(self) -> i32 {
        match self {
            Self::ReactorInstability => 8,
            Self::ElectricalArcs => 5,
            Self::AutomatedDefenses => 10,
            Self::UnexplodedAmmunition => 6,
            Self::MagneticInterference => 7,
            Self::StructuralCollapse => 14,
        }
    }

    pub const fn outcome_detail(self, outcome: WorkspaceOutcome) -> &'static str {
        match (self, outcome) {
            (Self::ReactorInstability, WorkspaceOutcome::Recovered) => {
                "Reactor heat stayed inside containment."
            }
            (Self::ReactorInstability, WorkspaceOutcome::DamagedHull) => {
                "Reactor heat spiked through the transfer line."
            }
            (Self::ReactorInstability, WorkspaceOutcome::LostTarget) => {
                "The hot core broke loose in the collapsing mount."
            }
            (Self::ElectricalArcs, WorkspaceOutcome::Recovered) => {
                "Arc flash stayed clear of the intake."
            }
            (Self::ElectricalArcs, WorkspaceOutcome::DamagedHull) => {
                "An arc flash hit the transfer line."
            }
            (Self::ElectricalArcs, WorkspaceOutcome::LostTarget) => {
                "Electrical arcs swallowed the target."
            }
            (Self::AutomatedDefenses, WorkspaceOutcome::Recovered) => {
                "Defense systems stayed asleep."
            }
            (Self::AutomatedDefenses, WorkspaceOutcome::DamagedHull) => {
                "A defense mount fired into the workboat."
            }
            (Self::AutomatedDefenses, WorkspaceOutcome::LostTarget) => {
                "A defense lock severed the target."
            }
            (Self::UnexplodedAmmunition, WorkspaceOutcome::Recovered) => "Ordnance remained cold.",
            (Self::UnexplodedAmmunition, WorkspaceOutcome::DamagedHull) => {
                "A shifting round struck the hull."
            }
            (Self::UnexplodedAmmunition, WorkspaceOutcome::LostTarget) => {
                "The crate slipped into the wreck before the clamp could seal."
            }
            (Self::MagneticInterference, WorkspaceOutcome::Recovered) => {
                "Magnetic drift stayed inside the forecast."
            }
            (Self::MagneticInterference, WorkspaceOutcome::DamagedHull) => {
                "Magnetic drift scrambled a ship system."
            }
            (Self::MagneticInterference, WorkspaceOutcome::LostTarget) => {
                "The field threw the target back into the wreck."
            }
            (Self::StructuralCollapse, WorkspaceOutcome::Recovered) => {
                "The frame held around the mount."
            }
            (Self::StructuralCollapse, WorkspaceOutcome::DamagedHull) => {
                "A collapsing frame tore into the hull."
            }
            (Self::StructuralCollapse, WorkspaceOutcome::LostTarget) => {
                "The mount vanished in the collapse."
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRiskReport {
    pub outcome: WorkspaceOutcome,
    pub hazard: Option<WorkspaceHazard>,
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
    stabilized: bool,
) -> WorkspaceRiskReport {
    let hazard = target
        .hazard
        .as_deref()
        .and_then(WorkspaceHazard::from_value);
    let section_load = (section_hazards.len() as i32 * 7).min(28);
    let extraction_load = if target.hazard.is_some() {
        target.extraction_difficulty / 3
    } else {
        target.extraction_difficulty / 8
    };
    let hazard_load = hazard.map_or(0, WorkspaceHazard::exposure_modifier);
    let exposure = (site_danger / 4 + section_load + extraction_load + hazard_load).clamp(0, 100);
    let mitigation = (stats.shielding
        + stats.scanning / 3
        + if has_stabilizer { 24 } else { 0 }
        + if has_scanner_array { 10 } else { 0 }
        + drone_support.clamp(0, 3) * 8
        + if stabilized { 20 } else { 0 })
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
    let explanation = if let Some(hazard) = hazard {
        format!("{explanation} {}", hazard.outcome_detail(outcome.clone()))
    } else {
        explanation
    };
    WorkspaceRiskReport {
        outcome,
        hazard,
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
