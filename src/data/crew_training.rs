//! Data-configured costs for port-based crew certification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewTrainingTuning {
    #[serde(default = "default_base_cost")]
    pub base_cost: i64,
    #[serde(default = "default_cost_step")]
    pub cost_step: i64,
}

impl Default for CrewTrainingTuning {
    fn default() -> Self {
        Self {
            base_cost: default_base_cost(),
            cost_step: default_cost_step(),
        }
    }
}

impl CrewTrainingTuning {
    pub fn cost_for(&self, experience: u16) -> i64 {
        self.base_cost + i64::from(experience) * self.cost_step
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.base_cost <= 0 || self.cost_step < 0 {
            return Err("game_config.json: invalid crew training tuning".to_owned());
        }
        Ok(())
    }
}

fn default_base_cost() -> i64 {
    160
}

fn default_cost_step() -> i64 {
    100
}
