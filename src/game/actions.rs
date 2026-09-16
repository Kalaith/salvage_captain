//! Action groups kept outside the top-level game coordinator.

use super::briefing;
use super::Game;
use crate::save;
use crate::state::{CargoStatus, GameSession, GameState, StateTransition};
use crate::ui::{self, UiAction};

impl Game {
    pub(super) fn apply_navigation_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::ContinueGame => {
                if self.resume_state != GameState::MainMenu {
                    self.state = self.resume_state;
                    self.note(crate::game::prompts::state_prompt(self.state));
                }
            }
            UiAction::NewGame => {
                self.data.clear_wreck_instances();
                self.session = GameSession::new(&self.data);
                self.session.discovery_seed = macroquad::miniquad::date::now().to_bits();
                self.port_selected_module = Some("engine_core".to_owned());
                self.selected_voyage_plan = crate::engine::VoyagePlan::Standard;
                self.wreck_selection = ui::site_cards::WreckSelection::default();
                self.return_elapsed = 0.0;
                self.port_hold_expanded = false;
                self.port_tab = crate::ui::port_panel::PortTab::default();
                self.port_stock_page = 0;
                self.port_loadouts_open = false;
                self.voyage_archive_open = false;
                self.voyage_archive = crate::ui::voyage_archive::ArchiveState::default();
                self.settings_open = false;
                self.transition(StateTransition::ToPort);
                self.note("Fresh ship, fresh debt. Shipyard online.");
            }
            UiAction::BackToMainMenu => {
                self.settings_open = false;
                self.transition(StateTransition::ToMainMenu);
                self.note("Main menu. Tap CONTINUE RUN to return to the current operation.");
            }
            UiAction::ExitGame => {
                self.settings_open = false;
                self.exit_requested = true;
            }
            UiAction::OpenSettings => {
                if self.state == GameState::Pause {
                    self.settings_open = true;
                }
            }
            UiAction::CloseSettings => {
                self.settings_open = false;
            }
            UiAction::ToggleFullscreen => {
                self.settings.fullscreen = !self.settings.fullscreen;
                self.settings.apply_display();
                self.persist_settings("Fullscreen setting saved.");
            }
            UiAction::ToggleReducedMotion => {
                self.settings.reduced_motion = !self.settings.reduced_motion;
                self.settings.apply_effects();
                self.persist_settings("Motion setting saved.");
            }
            UiAction::GoToPort => {
                if matches!(
                    self.state,
                    GameState::Travel | GameState::SalvageWorkspace | GameState::CargoInventory
                ) || (self.state == GameState::Results && !self.session.returned.is_empty())
                {
                    self.note("Finish the current salvage run before returning to port.");
                } else {
                    self.voyage_archive_open = false;
                    self.transition(StateTransition::ToPort);
                    self.note("At the port. Shipyard ready.");
                }
            }
            UiAction::GoToSites => {
                self.transition(StateTransition::ToSiteSelection);
                self.note(crate::game::prompts::state_prompt(GameState::SiteSelection));
            }
            _ => return false,
        }
        true
    }

    pub(super) fn apply_briefing_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::Depart(_) | UiAction::DepartPrivate(_) | UiAction::DepartInsured(_) => {
                briefing::depart(self, action.clone())
            }
            UiAction::BuyReconnaissance(site_id) => {
                match self.session.buy_reconnaissance(site_id, &self.data) {
                    Ok(message) => self.note(message),
                    Err(error) => self.note(error),
                }
            }
            UiAction::WreckSelection(choice) => {
                if self.state == GameState::SiteSelection {
                    self.wreck_selection.apply(choice, &self.data);
                    self.wreck_selection
                        .normalize_board(&self.session, &self.data);
                }
            }
            UiAction::CycleVoyagePlan => briefing::cycle_plan(self),
            UiAction::CycleCrew => briefing::cycle_crew(self),
            UiAction::RestCrew => match self.session.rest_crew() {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::TrainCrew => match self.session.train_crew(&self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::UseFieldPowerCell => match self.session.use_field_power_cell() {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::CycleReturnPolicy => briefing::cycle_return_policy(self),
            UiAction::ContinueTravel => {
                if self.state == GameState::Travel {
                    self.transition(StateTransition::ToSalvageWorkspace);
                    self.note("Arrival confirmed. Let the scene breathe, then tap SCAN.");
                }
            }
            UiAction::ContinueReturn => {
                if self.state == GameState::ReturnTravel {
                    self.transition(StateTransition::ToResults);
                    self.note(crate::game::prompts::state_prompt(GameState::Results));
                }
            }
            _ => return false,
        }
        true
    }

    pub(super) fn apply_packing_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::BeginDrag(object_id) => {
                self.dragged_item = Some(object_id.clone());
                self.note("Tap an open grid cell or drag the footprint to move this cargo.");
            }
            UiAction::DropDragged(position, rotation) => {
                if let Some(object_id) = self.dragged_item.take() {
                    match self
                        .session
                        .place_cargo(&object_id, *position, *rotation, &self.data)
                    {
                        Ok(message) => self.note(message),
                        Err(error) => self.note(error),
                    }
                }
            }
            UiAction::CancelDrag => {
                self.dragged_item = None;
                self.note("Salvage drag cancelled.");
            }
            UiAction::Rotate(object_id) => match self.session.rotate_cargo(object_id, &self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::Discard(object_id) => {
                self.set_cargo_status(object_id, CargoStatus::Discarded)
            }
            UiAction::LeaveAll => match self.session.leave_all_pending() {
                Ok(()) => self.note("All unplaced salvage left behind. Tap RETURN WITH HAUL."),
                Err(error) => self.note(error),
            },
            UiAction::ReturnWithHaul => self.return_with_haul(),
            UiAction::ReturnToWorkspace => {
                if self.state == GameState::CargoInventory && self.session.expedition.is_some() {
                    self.transition(StateTransition::ToSalvageWorkspace);
                    self.note("Back at the wreck. Scan another section or choose a target.");
                }
            }
            _ => return false,
        }
        true
    }

    pub(super) fn return_with_haul(&mut self) {
        if !matches!(
            self.state,
            GameState::SalvageWorkspace | GameState::CargoInventory
        ) || self
            .workspace_extraction
            .as_ref()
            .is_some_and(|extraction| !extraction.resolved)
        {
            return;
        }
        match self.session.finish_packing(&self.data) {
            Ok(message) => {
                self.workspace_extraction = None;
                self.workspace_risk = None;
                self.workspace_placement_rotation = None;
                self.transition(StateTransition::ToReturnTravel);
                self.note(format!("{message} Tap DOCK NOW to enter the yard debrief."));
            }
            Err(error) => self.note(error),
        }
    }

    fn set_cargo_status(&mut self, object_id: &str, status: CargoStatus) {
        match self.session.set_cargo_status(object_id, status) {
            Ok(message) => self.note(message),
            Err(error) => self.note(error),
        }
    }

    pub(super) fn apply_disposition_action(&mut self, action: &UiAction) -> bool {
        if let UiAction::ManifestPage(next) = action {
            if self.state == GameState::Results && !self.voyage_archive_open {
                self.manifest_page.turn(*next, self.session.returned.len());
            } else if self.state == GameState::CargoInventory {
                self.manifest_page
                    .turn(*next, self.session.inventory_cargo().count());
            }
            return true;
        }
        let UiAction::Disposition(object_id, disposition) = action else {
            return false;
        };
        match self.session.dispose(object_id, *disposition, &self.data) {
            Ok(message) => {
                self.note(message);
                if self.session.returned.is_empty() {
                    self.transition(StateTransition::ToPort);
                    self.note("At the port. Shipyard ready.");
                }
            }
            Err(error) => self.note(error),
        }
        true
    }

    pub(super) fn apply_shipyard_selection_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::SelectPortModule(module_id) => {
                if self.data.modules.contains(module_id) {
                    self.port_selected_module = Some(module_id.clone());
                    self.port_tab = crate::ui::port_panel::PortTab::Equipment;
                    self.port_hold_expanded = false;
                    self.port_loadouts_open = false;
                    if let Some(module) = self.data.modules.get(module_id) {
                        let installed = self
                            .session
                            .ship_layout
                            .placements
                            .iter()
                            .any(|item| item.permanent && item.id == *module_id);
                        self.note(format!(
                            "{} {}. Read the mount detail in SHIPYARD.",
                            module.display_name,
                            if installed {
                                "selected"
                            } else {
                                "preview selected"
                            }
                        ));
                    }
                }
            }
            UiAction::TogglePortHold => {
                self.port_hold_expanded = !self.port_hold_expanded;
                self.port_tab = crate::ui::port_panel::PortTab::Hangar;
            }
            UiAction::ToggleLoadoutPanel => {
                if self.state == GameState::Port {
                    self.port_loadouts_open = !self.port_loadouts_open;
                    if self.port_loadouts_open {
                        self.port_tab = crate::ui::port_panel::PortTab::Equipment;
                        self.voyage_archive_open = false;
                    }
                }
            }
            UiAction::StoreLoadout(slot) => match self.session.store_loadout(*slot) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::ApplyLoadout(slot) => match self.session.apply_loadout(*slot, &self.data) {
                Ok(message) => {
                    self.port_loadouts_open = false;
                    self.port_selected_module = self
                        .session
                        .ship_layout
                        .placements
                        .iter()
                        .find(|item| item.permanent)
                        .map(|item| item.id.clone());
                    self.note(message);
                }
                Err(error) => self.note(error),
            },
            UiAction::ToggleVoyageArchive => {
                if matches!(self.state, GameState::Port | GameState::Results) {
                    if !self.voyage_archive_open {
                        self.voyage_archive = ui::voyage_archive::ArchiveState::default();
                    }
                    self.voyage_archive_open = !self.voyage_archive_open;
                }
            }
            UiAction::Archive(action) => {
                if matches!(self.state, GameState::Port | GameState::Results)
                    && self.voyage_archive_open
                {
                    self.voyage_archive.apply(*action, &self.session.voyage_log);
                }
            }
            _ => return false,
        }
        true
    }

    pub(super) fn apply_shipyard_service_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::RemoveModule(module_id) => {
                match self.session.remove_module(module_id, &self.data) {
                    Ok(message) => {
                        if self.port_selected_module.as_deref() == Some(module_id.as_str()) {
                            self.port_selected_module = self
                                .session
                                .ship_layout
                                .placements
                                .iter()
                                .find(|item| item.permanent)
                                .map(|item| item.id.clone());
                        }
                        self.note(message);
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::PurchaseModule(module_id) => {
                match self.session.purchase_module(module_id, &self.data) {
                    Ok(message) => {
                        self.port_selected_module = Some(module_id.clone());
                        self.port_tab = crate::ui::port_panel::PortTab::Equipment;
                        self.port_loadouts_open = false;
                        self.note(message);
                    }
                    Err(error) => self.note(error),
                }
            }
            UiAction::Refuel => match self.session.refuel(&self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::Repair => match self.session.repair(&self.data) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::RefineAlloy => match self
                .session
                .refine_resource(crate::engine::refinery::RefineryResource::Alloy, &self.data)
            {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            UiAction::RefineElectronics => match self.session.refine_resource(
                crate::engine::refinery::RefineryResource::Electronics,
                &self.data,
            ) {
                Ok(message) => self.note(message),
                Err(error) => self.note(error),
            },
            _ => return false,
        }
        true
    }

    pub(super) fn apply_save_load_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::Save => {
                if self.state != GameState::Port && self.resume_state != GameState::Port {
                    self.note("Save is available at the port checkpoint.");
                    return true;
                }
                match save::save_session(&self.session, &self.data) {
                    Ok(()) => {
                        self.refresh_save_state();
                        self.note("Safe checkpoint saved.");
                    }
                    Err(error) => self.note(format!("Save failed: {error}")),
                }
            }
            UiAction::Load => match save::load_session(&self.data) {
                Ok(session) => self.restore_session(session),
                Err(error) => self.note(format!("Load failed: {error}")),
            },
            _ => return false,
        }
        true
    }

    fn restore_session(&mut self, session: GameSession) {
        match session.resolved_data(&self.data) {
            Ok(data) => self.data = data,
            Err(error) => {
                self.note(format!("Load failed: {error}"));
                return;
            }
        }
        self.session = session;
        let restored_state = if let Some(expedition) = &self.session.expedition {
            if expedition.workspace_scanned {
                GameState::SalvageWorkspace
            } else {
                GameState::Travel
            }
        } else if !self.session.returned.is_empty() {
            GameState::Results
        } else {
            GameState::Port
        };
        self.state = restored_state;
        self.resume_state = restored_state;
        self.settings_open = false;
        self.dragged_item = None;
        self.travel_elapsed = 0.0;
        self.workspace_elapsed = 0.0;
        self.workspace_camera_shift = 1.0;
        self.workspace_arrival_flash = 0.0;
        self.workspace_log_open = false;
        self.target_details_open = false;
        self.transit_details_open = false;
        self.manifest_page = ui::decision_panel::navigation::ManifestPage::default();
        self.workspace_scan_elapsed = 0.0;
        self.workspace_selected_target = None;
        self.workspace_placement_rotation = None;
        self.workspace_extraction = None;
        self.workspace_risk = None;
        self.workspace_notice.clear();
        self.workspace_notice_warning = false;
        self.workspace_notice_timer = 0.0;
        self.port_selected_module = self
            .session
            .ship_layout
            .placements
            .iter()
            .find(|item| item.permanent)
            .map(|item| item.id.clone());
        self.selected_voyage_plan = self.session.briefing_voyage_plan;
        self.wreck_selection = ui::site_cards::WreckSelection::default();
        self.wreck_selection
            .normalize_board(&self.session, &self.data);
        self.port_hold_expanded = false;
        self.port_tab = crate::ui::port_panel::PortTab::default();
        self.port_stock_page = 0;
        self.port_loadouts_open = false;
        self.voyage_archive_open = false;
        self.voyage_archive = crate::ui::voyage_archive::ArchiveState::default();
        self.refresh_save_state();
        self.note(format!(
            "Safe checkpoint loaded. {}",
            crate::game::prompts::state_prompt(restored_state)
        ));
    }

    pub(super) fn apply_pause_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::TogglePause => {
                if self.state == GameState::Pause {
                    self.settings_open = false;
                    self.state = self.resume_state;
                    self.note(crate::game::prompts::state_prompt(self.state));
                } else {
                    self.resume_state = self.state;
                    self.transition(StateTransition::ToPause);
                }
            }
            UiAction::ToggleStats => self.debug.toggle(),
            _ => return false,
        }
        true
    }
}
