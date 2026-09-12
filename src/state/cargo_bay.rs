//! Persistent internal cargo-berth upgrades for the shipyard.

use super::{CargoStatus, GameData, GameSession};
use crate::state::workspace::TransferMode;

pub const MAX_CARGO_BAY_LEVEL: u8 = 2;
const BASE_INTERNAL_CARGO_CAPACITY: i32 = 3;
const CAPACITY_PER_UPGRADE: i32 = 2;
const UPGRADE_COSTS: [i64; MAX_CARGO_BAY_LEVEL as usize] = [420, 760];

impl GameSession {
    pub fn cargo_bay_level(&self) -> u8 {
        self.cargo_bay_level
    }

    pub fn internal_cargo_capacity(&self) -> i32 {
        BASE_INTERNAL_CARGO_CAPACITY + i32::from(self.cargo_bay_level) * CAPACITY_PER_UPGRADE
    }

    pub fn internal_cargo_count(&self, data: &GameData, excluding: Option<&str>) -> i32 {
        let expedition_count = self.expedition.as_ref().map_or(0, |expedition| {
            expedition
                .cargo
                .iter()
                .filter(|item| item.status == CargoStatus::Packed)
                .filter(|item| Some(item.object_id.as_str()) != excluding)
                .filter(|item| {
                    data.salvage_objects
                        .get(&item.object_id)
                        .is_some_and(|object| {
                            !TransferMode::from_target(object).uses_external_rig()
                        })
                })
                .count() as i32
        });
        let returned_count = self
            .returned
            .iter()
            .filter(|item| Some(item.object_id.as_str()) != excluding)
            .filter(|item| {
                data.salvage_objects
                    .get(&item.object_id)
                    .is_some_and(|object| !TransferMode::from_target(object).uses_external_rig())
            })
            .count() as i32;
        expedition_count + returned_count
    }

    pub fn cargo_bay_upgrade_cost(&self) -> Option<i64> {
        UPGRADE_COSTS.get(self.cargo_bay_level as usize).copied()
    }

    pub fn cargo_bay_upgrade_label(&self) -> String {
        match self.cargo_bay_upgrade_cost() {
            None => "HOLD MAX".to_owned(),
            Some(cost) if self.economy.credits < cost => format!("LOW CR ¢{cost}"),
            Some(cost) => format!("UPGRADE ¢{cost}"),
        }
    }

    pub fn purchase_cargo_bay_upgrade(&mut self) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("finish the current expedition before expanding the cargo bay".to_owned());
        }
        let Some(cost) = self.cargo_bay_upgrade_cost() else {
            return Err("cargo bay is already at maximum capacity".to_owned());
        };
        if self.economy.credits < cost {
            return Err(format!("cargo bay expansion requires {cost} credits"));
        }
        self.economy.credits -= cost;
        self.cargo_bay_level += 1;
        self.career.record_cargo_bay_upgrade();
        Ok(format!(
            "Expanded the cargo bay to {} internal berths for {cost} credits.",
            self.internal_cargo_capacity()
        ))
    }
}
