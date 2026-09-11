//! Contract completion and payout rules for authored wreck objectives.

use super::{CargoItem, CargoStatus, GameData, GameSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractObjectiveState {
    Open,
    Recovered,
    Complete,
    Failed,
}

impl ContractObjectiveState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Open => "OPEN",
            Self::Recovered => "IN HOLD",
            Self::Complete => "COMPLETE",
            Self::Failed => "FAILED",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractObjectiveStatus {
    pub target_id: String,
    pub state: ContractObjectiveState,
}

impl GameSession {
    pub fn contract_objective_status(
        &self,
        site_id: &str,
        data: &GameData,
    ) -> Option<ContractObjectiveStatus> {
        let site = data.sites.get(site_id)?;
        let target_id = site.contract_target.as_deref()?;
        let progress = self.site_progress.get(site_id);
        let removed =
            progress.is_some_and(|value| value.removed_targets.iter().any(|id| id == target_id));
        let in_hold = removed && self.expedition_contains_target(site_id, target_id);
        let state = if progress.is_some_and(|value| value.contract_completed) {
            ContractObjectiveState::Complete
        } else if progress.is_some_and(|value| value.contract_failed) || removed && !in_hold {
            ContractObjectiveState::Failed
        } else if in_hold {
            ContractObjectiveState::Recovered
        } else {
            ContractObjectiveState::Open
        };
        Some(ContractObjectiveStatus {
            target_id: target_id.to_owned(),
            state,
        })
    }

    fn expedition_contains_target(&self, site_id: &str, target_id: &str) -> bool {
        self.expedition.as_ref().is_some_and(|expedition| {
            expedition.site_id == site_id
                && expedition.cargo.iter().any(|item| {
                    item.object_id == target_id
                        && matches!(item.status, CargoStatus::Pending | CargoStatus::Packed)
                })
        })
    }

    pub fn complete_site_contract(
        &mut self,
        site_id: &str,
        cargo: &[CargoItem],
        data: &GameData,
    ) -> Option<String> {
        let site = data.sites.get(site_id)?;
        let target_id = site.contract_target.as_deref()?;
        let progress = self.site_progress.get_mut(site_id)?;
        if progress.contract_completed || progress.contract_failed {
            return None;
        }
        let packed = cargo
            .iter()
            .any(|item| item.object_id == target_id && item.status == CargoStatus::Packed);
        if !packed {
            if progress.removed_targets.iter().any(|id| id == target_id) {
                progress.contract_failed = true;
                return Some(format!(
                    " Contract failed: {} was lost before delivery.",
                    data.salvage_objects
                        .get(target_id)
                        .map_or(target_id, |target| target.display_name.as_str())
                ));
            }
            return None;
        }
        progress.contract_completed = true;
        self.economy.credits += site.contract_reward;
        self.milestone_reached = self.economy.credits >= data.config.progression_credit_threshold
            && self.unlocked_modules.len() > data.config.starting_modules.len();
        let target_name = data
            .salvage_objects
            .get(target_id)
            .map_or(target_id, |target| target.display_name.as_str());
        Some(format!(
            " Contract complete: {} delivered. Bonus +{} credits.",
            target_name, site.contract_reward
        ))
    }
}

#[cfg(test)]
mod tests;
