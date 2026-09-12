//! Port-only service and field-supply actions kept outside the main coordinator.

use super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_port_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::SelectPortTab(tab) => {
                if self.state == crate::state::GameState::Port {
                    self.port_tab = if self.port_tab == *tab {
                        crate::ui::port_panel::PortTab::Hangar
                    } else {
                        *tab
                    };
                    self.port_hold_expanded = false;
                    self.port_loadouts_open = false;
                }
                true
            }
            UiAction::PortStockPage(next) => {
                if self.state == crate::state::GameState::Port {
                    self.port_stock_page = crate::ui::port_panel::stock_page(
                        self.port_stock_page,
                        *next,
                        self.data.modules.iter().count(),
                    );
                }
                true
            }
            UiAction::Service(plan) => {
                match self.session.service(*plan, &self.data) {
                    Ok(message) => {
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
            UiAction::UpgradeCargoBay => {
                match self.session.purchase_cargo_bay_upgrade() {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
                true
            }
            _ => false,
        }
    }
}
