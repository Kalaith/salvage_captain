//! Transient briefing choices owned by Game, separate from an active expedition.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionAction {
    Select(String),
    TogglePrivate,
    ToggleInsurance,
    ToggleDetails,
}

#[derive(Debug, Clone, Default)]
pub struct WreckSelection {
    pub page: usize,
    pub archived: bool,
    pub site_id: Option<String>,
    pub private_haul: bool,
    pub insured: bool,
    pub details_open: bool,
}

impl WreckSelection {
    pub fn selected<'a>(&self, data: &'a GameData) -> Option<&'a crate::data::SiteData> {
        self.site_id
            .as_deref()
            .and_then(|id| data.sites.get(id))
            .or_else(|| data.ordered_sites().into_iter().next())
    }

    pub fn apply(&mut self, action: &SelectionAction, data: &GameData) {
        match action {
            SelectionAction::Select(id) if data.sites.get(id).is_some() => {
                self.site_id = Some(id.clone());
                self.details_open = false;
            }
            SelectionAction::TogglePrivate => {
                self.private_haul = !self.private_haul;
                self.insured = false;
            }
            SelectionAction::ToggleInsurance if !self.private_haul => self.insured = !self.insured,
            SelectionAction::ToggleDetails => self.details_open = !self.details_open,
            _ => {}
        }
    }

    pub fn departure_action(&self, data: &GameData) -> Option<UiAction> {
        let id = self.selected(data)?.id.clone();
        Some(self.departure_to(&id))
    }

    pub fn departure_to(&self, id: &str) -> UiAction {
        if self.private_haul {
            UiAction::DepartPrivate(id.to_owned())
        } else if self.insured {
            UiAction::DepartInsured(id.to_owned())
        } else {
            UiAction::Depart(id.to_owned())
        }
    }
}
