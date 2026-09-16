//! Shared scene rectangles for drawing and pointer hit-testing.

use super::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SalvageLayout {
    pub viewport: Rect,
    pub ship: Rect,
    pub wreck: Rect,
    pub command: Rect,
    pub target_panel: Rect,
    pub power_relay: Rect,
    pub navigation_core: Rect,
    pub engine_assembly: Rect,
}

impl SalvageLayout {
    pub fn target_rect(self, target_id: &str) -> Option<Rect> {
        if target_id.starts_with("wreck:") {
            let (_, slot) = target_id.rsplit_once(':')?;
            let index = slot.parse::<usize>().ok()?;
            return (index < 4)
                .then(|| Rect::new(430.0 + index as f32 * 195.0, 310.0, 150.0, 96.0));
        }
        match target_id {
            "industrial_battery" | "quantum_lens" | "medical_supplies" | "shield_generator" => {
                Some(self.power_relay)
            }
            "navigation_computer" => Some(self.navigation_core),
            "engine_assembly" | "damaged_reactor" | "experimental_sensor" | "military_crate" => {
                Some(self.engine_assembly)
            }
            "titanium_plating" | "sealed_container" | "trade_crate" => Some(self.navigation_core),
            _ => None,
        }
    }
}

pub fn salvage_layout() -> SalvageLayout {
    SalvageLayout {
        viewport: Rect::new(0.0, 84.0, LOGICAL_WIDTH, LOGICAL_HEIGHT - 84.0),
        ship: Rect::new(32.0, 330.0, 330.0, 172.0),
        wreck: Rect::new(300.0, 196.0, 930.0, 324.0),
        command: Rect::new(24.0, 558.0, 374.0, 144.0),
        target_panel: Rect::new(414.0, 558.0, 838.0, 144.0),
        power_relay: Rect::new(472.0, 300.0, 140.0, 88.0),
        navigation_core: Rect::new(728.0, 280.0, 154.0, 96.0),
        engine_assembly: Rect::new(1010.0, 336.0, 166.0, 96.0),
    }
}
