//! Small coordinator helpers shared by game action and capture modules.

use super::Game;
use crate::save;
use crate::state::{GameState, StateTransition};
use crate::ui;

impl Game {
    pub(crate) fn note(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    pub(crate) fn refresh_save_state(&mut self) {
        self.save_exists = save::has_save(&self.data);
    }

    pub(super) fn transition(&mut self, transition: StateTransition) {
        if transition == StateTransition::ToPause {
            if self.state != GameState::Pause {
                self.resume_state = self.state;
            }
            self.state = GameState::Pause;
            return;
        }
        self.state = match transition {
            StateTransition::ToMainMenu => GameState::MainMenu,
            StateTransition::ToPort => GameState::Port,
            StateTransition::ToSiteSelection => GameState::SiteSelection,
            StateTransition::ToTravel => GameState::Travel,
            StateTransition::ToSalvageWorkspace => GameState::SalvageWorkspace,
            StateTransition::ToPacking => GameState::SalvagePacking,
            StateTransition::ToReturnTravel => GameState::ReturnTravel,
            StateTransition::ToResults => GameState::Results,
            StateTransition::ToPause => GameState::Pause,
        };
        self.dragged_item = None;
        if self.state != GameState::Port {
            self.port_service_open = false;
            self.port_loadouts_open = false;
            self.voyage_archive_open = false;
            self.voyage_archive_offset = 0;
            self.voyage_archive_filter = ui::voyage_archive::ArchiveFilter::All;
        }
        match self.state {
            GameState::Travel => {
                self.travel_elapsed = 0.0;
                self.workspace_extraction = None;
                self.workspace_risk = None;
                self.workspace_selected_target = None;
                self.workspace_notice_warning = false;
            }
            GameState::ReturnTravel => {
                self.return_elapsed = 0.0;
            }
            GameState::SalvageWorkspace => {
                self.workspace_elapsed = 0.0;
                self.workspace_camera_shift = 1.0;
                self.workspace_arrival_flash = 0.0;
                self.workspace_log_open = false;
                self.workspace_scan_elapsed = 0.0;
                self.workspace_extraction = None;
                self.workspace_risk = None;
                self.workspace_selected_target = None;
                self.workspace_notice.clear();
                self.workspace_notice_warning = false;
                self.workspace_notice_timer = 0.0;
            }
            GameState::Port
            | GameState::MainMenu
            | GameState::SiteSelection
            | GameState::SalvagePacking
            | GameState::Results
            | GameState::Pause => {}
        }
    }
}
