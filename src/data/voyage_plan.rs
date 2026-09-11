//! Data-driven fuel and danger adjustments for selectable voyage plans.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VoyagePlanTuning {
    #[serde(default = "default_cautious_fuel_delta")]
    pub cautious_fuel_delta: i32,
    #[serde(default = "default_cautious_danger_delta")]
    pub cautious_danger_delta: i32,
    #[serde(default = "default_expedited_fuel_delta")]
    pub expedited_fuel_delta: i32,
    #[serde(default = "default_expedited_danger_delta")]
    pub expedited_danger_delta: i32,
}

impl Default for VoyagePlanTuning {
    fn default() -> Self {
        Self {
            cautious_fuel_delta: default_cautious_fuel_delta(),
            cautious_danger_delta: default_cautious_danger_delta(),
            expedited_fuel_delta: default_expedited_fuel_delta(),
            expedited_danger_delta: default_expedited_danger_delta(),
        }
    }
}

fn default_cautious_fuel_delta() -> i32 {
    1
}

fn default_cautious_danger_delta() -> i32 {
    -10
}

fn default_expedited_fuel_delta() -> i32 {
    -1
}

fn default_expedited_danger_delta() -> i32 {
    10
}
