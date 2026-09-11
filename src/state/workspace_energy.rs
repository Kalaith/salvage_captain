//! One-shot field power recovery that keeps the return reserve protected.

use super::{GameData, GameSession, WorkspaceLogEvent};

pub const POWER_CYCLE_FUEL_COST: i32 = 1;
pub const POWER_CYCLE_ENERGY_RESTORE: i32 = 4;
pub const FIELD_POWER_CELL_PRICE: i64 = 80;
pub const FIELD_POWER_CELL_ALLOY_COST: i32 = 1;
pub const FIELD_POWER_CELL_ELECTRONICS_COST: i32 = 1;
pub const FIELD_POWER_CELL_ENERGY_RESTORE: i32 = 4;
pub const MAX_FIELD_POWER_CELLS: u8 = 3;

impl GameSession {
    pub fn can_buy_field_power_cell(&self) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && self.field_power_cells < MAX_FIELD_POWER_CELLS
            && self.economy.credits >= FIELD_POWER_CELL_PRICE
    }

    pub fn buy_field_power_cell(&mut self) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("field power cells are stocked at the safe port".to_owned());
        }
        if self.field_power_cells >= MAX_FIELD_POWER_CELLS {
            return Err("field power cell rack is full".to_owned());
        }
        if self.economy.credits < FIELD_POWER_CELL_PRICE {
            return Err(format!(
                "field power cell requires {} credits",
                FIELD_POWER_CELL_PRICE
            ));
        }
        self.economy.credits -= FIELD_POWER_CELL_PRICE;
        self.field_power_cells += 1;
        self.career
            .record_field_power_cell_purchase(FIELD_POWER_CELL_PRICE);
        Ok(format!(
            "Bought field power cell for {} credits. Stock {}/{}.",
            FIELD_POWER_CELL_PRICE, self.field_power_cells, MAX_FIELD_POWER_CELLS
        ))
    }

    pub fn can_fabricate_field_power_cell(&self) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && self.field_power_cells < MAX_FIELD_POWER_CELLS
            && self.economy.alloy >= FIELD_POWER_CELL_ALLOY_COST
            && self.economy.electronics >= FIELD_POWER_CELL_ELECTRONICS_COST
    }

    pub fn fabricate_field_power_cell(&mut self) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("field power cells are fabricated at the safe port".to_owned());
        }
        if self.field_power_cells >= MAX_FIELD_POWER_CELLS {
            return Err("field power cell rack is full".to_owned());
        }
        if self.economy.alloy < FIELD_POWER_CELL_ALLOY_COST
            || self.economy.electronics < FIELD_POWER_CELL_ELECTRONICS_COST
        {
            return Err(format!(
                "fabrication needs {} Alloy and {} Electronics",
                FIELD_POWER_CELL_ALLOY_COST, FIELD_POWER_CELL_ELECTRONICS_COST
            ));
        }
        self.economy.alloy -= FIELD_POWER_CELL_ALLOY_COST;
        self.economy.electronics -= FIELD_POWER_CELL_ELECTRONICS_COST;
        self.field_power_cells += 1;
        self.career.record_field_power_cell_fabrication(
            FIELD_POWER_CELL_ALLOY_COST,
            FIELD_POWER_CELL_ELECTRONICS_COST,
        );
        Ok(format!(
            "Fabricated field power cell from salvage. Stock {}/{}.",
            self.field_power_cells, MAX_FIELD_POWER_CELLS
        ))
    }

    pub fn can_use_field_power_cell(&self) -> bool {
        self.field_power_cells > 0
            && self.expedition.as_ref().is_some_and(|expedition| {
                expedition.workspace_energy < expedition.workspace_energy_capacity
            })
    }

    pub fn use_field_power_cell(&mut self) -> Result<String, String> {
        let (energy, capacity) = self
            .expedition
            .as_ref()
            .map(|expedition| {
                (
                    expedition.workspace_energy,
                    expedition.workspace_energy_capacity,
                )
            })
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        if self.field_power_cells == 0 {
            return Err("no field power cells remain in the rack".to_owned());
        }
        if energy >= capacity {
            return Err("workspace power reserve is already full".to_owned());
        }
        let restored = FIELD_POWER_CELL_ENERGY_RESTORE.min(capacity - energy);
        self.field_power_cells -= 1;
        self.career.record_field_power_cell_use();
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_energy += restored;
        let remaining = expedition.workspace_energy;
        let power_capacity = expedition.workspace_energy_capacity;
        let section_id = expedition.workspace_section.clone();
        let site_id = expedition.site_id.clone();
        self.append_workspace_log(
            &site_id,
            WorkspaceLogEvent::FieldPowerCellUsed,
            Some(section_id.as_str()),
            None,
        );
        Ok(format!(
            "Field power cell used: +{restored} power. Reserve {remaining}/{power_capacity}; {} cell(s) remain.",
            self.field_power_cells
        ))
    }
}

impl GameSession {
    pub fn can_power_cycle_workspace(&self, data: &GameData) -> bool {
        let Some(expedition) = self.expedition.as_ref() else {
            return false;
        };
        expedition.power_cycles_used == 0
            && expedition.workspace_energy < expedition.workspace_energy_capacity
            && self.economy.fuel > data.config.safe_return_buffer
    }

    pub fn power_cycle_workspace(&mut self, data: &GameData) -> Result<String, String> {
        let (energy, capacity, cycles_used) = self
            .expedition
            .as_ref()
            .map(|expedition| {
                (
                    expedition.workspace_energy,
                    expedition.workspace_energy_capacity,
                    expedition.power_cycles_used,
                )
            })
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        if cycles_used > 0 {
            return Err("field power cycle already used this run".to_owned());
        }
        if energy >= capacity {
            return Err("workspace power reserve is already full".to_owned());
        }
        if self.economy.fuel <= data.config.safe_return_buffer {
            return Err(format!(
                "power cycle unavailable: keep {} fuel for the return burn",
                data.config.safe_return_buffer
            ));
        }
        let restored = POWER_CYCLE_ENERGY_RESTORE.min(capacity - energy);
        self.economy.fuel -= POWER_CYCLE_FUEL_COST;
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_energy += restored;
        expedition.power_cycles_used = 1;
        let remaining = expedition.workspace_energy;
        let power_capacity = expedition.workspace_energy_capacity;
        let section_id = expedition.workspace_section.clone();
        let site_id = expedition.site_id.clone();
        let fuel_remaining = self.economy.fuel;
        self.append_workspace_log(
            &site_id,
            WorkspaceLogEvent::PowerCycled,
            Some(section_id.as_str()),
            None,
        );
        Ok(format!(
            "Field power cycled: +{restored} power for {POWER_CYCLE_FUEL_COST} fuel. Reserve {remaining}/{power_capacity}; FUEL {fuel_remaining} // RETURN {} RESERVED.",
            data.config.safe_return_buffer
        ))
    }
}
