//! Port screen identity and safe-checkpoint copy.

pub const TITLE: &str = "PORT // SAFE CHECKPOINT";

use super::{GameData, GameSession};

impl GameSession {
    pub fn max_fuel(&self, data: &GameData) -> i32 {
        data.config.max_fuel + self.module_stats(data).fuel_capacity
    }

    pub fn remove_module(&mut self, module_id: &str, data: &GameData) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("finish the current expedition before changing the ship".to_owned());
        }
        let module = data
            .modules
            .get(module_id)
            .ok_or_else(|| format!("unknown module '{module_id}'"))?;
        if !self
            .ship_layout
            .placements
            .iter()
            .any(|item| item.permanent && item.id == module_id)
        {
            return Err(format!("{} is not installed", module.display_name));
        }
        if self.economy.credits < module.remove_cost {
            return Err(format!("removal requires {} credits", module.remove_cost));
        }
        self.ship_layout.remove(module_id);
        self.economy.credits -= module.remove_cost;
        self.career.record_module_change(module.remove_cost);
        self.damaged_modules.retain(|id| id != module_id);
        Ok(format!(
            "Removed {} for {} credits",
            module.display_name, module.remove_cost
        ))
    }

    pub fn purchase_module(&mut self, module_id: &str, data: &GameData) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("finish the current expedition before changing the ship".to_owned());
        }
        self.refresh_module_unlocks(data);
        let module = data
            .modules
            .get(module_id)
            .ok_or_else(|| format!("unknown module '{module_id}'"))?;
        if !self.module_is_unlocked(module_id, data) {
            return Err(format!(
                "{} blueprint is locked; reach {} credits to unlock it",
                module.display_name, module.unlock_credits
            ));
        }
        if self
            .ship_layout
            .placements
            .iter()
            .any(|item| item.permanent && item.id == module_id)
        {
            return Err(format!("{} is already installed", module.display_name));
        }
        if self.economy.credits < module.purchase_cost {
            return Err(format!(
                "{} requires {} credits",
                module.display_name, module.purchase_cost
            ));
        }
        let Some((position, rotation)) =
            self.ship_layout
                .first_fit(module_id, module.footprint, true)
        else {
            return Err(format!(
                "{} has no open fit in the ship grid",
                module.display_name
            ));
        };
        self.ship_layout
            .place(module_id, module.footprint, position, rotation, true)
            .map_err(|error| error.to_string())?;
        self.economy.credits -= module.purchase_cost;
        self.career.record_module_change(module.purchase_cost);
        if !self.unlocked_modules.iter().any(|id| id == module_id) {
            self.unlocked_modules.push(module_id.to_owned());
        }
        if module.external_capacity > 0 {
            Ok(format!(
                "Bought and installed {} for {} credits. Clamp capacity is now {}.",
                module.display_name,
                module.purchase_cost,
                self.external_capacity(data)
            ))
        } else {
            Ok(format!(
                "Bought and installed {} for {} credits",
                module.display_name, module.purchase_cost
            ))
        }
    }

    pub fn refuel(&mut self, data: &GameData) -> Result<String, String> {
        let missing = (self.max_fuel(data) - self.economy.fuel).max(0);
        let affordable = self.economy.credits / i64::from(data.config.refuel_price_per_unit);
        let amount = missing.min(affordable as i32);
        if amount == 0 {
            return Err("fuel tank is full or credits are too low".to_owned());
        }
        let cost = i64::from(amount * data.config.refuel_price_per_unit);
        self.economy.fuel += amount;
        self.economy.credits -= cost;
        self.career.record_refuel(amount, cost);
        Ok(format!("Refuelled {amount} units for {cost} credits"))
    }
}
