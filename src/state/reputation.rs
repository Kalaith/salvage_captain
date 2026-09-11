//! Persistent contract standing and transparent license benefits.

use super::GameSession;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SalvageStanding {
    Independent,
    LocalContractor,
    TrustedSalvor,
    FleetPartner,
}

impl SalvageStanding {
    pub const fn from_reputation(reputation: i32) -> Self {
        match reputation {
            0..=1 => Self::Independent,
            2..=3 => Self::LocalContractor,
            4..=6 => Self::TrustedSalvor,
            _ => Self::FleetPartner,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Independent => "INDEPENDENT",
            Self::LocalContractor => "LOCAL CONTRACTOR",
            Self::TrustedSalvor => "TRUSTED SALVOR",
            Self::FleetPartner => "FLEET PARTNER",
        }
    }

    pub const fn next_threshold(self) -> Option<i32> {
        match self {
            Self::Independent => Some(2),
            Self::LocalContractor => Some(4),
            Self::TrustedSalvor => Some(7),
            Self::FleetPartner => None,
        }
    }

    pub const fn contract_bonus_percent(self) -> i32 {
        match self {
            Self::Independent => 0,
            Self::LocalContractor => 5,
            Self::TrustedSalvor => 10,
            Self::FleetPartner => 15,
        }
    }
}

impl GameSession {
    pub fn salvage_standing(&self) -> SalvageStanding {
        SalvageStanding::from_reputation(self.reputation)
    }

    pub fn next_standing_threshold(&self) -> Option<i32> {
        self.salvage_standing().next_threshold()
    }

    pub fn contract_reward_bonus(&self, base_reward: i64) -> i64 {
        base_reward.max(0) * i64::from(self.salvage_standing().contract_bonus_percent()) / 100
    }
}

#[cfg(test)]
mod tests;
