//! Aggregate visible capability effects from the installed module layout.

use crate::data::{GameData, ModuleEffect};
use crate::engine::ShipLayout;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModuleStats {
    pub fuel_capacity: i32,
    pub fuel_efficiency: i32,
    pub hull: i32,
    pub power: i32,
    pub scanning: i32,
    pub shielding: i32,
}

pub fn stats_from_layout(layout: &ShipLayout, data: &GameData) -> ModuleStats {
    let mut stats = ModuleStats::default();
    for placement in layout.placements.iter().filter(|item| item.permanent) {
        let Some(module) = data.modules.get(&placement.id) else {
            continue;
        };
        match module.effect {
            ModuleEffect::CargoSpace => {}
            ModuleEffect::FuelCapacity(value) => stats.fuel_capacity += value,
            ModuleEffect::FuelEfficiency(value) => stats.fuel_efficiency += value,
            ModuleEffect::Hull(value) => stats.hull += value,
            ModuleEffect::Power(value) => stats.power += value,
            ModuleEffect::Scanning(value) => stats.scanning += value,
            ModuleEffect::Shielding(value) => stats.shielding += value,
        }
    }
    stats
}
