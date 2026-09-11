//! Persistent crew assignment rules for the captain's operating profile.

use super::GameSession;
use crate::data::GameData;

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
        self.crew_role.fuel_delta()
    }

    pub fn crew_adjusted_danger(&self, danger: i32) -> i32 {
        (danger + self.crew_role.danger_delta()).clamp(0, 100)
    }

    pub fn crew_external_capacity(&self) -> i32 {
        self.crew_role.external_capacity()
    }

    pub fn crew_contract_bonus_percent(&self) -> i32 {
        self.crew_role.contract_bonus_percent()
    }

    pub fn crew_briefing_label(&self, data: &GameData) -> String {
        let role = self.crew_role;
        if role != CrewRole::Rigger {
            return format!("CREW {} // {}", role.short_label(), role.description());
        }
        let capacity = self.external_capacity(data);
        format!(
            "CREW {} // {} // CLAMPS {}",
            role.short_label(),
            role.description(),
            capacity
        )
    }
}

#[cfg(test)]
mod tests;
