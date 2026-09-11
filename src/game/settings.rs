//! Settings persistence kept outside the main action coordinator.

use super::Game;

impl Game {
    pub(super) fn persist_settings(&mut self, success_message: &str) {
        self.settings.sanitize();
        match self.settings.save(&self.data.config.game_name) {
            Ok(()) => self.note(success_message),
            Err(error) => self.note(format!("Settings save failed: {error}")),
        }
    }
}
