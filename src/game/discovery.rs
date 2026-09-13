//! Coordinates discovery intents and safe port replenishment.

use super::Game;
use crate::state::GameState;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_discovery_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::DiscoverWreck(kind) => {
                if self.state != GameState::SiteSelection {
                    return true;
                }
                match self.session.discover_wreck(*kind, &mut self.data) {
                    Ok(id) => {
                        self.wreck_selection.archived = false;
                        self.wreck_selection.page = 0;
                        self.wreck_selection.site_id = Some(id.clone());
                        self.wreck_selection.details_open = false;
                        self.wreck_selection.private_haul = false;
                        self.wreck_selection.insured = false;
                        let name = self
                            .data
                            .sites
                            .get(&id)
                            .map_or(id.as_str(), |s| s.display_name.as_str());
                        self.note(self.data.discovery.copy.found.replace("{name}", name));
                    }
                    Err(reason) => self.note(reason),
                }
            }
            UiAction::WreckBoard(action) => {
                if self.state == GameState::SiteSelection {
                    self.wreck_selection
                        .apply_board(*action, &self.session, &self.data);
                    self.note(self.data.selection_ui.instruction.clone());
                }
            }
            _ => return false,
        }
        true
    }

    pub(super) fn refresh_wreck_board(&mut self) {
        match self.session.replenish_wrecks(&mut self.data) {
            Ok(Some(id)) => {
                self.wreck_selection.archived = false;
                self.wreck_selection.page = 0;
                self.wreck_selection.site_id = Some(id);
            }
            Ok(None) => {}
            Err(error) => self.note(error),
        }
        self.wreck_selection
            .normalize_board(&self.session, &self.data);
    }
}
