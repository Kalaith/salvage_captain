//! Safe-port refit slots for reusable permanent-module arrangements.

use super::GameSession;
use crate::data::GameData;
use crate::engine::packing::{PlacedItem, ShipLayout};

pub const SLOT_COUNT: usize = 3;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadoutPreset {
    pub placements: Vec<PlacedItem>,
}

impl GameSession {
    pub fn loadout_slot(&self, slot: usize) -> Option<&LoadoutPreset> {
        self.loadout_slots.get(slot).and_then(Option::as_ref)
    }

    pub fn loadout_matches_current(&self, slot: usize) -> bool {
        let Some(preset) = self.loadout_slot(slot) else {
            return false;
        };
        let current = self
            .ship_layout
            .placements
            .iter()
            .filter(|item| item.permanent)
            .cloned()
            .collect::<Vec<_>>();
        canonical_placements(&preset.placements) == canonical_placements(&current)
    }

    pub fn store_loadout(&mut self, slot: usize) -> Result<String, String> {
        self.ensure_safe_port()?;
        if slot >= SLOT_COUNT {
            return Err(format!("loadout slot {} does not exist", slot + 1));
        }
        let placements = self
            .ship_layout
            .placements
            .iter()
            .filter(|item| item.permanent)
            .cloned()
            .collect::<Vec<_>>();
        let module_count = placements.len();
        self.loadout_slots[slot] = Some(LoadoutPreset { placements });
        Ok(format!(
            "Loadout slot {} stored. {module_count} permanent module(s) remembered.",
            slot + 1
        ))
    }

    pub fn apply_loadout(&mut self, slot: usize, data: &GameData) -> Result<String, String> {
        self.ensure_safe_port()?;
        if slot >= SLOT_COUNT {
            return Err(format!("loadout slot {} does not exist", slot + 1));
        }
        let preset = self
            .loadout_slot(slot)
            .cloned()
            .ok_or_else(|| format!("loadout slot {} is empty", slot + 1))?;
        let candidate = build_layout(&preset, self, data)?;
        let stats =
            crate::engine::progression::stats_from_layout(&candidate, data, &self.damaged_modules);
        let max_fuel = data.config.max_fuel + stats.fuel_capacity;
        if self.economy.fuel > max_fuel {
            return Err(format!(
                "loadout slot {} caps fuel at {}; burn or refit before applying",
                slot + 1,
                max_fuel
            ));
        }
        let max_hull = self.max_hull + stats.hull;
        if self.hull > max_hull {
            return Err(format!(
                "loadout slot {} caps hull at {}; repair or refit before applying",
                slot + 1,
                max_hull
            ));
        }
        let module_count = candidate.placements.len();
        self.ship_layout = candidate;
        self.damaged_modules.retain(|id| {
            self.ship_layout
                .placements
                .iter()
                .any(|item| item.permanent && item.id == *id)
        });
        Ok(format!(
            "Loadout slot {} applied. {module_count} permanent module(s) online.",
            slot + 1
        ))
    }

    fn ensure_safe_port(&self) -> Result<(), String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            Err("loadouts can be changed only at the safe port".to_owned())
        } else {
            Ok(())
        }
    }
}

pub(crate) fn validate_saved_loadouts(
    slots: &[Option<LoadoutPreset>; SLOT_COUNT],
    session: &GameSession,
    data: &GameData,
) -> Result<(), String> {
    for (slot, preset) in slots.iter().enumerate() {
        if let Some(preset) = preset {
            build_layout(preset, session, data)
                .map_err(|error| format!("save loadout slot {} is invalid: {error}", slot + 1))?;
        }
    }
    Ok(())
}

fn build_layout(
    preset: &LoadoutPreset,
    session: &GameSession,
    data: &GameData,
) -> Result<ShipLayout, String> {
    let mut layout = ShipLayout::new(session.ship_layout.width, session.ship_layout.height);
    for item in &preset.placements {
        if !item.permanent {
            return Err(format!("'{}' is not a permanent module", item.id));
        }
        let module = data
            .modules
            .get(&item.id)
            .ok_or_else(|| format!("unknown module '{}'", item.id))?;
        if !session.module_is_unlocked(&item.id, data) {
            return Err(format!("module '{}' is still blueprint-locked", item.id));
        }
        if item.footprint != module.footprint {
            return Err(format!("module '{}' has a mismatched footprint", item.id));
        }
        layout
            .place(
                item.id.clone(),
                item.footprint,
                item.position,
                item.rotation,
                true,
            )
            .map_err(|error| format!("{}: {error}", item.id))?;
    }
    Ok(layout)
}

fn canonical_placements(items: &[PlacedItem]) -> Vec<PlacedItem> {
    let mut placements = items.to_vec();
    placements.sort_by(|left, right| {
        left.id
            .cmp(&right.id)
            .then(left.position.y.cmp(&right.position.y))
            .then(left.position.x.cmp(&right.position.x))
            .then(left.rotation.cmp(&right.rotation))
    });
    placements
}

#[cfg(test)]
mod tests;
