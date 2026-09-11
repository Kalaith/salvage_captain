//! Persistent crew readiness: hard runs leave the hands tired until shore leave.

use super::GameSession;
use crate::engine::RiskOutcome;

pub const MAX_CREW_FATIGUE: u8 = 100;

pub fn fatigue_gain(outcome: RiskOutcome, external_load: i32, power_cycles_used: u8) -> u8 {
    let outcome_load = match outcome {
        RiskOutcome::OrdinaryReturn => 8,
        RiskOutcome::DamagedModule => 14,
        RiskOutcome::LostSalvage | RiskOutcome::EmergencyRepair => 18,
        RiskOutcome::ForcedAbandon => 24,
    };
    let cargo_load = external_load.max(0).min(5) * 2;
    let power_load = i32::from(power_cycles_used.min(1)) * 3;
    (outcome_load + cargo_load + power_load).clamp(0, i32::from(MAX_CREW_FATIGUE)) as u8
}

impl GameSession {
    pub fn crew_fatigue(&self) -> u8 {
        self.crew_fatigue.min(MAX_CREW_FATIGUE)
    }

    pub fn crew_readiness(&self) -> u8 {
        MAX_CREW_FATIGUE - self.crew_fatigue()
    }

    pub fn crew_fatigue_danger_delta(&self) -> i32 {
        i32::from(self.crew_fatigue()) / 10
    }

    pub fn crew_danger_delta(&self) -> i32 {
        self.crew_role().danger_delta() + self.crew_fatigue_danger_delta()
    }

    pub fn register_crew_fatigue(
        &mut self,
        outcome: RiskOutcome,
        external_load: i32,
        power_cycles_used: u8,
    ) -> u8 {
        let gain = fatigue_gain(outcome, external_load, power_cycles_used);
        self.crew_fatigue = self
            .crew_fatigue()
            .saturating_add(gain)
            .min(MAX_CREW_FATIGUE);
        gain
    }

    pub fn rest_crew(&mut self) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("the crew can take shore leave only at the safe port".to_owned());
        }
        if self.crew_fatigue() == 0 {
            return Ok("Crew already rested. Readiness 100%.".to_owned());
        }
        self.crew_fatigue = 0;
        Ok("Crew rested at the safe port. Readiness 100%; route watch restored.".to_owned())
    }
}

#[cfg(test)]
mod tests;
