//! Shared scene rectangles for drawing and pointer hit-testing.

use super::LOGICAL_HEIGHT;
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
        viewport: Rect::new(24.0, 112.0, 1232.0, 488.0),
        ship: Rect::new(74.0, 326.0, 244.0, 126.0),
        wreck: Rect::new(414.0, 160.0, 518.0, 580.0),
        command: Rect::new(24.0, 496.0, 360.0, 104.0),
        target_panel: Rect::new(958.0, 160.0, 274.0, 350.0),
        power_relay: Rect::new(514.0, 264.0, 112.0, 72.0),
        navigation_core: Rect::new(682.0, 214.0, 126.0, 86.0),
        engine_assembly: Rect::new(756.0, 374.0, 146.0, 86.0),
    }
}

pub fn travel_view() -> Rect {
    Rect::new(24.0, 112.0, 1232.0, LOGICAL_HEIGHT - 232.0)
}

pub fn travel_ship_rect(progress: f32) -> Rect {
    Rect::new(
        118.0 + progress.clamp(0.0, 1.0) * 620.0,
        336.0,
        208.0,
        108.0,
    )
}
