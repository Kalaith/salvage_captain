//! Expedition packing screen identity.

use super::{CargoStatus, GameSession};
use crate::data::GameData;

pub const TITLE: &str = "PACK THE HAUL";

impl GameSession {
    pub fn external_capacity(&self, data: &GameData) -> i32 {
        self.module_stats(data).external_capacity
    }

    pub fn external_cargo_count(&self, data: &GameData, excluding: Option<&str>) -> i32 {
        let in_expedition = self.expedition.as_ref().map_or(0, |expedition| {
            expedition
                .cargo
                .iter()
                .filter(|cargo| cargo.status == CargoStatus::Packed)
                .filter(|cargo| Some(cargo.object_id.as_str()) != excluding)
                .filter(|cargo| {
                    data.salvage_objects
                        .get(&cargo.object_id)
                        .is_some_and(|object| object.transfer_mode != "internal_cargo")
                })
                .count() as i32
        });
        let returned = self.returned.iter().filter(|cargo| {
            Some(cargo.object_id.as_str()) != excluding
                && data
                    .salvage_objects
                    .get(&cargo.object_id)
                    .is_some_and(|object| object.transfer_mode != "internal_cargo")
        });
        in_expedition + returned.count() as i32
    }
}
