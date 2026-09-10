//! Contract completion and payout rules for authored wreck objectives.

use super::{CargoItem, CargoStatus, GameData, GameSession};

impl GameSession {
    pub fn complete_site_contract(
        &mut self,
        site_id: &str,
        cargo: &[CargoItem],
        data: &GameData,
    ) -> Option<String> {
        let site = data.sites.get(site_id)?;
        let target_id = site.contract_target.as_deref()?;
        let progress = self.site_progress.get_mut(site_id)?;
        if progress.contract_completed
            || !cargo
                .iter()
                .any(|item| item.object_id == target_id && item.status == CargoStatus::Packed)
        {
            return None;
        }
        progress.contract_completed = true;
        self.economy.credits += site.contract_reward;
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
