//! Deterministic port and maintenance scenes for UI verification.

use super::Game;
use crate::state::GameState;

impl Game {
    pub(super) fn capture_port_scene(&mut self, scene: &str) -> GameState {
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
