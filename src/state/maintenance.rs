//! Port maintenance quotes for hull damage and offline ship modules.

use super::GameSession;
use crate::data::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairQuote {
    pub missing_hull: i32,
    pub offline_modules: usize,
    pub hull_cost: i64,
    pub module_cost: i64,
    pub total_cost: i64,
}

impl RepairQuote {
    pub const fn is_due(self) -> bool {
        self.total_cost > 0
    }
}

impl GameSession {
    pub fn repair_quote(&self, data: &GameData) -> RepairQuote {
        let missing_hull = (self.max_hull_with_modules(data) - self.hull).max(0);
        let offline_modules = self.damaged_modules.len();
        let hull_cost = i64::from(missing_hull) * i64::from(data.config.repair_price_per_hull);
        let module_cost = offline_modules as i64 * i64::from(data.config.module_repair_price);
        RepairQuote {
            missing_hull,
            offline_modules,
            hull_cost,
            module_cost,
            total_cost: hull_cost + module_cost,
        }
    }

    pub fn repair(&mut self, data: &GameData) -> Result<String, String> {
        let quote = self.repair_quote(data);
        if !quote.is_due() {
            return Err("the ship does not need repairs".to_owned());
        }
        if self.economy.credits < quote.total_cost {
            return Err(format!(
                "repairs require {} credits (hull {} + modules {})",
                quote.total_cost, quote.hull_cost, quote.module_cost
            ));
        }
        let restored: Vec<String> = self
            .damaged_modules
            .iter()
            .filter_map(|id| {
                data.modules
                    .get(id)
                    .map(|module| module.display_name.clone())
            })
            .collect();
        self.economy.credits -= quote.total_cost;
        self.damaged_modules.clear();
        self.hull = self.max_hull_with_modules(data);
        self.career
            .record_repair(quote.total_cost, quote.offline_modules);
        if restored.is_empty() {
            Ok(format!("Repaired hull for {} credits", quote.total_cost))
        } else {
            Ok(format!(
                "Repaired hull and systems for {} credits. Restored: {}.",
                quote.total_cost,
                restored.join(", ")
            ))
        }
    }
}
