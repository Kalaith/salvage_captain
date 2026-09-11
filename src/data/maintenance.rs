//! Data-driven tuning for persistent ship wear and safe-port servicing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTuning {
    #[serde(default = "default_base_wear")]
    pub base_wear_per_voyage: i32,
    #[serde(default = "default_external_load_wear")]
    pub external_load_wear: i32,
    #[serde(default = "default_power_cycle_wear")]
    pub power_cycle_wear: i32,
    #[serde(default = "default_setback_wear")]
    pub setback_wear: i32,
    #[serde(default = "default_price_per_wear")]
    pub price_per_wear: i64,
    #[serde(default = "default_danger_per_wear_band")]
    pub danger_per_wear_band: i32,
}

impl Default for MaintenanceTuning {
    fn default() -> Self {
        Self {
            base_wear_per_voyage: default_base_wear(),
            external_load_wear: default_external_load_wear(),
            power_cycle_wear: default_power_cycle_wear(),
            setback_wear: default_setback_wear(),
            price_per_wear: default_price_per_wear(),
            danger_per_wear_band: default_danger_per_wear_band(),
        }
    }
}

impl MaintenanceTuning {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_wear_per_voyage < 0
            || self.external_load_wear < 0
            || self.power_cycle_wear < 0
            || self.setback_wear < 0
            || self.price_per_wear < 0
            || self.danger_per_wear_band < 0
        {
            return Err("game_config.json: maintenance tuning cannot be negative".to_owned());
        }
        Ok(())
    }
}

fn default_base_wear() -> i32 {
    4
}

fn default_external_load_wear() -> i32 {
    2
}

fn default_power_cycle_wear() -> i32 {
    5
}

fn default_setback_wear() -> i32 {
    8
}

fn default_price_per_wear() -> i64 {
    6
}

fn default_danger_per_wear_band() -> i32 {
    4
}
