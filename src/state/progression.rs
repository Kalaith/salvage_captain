//! Persistent blueprint milestones for ship upgrades.

use super::GameSession;
use crate::data::{GameData, ModuleData};
use std::collections::HashSet;

impl GameSession {
    pub fn refresh_module_unlocks(&mut self, data: &GameData) -> Vec<String> {
        let known: HashSet<&str> = self.unlocked_modules.iter().map(String::as_str).collect();
        let mut newly_unlocked: Vec<String> = data
            .modules
            .iter()
            .filter(|(module_id, module)| {
                module.unlock_credits <= self.economy.credits && !known.contains(module_id.as_str())
            })
            .map(|(module_id, _)| module_id.clone())
            .collect();
        newly_unlocked.sort();
        self.unlocked_modules.extend(newly_unlocked.iter().cloned());
        self.milestone_reached = self.economy.credits >= data.config.progression_credit_threshold
            && self.unlocked_modules.len() > data.config.starting_modules.len();
        newly_unlocked
    }

    pub fn module_is_unlocked(&self, module_id: &str, data: &GameData) -> bool {
        self.unlocked_modules.iter().any(|id| id == module_id)
            || data
                .config
                .starting_modules
                .iter()
                .any(|module| module.module_id == module_id)
    }

    pub fn unlocked_module_count(&self, data: &GameData) -> usize {
        data.modules
            .iter()
            .filter(|(module_id, _)| self.module_is_unlocked(module_id, data))
            .count()
    }

    pub fn next_module_unlock<'a>(&self, data: &'a GameData) -> Option<&'a ModuleData> {
        data.modules
            .iter()
            .filter(|(module_id, _)| !self.module_is_unlocked(module_id, data))
            .min_by_key(|(_, module)| module.unlock_credits)
            .map(|(_, module)| module)
    }
}
