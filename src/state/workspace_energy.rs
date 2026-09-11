//! One-shot field power recovery that keeps the return reserve protected.

use super::{GameData, GameSession, WorkspaceLogEvent};

pub const POWER_CYCLE_FUEL_COST: i32 = 1;
pub const POWER_CYCLE_ENERGY_RESTORE: i32 = 4;

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
