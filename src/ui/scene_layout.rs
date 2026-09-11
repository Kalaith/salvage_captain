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
        match target_id {
            "industrial_battery" | "quantum_lens" | "medical_supplies" => Some(self.power_relay),
            "navigation_computer" | "experimental_sensor" => Some(self.navigation_core),
            "engine_assembly" | "damaged_reactor" | "shield_generator" | "military_crate" => {
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
        ship: Rect::new(72.0, 332.0, 278.0, 144.0),
        wreck: Rect::new(410.0, 134.0, 560.0, 586.0),
        command: Rect::new(30.0, 548.0, 360.0, 142.0),
        target_panel: Rect::new(994.0, 132.0, 258.0, 350.0),
        power_relay: Rect::new(520.0, 250.0, 118.0, 76.0),
        navigation_core: Rect::new(700.0, 204.0, 132.0, 90.0),
        engine_assembly: Rect::new(778.0, 380.0, 150.0, 90.0),
    }
}

pub fn travel_view() -> Rect {
    Rect::new(0.0, 84.0, LOGICAL_WIDTH, LOGICAL_HEIGHT - 84.0)
}

pub fn travel_ship_rect(progress: f32) -> Rect {
    Rect::new(
        144.0 + progress.clamp(0.0, 1.0) * 590.0,
        352.0,
        238.0,
        124.0,
    )
}
