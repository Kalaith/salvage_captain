//! Persistent crew assignment rules for the captain's operating profile.

use super::GameSession;
use crate::data::GameData;
use crate::engine::RiskOutcome;

pub const CREW_EXPERIENCE_PER_LEVEL: u16 = 3;
pub const MAX_CREW_EXPERIENCE: u16 = CREW_EXPERIENCE_PER_LEVEL * 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CrewRole {
    Deckhand,
    Navigator,
    Rigger,
    SafetyOfficer,
    Broker,
}

impl Default for CrewRole {
    fn default() -> Self {
        Self::Deckhand
    }
}

impl CrewRole {
    pub const ALL: [Self; 5] = [
        Self::Deckhand,
        Self::Navigator,
        Self::Rigger,
        Self::SafetyOfficer,
        Self::Broker,
    ];

    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|role| *role == self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Deckhand => 0,
            Self::Navigator => 1,
            Self::Rigger => 2,
            Self::SafetyOfficer => 3,
            Self::Broker => 4,
        }
    }

    pub const fn expertise_label(level: u8) -> &'static str {
        match level {
            0 => "NOVICE",
            1 => "QUALIFIED",
            _ => "VETERAN",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Deckhand => "DECKHAND",
            Self::Navigator => "NAVIGATOR",
            Self::Rigger => "RIGGER",
            Self::SafetyOfficer => "SAFETY OFFICER",
            Self::Broker => "BROKER",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Deckhand => "DECKHAND",
            Self::Navigator => "NAV",
            Self::Rigger => "RIGGER",
            Self::SafetyOfficer => "SAFETY",
            Self::Broker => "BROKER",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Deckhand => "steady hands // no route adjustment",
            Self::Navigator => "-1 outbound fuel // clean approach math",
            Self::Rigger => "+1 external clamp // heavier loads come home",
            Self::SafetyOfficer => "-8 route danger // disciplined return watch",
            Self::Broker => "+5% contract reward // buyer relations",
        }
    }

    pub const fn fuel_delta(self) -> i32 {
        match self {
            Self::Navigator => -1,
            _ => 0,
        }
    }

    pub const fn danger_delta(self) -> i32 {
        match self {
            Self::SafetyOfficer => -8,
            _ => 0,
        }
    }

    pub const fn external_capacity(self) -> i32 {
        match self {
            Self::Rigger => 1,
            _ => 0,
        }
    }

    pub const fn contract_bonus_percent(self) -> i32 {
        match self {
            Self::Broker => 5,
            _ => 0,
        }
    }
}

impl GameSession {
    pub fn crew_role(&self) -> CrewRole {
        self.crew_role
    }

    pub fn cycle_crew(&mut self) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err(
                "crew assignments are available at the safe port before departure".to_owned(),
            );
        }
        self.crew_role = self.crew_role.next();
        Ok(format!(
            "Crew assigned: {}. {}.",
            self.crew_role.label(),
            self.crew_role.description()
        ))
    }

    pub fn crew_fuel_delta(&self) -> i32 {
        let expertise = match self.crew_role {
            CrewRole::Navigator => i32::from(self.crew_expertise_level()),
            _ => 0,
        };
        self.crew_role.fuel_delta() - expertise
    }

    pub fn crew_adjusted_danger(&self, danger: i32) -> i32 {
        (danger + self.crew_danger_delta()).clamp(0, 100)
    }

    pub fn crew_external_capacity(&self) -> i32 {
        let veteran_bonus = match self.crew_role {
            CrewRole::Rigger => i32::from(self.crew_expertise_level().saturating_sub(1)),
            _ => 0,
        };
        self.crew_role.external_capacity() + veteran_bonus
    }

    pub fn crew_contract_bonus_percent(&self) -> i32 {
        let expertise_bonus = match self.crew_role {
            CrewRole::Broker => i32::from(self.crew_expertise_level()) * 2,
            _ => 0,
        };
        self.crew_role.contract_bonus_percent() + expertise_bonus
    }

    pub fn crew_experience(&self) -> u16 {
        self.crew_experience_for(self.crew_role)
    }

    pub fn crew_experience_for(&self, role: CrewRole) -> u16 {
        self.career.crew_experience[role.index()].min(MAX_CREW_EXPERIENCE)
    }

    pub fn crew_expertise_level(&self) -> u8 {
        (self.crew_experience() / CREW_EXPERIENCE_PER_LEVEL).min(2) as u8
    }

    pub fn crew_expertise_label(&self) -> &'static str {
        CrewRole::expertise_label(self.crew_expertise_level())
    }

    pub fn crew_experience_gain(outcome: RiskOutcome, contract_completed: bool) -> u16 {
        let base = match outcome {
            RiskOutcome::OrdinaryReturn => 1,
            RiskOutcome::DamagedModule | RiskOutcome::LostSalvage => 2,
            RiskOutcome::EmergencyRepair => 2,
            RiskOutcome::ForcedAbandon => 3,
        };
        base + u16::from(contract_completed)
    }

    pub fn record_crew_experience(
        &mut self,
        outcome: RiskOutcome,
        contract_completed: bool,
    ) -> u16 {
        let gain = Self::crew_experience_gain(outcome, contract_completed);
        let index = self.crew_role.index();
        let before = self.career.crew_experience[index];
        self.career.crew_experience[index] = before.saturating_add(gain).min(MAX_CREW_EXPERIENCE);
        self.career.crew_experience[index].saturating_sub(before)
    }

    pub fn crew_experience_report(&self, gain: u16, previous_level: u8) -> String {
        let current_level = self.crew_expertise_level();
        if current_level > previous_level {
            format!(
                "Crew {} expertise +{} // PROMOTED {}.",
                self.crew_role.short_label(),
                gain,
                self.crew_expertise_label()
            )
        } else {
            format!(
                "Crew {} expertise +{} // {}.",
                self.crew_role.short_label(),
                gain,
                self.crew_expertise_label()
            )
        }
    }

    pub fn crew_briefing_label(&self, data: &GameData) -> String {
        let role = self.crew_role;
        if role != CrewRole::Rigger {
            return format!(
                "CREW {} // {} // READY {}%",
                role.short_label(),
                self.crew_expertise_label(),
                self.crew_readiness()
            );
        }
        let capacity = self.external_capacity(data);
        format!(
            "CREW {} // {} // CLAMPS {} // READY {}%",
            role.short_label(),
            self.crew_expertise_label(),
            capacity,
            self.crew_readiness()
        )
    }
}

#[cfg(test)]
mod tests;
