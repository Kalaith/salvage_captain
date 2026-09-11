//! Player-selected priorities for resolving a forced return cargo casualty.

use super::{CargoItem, CargoStatus, GameSession};
use crate::data::GameData;
use crate::engine::RiskOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReturnPolicy {
    Standard,
    ProtectObjective,
    ProtectValue,
}

impl Default for ReturnPolicy {
    fn default() -> Self {
        Self::Standard
    }
}

impl ReturnPolicy {
    pub const fn next(self) -> Self {
        match self {
            Self::Standard => Self::ProtectObjective,
            Self::ProtectObjective => Self::ProtectValue,
            Self::ProtectValue => Self::Standard,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::ProtectObjective => "PROTECT OBJECTIVE",
            Self::ProtectValue => "PROTECT VALUE",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::ProtectObjective => "OBJECTIVE",
            Self::ProtectValue => "VALUE",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Standard => "normal casualty selection",
            Self::ProtectObjective => "sacrifice another load before the contract target",
            Self::ProtectValue => "sacrifice the lowest-value load first",
        }
    }
}

impl GameSession {
    pub fn return_policy(&self) -> Option<ReturnPolicy> {
        self.expedition
            .as_ref()
            .map(|expedition| expedition.return_policy)
    }

    pub fn cycle_return_policy(&mut self) -> Result<String, String> {
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "return priorities are available during an active haul".to_owned())?;
        expedition.return_policy = expedition.return_policy.next();
        Ok(format!(
            "Return policy: {}. {}.",
            expedition.return_policy.label(),
            expedition.return_policy.description()
        ))
    }
}

pub(crate) fn select_casualty<'a>(
    cargo: &'a mut [CargoItem],
    data: &GameData,
    policy: ReturnPolicy,
    protected_objective: Option<&str>,
    outcome: RiskOutcome,
) -> Option<&'a mut CargoItem> {
    let mut candidates: Vec<usize> = cargo
        .iter()
        .enumerate()
        .filter(|(_, item)| item.status == CargoStatus::Packed)
        .map(|(index, _)| index)
        .collect();
    if policy == ReturnPolicy::ProtectObjective {
        let unprotected: Vec<usize> = candidates
            .iter()
            .copied()
            .filter(|index| {
                protected_objective.map_or(true, |target_id| cargo[*index].object_id != target_id)
            })
            .collect();
        if !unprotected.is_empty() {
            candidates = unprotected;
        }
    }
    let protect_value = policy == ReturnPolicy::ProtectValue;
    let lose_highest = outcome == RiskOutcome::LostSalvage && !protect_value;
    let selected = candidates.into_iter().max_by_key(|index| {
        let value = data
            .salvage_objects
            .get(&cargo[*index].object_id)
            .map_or(0, |object| object.sale_value);
        if lose_highest {
            value
        } else {
            -value
        }
    })?;
    cargo.get_mut(selected)
}

#[cfg(test)]
mod tests;
