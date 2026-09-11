//! Operator-selected drone behavior for the active salvage workspace.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DroneDirective {
    Standby,
    Survey,
    PullSupport,
}

impl Default for DroneDirective {
    fn default() -> Self {
        Self::PullSupport
    }
}

impl DroneDirective {
    pub const fn next(self) -> Self {
        match self {
            Self::Standby => Self::Survey,
            Self::Survey => Self::PullSupport,
            Self::PullSupport => Self::Standby,
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Standby => "STANDBY",
            Self::Survey => "SURVEY",
            Self::PullSupport => "PULL",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standby => "DRONES STANDBY",
            Self::Survey => "SURVEY NET",
            Self::PullSupport => "PULL SUPPORT",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Standby => "Drones recalled; the workboat carries the full field load.",
            Self::Survey => "Drones widen the hazard read and trade speed for safer pulls.",
            Self::PullSupport => "Drones brace the line and shorten active extraction time.",
        }
    }

    pub const fn deploys_drones(self) -> bool {
        !matches!(self, Self::Standby)
    }

    pub const fn extraction_reduction(self) -> f32 {
        match self {
            Self::PullSupport => 0.12,
            Self::Standby | Self::Survey => 0.0,
        }
    }

    pub fn effective_support(self, module_support: i32) -> i32 {
        match self {
            Self::Standby => 0,
            Self::Survey => (module_support + 1).clamp(0, 3),
            Self::PullSupport => module_support.clamp(0, 3),
        }
    }
}

#[cfg(test)]
mod tests;
