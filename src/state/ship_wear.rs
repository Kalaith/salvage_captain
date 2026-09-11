//! Persistent operational wear that turns hard salvage runs into service decisions.

use super::GameSession;
use crate::data::{GameData, MaintenanceTuning};
use crate::engine::RiskOutcome;

pub const MAX_SHIP_WEAR: u8 = 100;

pub fn wear_gain(
    outcome: RiskOutcome,
    external_load: i32,
    power_cycles_used: u8,
    tuning: &MaintenanceTuning,
) -> u8 {
    let setback_multiplier = match outcome {
        RiskOutcome::OrdinaryReturn => 0,
        RiskOutcome::DamagedModule => 1,
        RiskOutcome::LostSalvage | RiskOutcome::EmergencyRepair => 2,
        RiskOutcome::ForcedAbandon => 3,
    };
    let gain = i64::from(tuning.base_wear_per_voyage)
        + i64::from(setback_multiplier) * i64::from(tuning.setback_wear)
        + i64::from(external_load.max(0)) * i64::from(tuning.external_load_wear)
        + i64::from(power_cycles_used.min(1)) * i64::from(tuning.power_cycle_wear);
    gain.clamp(0, i64::from(MAX_SHIP_WEAR)) as u8
}

impl GameSession {
    pub fn ship_wear(&self) -> u8 {
        self.ship_wear.min(MAX_SHIP_WEAR)
    }

    pub fn maintenance_danger_delta(&self, data: &GameData) -> i32 {
        (i32::from(self.ship_wear()) / 20) * data.config.maintenance.danger_per_wear_band
    }

    pub fn maintenance_adjusted_danger(&self, danger: i32, data: &GameData) -> i32 {
        (danger + self.maintenance_danger_delta(data)).clamp(0, 100)
    }

    pub fn maintenance_cost(&self, data: &GameData) -> i64 {
        i64::from(self.ship_wear()) * data.config.maintenance.price_per_wear
    }

    pub fn register_ship_wear(
        &mut self,
        outcome: RiskOutcome,
        external_load: i32,
        power_cycles_used: u8,
        tuning: &MaintenanceTuning,
    ) -> u8 {
        let gain = wear_gain(outcome, external_load, power_cycles_used, tuning);
        self.ship_wear = self.ship_wear().saturating_add(gain).min(MAX_SHIP_WEAR);
        gain
    }
}

#[cfg(test)]
mod tests;
