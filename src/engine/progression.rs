//! Aggregate visible capability effects from the installed module layout.

use crate::data::{GameData, ModuleEffect};
use crate::engine::ShipLayout;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModuleStats {
    pub fuel_capacity: i32,
    pub fuel_efficiency: i32,
    pub external_capacity: i32,
    pub drone_support: i32,
    pub hull: i32,
    pub power: i32,
    pub scanning: i32,
    pub shielding: i32,
}

pub fn stats_from_layout(
    layout: &ShipLayout,
    data: &GameData,
    damaged_modules: &[String],
) -> ModuleStats {
    let mut stats = ModuleStats::default();
    for placement in layout.placements.iter().filter(|item| item.permanent) {
        if damaged_modules.iter().any(|id| id == &placement.id) {
            continue;
        }
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
        stats.external_capacity += module.external_capacity;
        stats.drone_support += module.drone_support;
    }
    stats
}
