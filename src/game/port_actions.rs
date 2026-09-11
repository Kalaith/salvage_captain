//! Port-only service and field-supply actions kept outside the main coordinator.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_port_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::Service(plan) => {
                match self.session.service(*plan, &self.data) {
                    Ok(message) => {
                        self.port_service_open = false;
                        self.note(message);
                    }
                    Err(error) => self.note(error),
                }
                true
            }
            UiAction::BuyFieldPowerCell => {
                match self.session.buy_field_power_cell() {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
                true
            }
            UiAction::FabricateFieldPowerCell => {
                match self.session.fabricate_field_power_cell() {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
                true
            }
            _ => false,
        }
    }
}
