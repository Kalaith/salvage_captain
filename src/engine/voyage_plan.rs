//! Pure rules for selecting a fuel-versus-danger operating plan.

use crate::data::VoyagePlanTuning;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoyagePlan {
    Cautious,
    Standard,
    Expedited,
}

impl Default for VoyagePlan {
    fn default() -> Self {
        Self::Standard
    }
}

impl VoyagePlan {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cautious => "CAUTIOUS",
            Self::Standard => "STANDARD",
            Self::Expedited => "EXPEDITED",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Cautious => "extra fuel // lower route danger",
            Self::Standard => "normal fuel // balanced exposure",
            Self::Expedited => "save fuel // higher route danger",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Cautious => Self::Standard,
            Self::Standard => Self::Expedited,
            Self::Expedited => Self::Cautious,
        }
    }

    pub const fn fuel_delta(self, tuning: &VoyagePlanTuning) -> i32 {
        match self {
            Self::Cautious => tuning.cautious_fuel_delta,
            Self::Standard => 0,
            Self::Expedited => tuning.expedited_fuel_delta,
        }
    }

    pub const fn danger_delta(self, tuning: &VoyagePlanTuning) -> i32 {
        match self {
            Self::Cautious => tuning.cautious_danger_delta,
            Self::Standard => 0,
            Self::Expedited => tuning.expedited_danger_delta,
        }
    }

    pub fn adjust_fuel(self, base_cost: i32, tuning: &VoyagePlanTuning) -> i32 {
        (base_cost + self.fuel_delta(tuning)).max(1)
    }

    pub fn adjust_danger(self, base_danger: i32, tuning: &VoyagePlanTuning) -> i32 {
        (base_danger + self.danger_delta(tuning)).clamp(0, 100)
    }
}

#[cfg(test)]
mod tests;
