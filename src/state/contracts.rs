//! Contract completion and payout rules for authored wreck objectives.

use super::reputation::SalvageStanding;
use super::{CargoItem, CargoStatus, GameData, GameSession};
use crate::data::SiteData;

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
    pub fn contract_streak(&self) -> u32 {
        self.career.contract_streak
    }

    pub fn next_contract_streak_bonus(&self) -> i64 {
        self.career.next_contract_streak_bonus()
    }

    pub fn contract_objective_status(
        &self,
        site_id: &str,
        data: &GameData,
    ) -> Option<ContractObjectiveStatus> {
        if self.expedition.as_ref().is_some_and(|expedition| {
            expedition.site_id == site_id && !expedition.contract_accepted
        }) || self.expedition.is_none()
            && self
                .last_voyage()
                .is_some_and(|record| record.site_id == site_id && !record.contract_accepted)
        {
            return None;
        }
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
        self.complete_site_contract_for_run(site_id, cargo, data, true)
    }

    pub(crate) fn complete_site_contract_for_run(
        &mut self,
        site_id: &str,
        cargo: &[CargoItem],
        data: &GameData,
        contract_accepted: bool,
    ) -> Option<String> {
        if !contract_accepted {
            return None;
        }
        let site = data.sites.get(site_id)?;
        let target_id = site.contract_target.as_deref()?;
        let progress = self.site_progress.get(site_id)?;
        if progress.contract_completed || progress.contract_failed {
            return None;
        }
        let packed = cargo
            .iter()
            .any(|item| item.object_id == target_id && item.status == CargoStatus::Packed);
        if !packed {
            return self.site_contract_failure(site_id, target_id, data);
        }
        self.mark_contract_complete(site_id);
        Some(self.site_contract_completion(site, target_id, data))
    }

    fn site_contract_failure(
        &mut self,
        site_id: &str,
        target_id: &str,
        data: &GameData,
    ) -> Option<String> {
        let progress = self.site_progress.get_mut(site_id)?;
        if !progress.removed_targets.iter().any(|id| id == target_id) {
            return None;
        }
        progress.contract_failed = true;
        self.career.record_contract_failure();
        let standing_before = self.salvage_standing();
        self.reputation = self.reputation.saturating_sub(1);
        let standing_after = self.salvage_standing();
        let standing_notice = standing_change_notice(standing_before, standing_after, false);
        Some(format!(
            " Contract failed: {} was lost before returning to port. Standing -1. Contract streak reset.{standing_notice}",
            contract_target_name(data, target_id),
        ))
    }

    fn mark_contract_complete(&mut self, site_id: &str) {
        if let Some(progress) = self.site_progress.get_mut(site_id) {
            progress.contract_completed = true;
        }
    }

    fn site_contract_completion(
        &mut self,
        site: &SiteData,
        target_id: &str,
        data: &GameData,
    ) -> String {
        let (contract_streak, streak_bonus) = self.career.record_contract_success();
        let standing_before = self.salvage_standing();
        let standing_bonus = self.contract_reward_bonus(site.contract_reward);
        let contract_payout = site.contract_reward + standing_bonus + streak_bonus;
        self.economy.credits += contract_payout;
        self.career.record_contract_income(contract_payout);
        self.reputation = self.reputation.saturating_add(1);
        let standing_after = self.salvage_standing();
        let newly_unlocked = self.refresh_module_unlocks(data);
        let blueprint_notice = blueprint_notice(data, &newly_unlocked);
        let standing_notice = standing_change_notice(standing_before, standing_after, true);
        let payout_notice = payout_notice(standing_bonus, streak_bonus, contract_payout);
        let streak_notice = streak_notice(contract_streak, streak_bonus);
        format!(
            " Contract complete: {} recovered. You keep the hardware; no hand-in needed. Bonus +{} credits.{}{}{}{}",
            contract_target_name(data, target_id),
            site.contract_reward,
            payout_notice,
            standing_notice,
            streak_notice,
            blueprint_notice,
        )
    }
}

fn contract_target_name<'a>(data: &'a GameData, target_id: &'a str) -> &'a str {
    data.salvage_objects
        .get(target_id)
        .map_or(target_id, |target| target.display_name.as_str())
}

fn standing_change_notice(
    standing_before: SalvageStanding,
    standing_after: SalvageStanding,
    advanced: bool,
) -> String {
    if standing_before != standing_after {
        let verb = if advanced { "advanced to" } else { "fell to" };
        format!(" Standing {verb} {}.", standing_after.label())
    } else if advanced {
        format!(" Standing +1; {} remains active.", standing_after.label())
    } else {
        format!(" Standing held at {}.", standing_after.label())
    }
}

fn blueprint_notice(data: &GameData, newly_unlocked: &[String]) -> String {
    if newly_unlocked.is_empty() {
        return String::new();
    }
    format!(
        " Blueprint unlocked: {}.",
        newly_unlocked
            .iter()
            .filter_map(|id| data.modules.get(id))
            .map(|module| module.display_name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn payout_notice(standing_bonus: i64, streak_bonus: i64, contract_payout: i64) -> String {
    match (standing_bonus > 0, streak_bonus > 0) {
        (true, true) => format!(
            " Standing bonus +{}; streak bonus +{} credits; paid {} credits total.",
            standing_bonus, streak_bonus, contract_payout
        ),
        (true, false) => format!(
            " Standing bonus +{}; paid {} credits total.",
            standing_bonus, contract_payout
        ),
        (false, true) => format!(
            " Streak bonus +{} credits; paid {} credits total.",
            streak_bonus, contract_payout
        ),
        (false, false) => format!(" Paid {} credits.", contract_payout),
    }
}

fn streak_notice(contract_streak: u32, streak_bonus: i64) -> String {
    if streak_bonus > 0 {
        format!(" Contract streak x{contract_streak}.")
    } else {
        " Contract streak started.".to_owned()
    }
}
