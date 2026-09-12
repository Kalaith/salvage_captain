//! Deterministic port and maintenance scenes for UI verification.

use super::Game;
use crate::state::GameState;

impl Game {
    pub(super) fn capture_upgraded_ship(&mut self, scene: &str) -> GameState {
        self.session.economy.credits = 50_000;
        for id in [
            "reactor_module",
            "shield_module",
            "scanner_module",
            "drone_bay",
            "nav_module",
            "battery_module",
            "hull_plating",
        ] {
            super::purchase_capture_module(self, id);
        }
        self.port_tab = crate::ui::port_panel::PortTab::Equipment;
        self.port_selected_module = Some("reactor_module".to_owned());
        if scene.ends_with("damage") {
            self.session.damaged_modules =
                vec!["reactor_module".to_owned(), "scanner_module".to_owned()];
        }
        GameState::Port
    }

    pub(super) fn capture_port_scene(&mut self, scene: &str) -> GameState {
        self.port_tab = crate::ui::port_panel::PortTab::Service;
        self.session.damaged_modules = vec!["engine_core".to_owned()];
        self.session.hull = 7;
        match scene {
            "port_damage" => GameState::Port,
            "port_repair_low_funds" => {
                self.session.economy.credits = 40;
                GameState::Port
            }
            "port_repaired" => {
                self.message = self
                    .session
                    .repair(&self.data)
                    .unwrap_or_else(|error| format!("Repair capture failed: {error}"));
                GameState::Port
            }
            _ => unreachable!("unsupported port capture scene: {scene}"),
        }
    }
}
