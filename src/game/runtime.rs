//! Small coordinator helpers shared by game action and capture modules.

use super::Game;
use crate::save;

impl Game {
    pub(crate) fn note(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    pub(crate) fn refresh_save_state(&mut self) {
        self.save_exists = save::has_save(&self.data);
    }
}
