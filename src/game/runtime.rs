//! Small coordinator helpers shared by game action and capture modules.

use super::Game;
use crate::engine::WorkspaceOutcome;
use crate::save;
use crate::state::{GameState, StateTransition};
use crate::ui;

impl Game {
    pub(super) fn update_runtime(&mut self, dt: f32) {
        match self.state {
            GameState::Travel => {
                self.travel_elapsed =
                    (self.travel_elapsed + dt).min(ui::travel::TRAVEL_DURATION_SECONDS)
            }
            GameState::ReturnTravel => {
                self.return_elapsed = (self.return_elapsed + dt)
                    .min(ui::return_travel::RETURN_TRAVEL_DURATION_SECONDS)
            }
            GameState::SalvageWorkspace => self.update_workspace_runtime(dt),
            _ => {}
        }
    }

    fn update_workspace_runtime(&mut self, dt: f32) {
        self.update_workspace_motion(dt);
        self.update_workspace_scan(dt);
        self.workspace_notice_timer = (self.workspace_notice_timer - dt).max(0.0);
        let (completed_target, clear_extraction) = self.advance_extraction(dt);
        if let Some(target_id) = completed_target {
            self.resolve_extraction(&target_id);
        }
        if clear_extraction {
            self.workspace_extraction = None;
        }
    }

    fn update_workspace_motion(&mut self, dt: f32) {
        let was_shifting = self.workspace_camera_shift < 1.0;
        let was_ready = ui::salvage_scene::section_arrival_ready(
            self.workspace_camera_shift,
            self.workspace_elapsed,
        );
        self.workspace_elapsed += dt;
        self.workspace_camera_shift =
            (self.workspace_camera_shift + dt / ui::salvage_scene::SECTION_SHIFT_SECONDS).min(1.0);
        self.workspace_arrival_flash = (self.workspace_arrival_flash
            - dt / ui::salvage_scene::SECTION_ARRIVAL_FLASH_SECONDS)
            .max(0.0);
        if was_shifting && self.workspace_camera_shift >= 1.0 {
            self.workspace_arrival_flash = 1.0;
        }
        let is_ready = ui::salvage_scene::section_arrival_ready(
            self.workspace_camera_shift,
            self.workspace_elapsed,
        );
        if !was_ready
            && is_ready
            && self
                .session
                .expedition
                .as_ref()
                .is_some_and(|expedition| !expedition.workspace_scanned)
        {
            self.note(ui::salvage_scene::SECTION_SETTLED_PROMPT);
        }
    }

    fn update_workspace_scan(&mut self, dt: f32) {
        if self.workspace_scan_elapsed > 0.0 {
            self.workspace_scan_elapsed += dt;
            if self.workspace_scan_elapsed >= 0.9 {
                self.workspace_scan_elapsed = 0.0;
            }
        }
    }

    fn advance_extraction(&mut self, dt: f32) -> (Option<String>, bool) {
        let mut completed_target = None;
        let mut clear_extraction = false;
        if let Some(extraction) = self.workspace_extraction.as_mut() {
            extraction.elapsed += dt;
            if extraction.elapsed >= extraction.duration && !extraction.resolved {
                extraction.resolved = true;
                completed_target = Some(extraction.target_id.clone());
            }
            if extraction.resolved && extraction.elapsed > extraction.duration + 0.55 {
                clear_extraction = true;
            }
        }
        (completed_target, clear_extraction)
    }

    fn resolve_extraction(&mut self, target_id: &str) {
        let resolution = self.workspace_risk.clone();
        self.workspace_notice_warning = resolution.as_ref().is_some_and(|report| {
            matches!(
                report.outcome,
                WorkspaceOutcome::DamagedHull | WorkspaceOutcome::LostTarget
            )
        });
        let resolution_explanation = resolution.as_ref().map(|report| report.explanation.clone());
        let result = match resolution.as_ref().map(|report| report.outcome.clone()) {
            Some(WorkspaceOutcome::LostTarget) => {
                self.session.lose_workspace_target(target_id, &self.data)
            }
            Some(WorkspaceOutcome::DamagedHull) => self
                .session
                .recover_workspace_target(target_id, &self.data)
                .map(|message| {
                    format!(
                        "{message}{}",
                        self.session.apply_workspace_damage(&self.data)
                    )
                }),
            _ => self.session.recover_workspace_target(target_id, &self.data),
        };
        match result {
            Ok(message) => self.show_extraction_result(target_id, message, resolution_explanation),
            Err(error) => self.workspace_error(error),
        }
    }

    fn show_extraction_result(
        &mut self,
        target_id: &str,
        message: String,
        resolution_explanation: Option<String>,
    ) {
        let display_name = self
            .data
            .salvage_objects
            .get(target_id)
            .map_or(target_id, |target| {
                if target.workspace_name.is_empty() {
                    target.display_name.as_str()
                } else {
                    target.workspace_name.as_str()
                }
            });
        let lost = self
            .workspace_risk
            .as_ref()
            .is_some_and(|report| report.outcome == WorkspaceOutcome::LostTarget);
        self.workspace_notice = format!(
            "{} / {}",
            display_name,
            if lost { "LOST" } else { "RECOVERED" }
        );
        self.workspace_notice_timer = 5.0;
        self.note(match resolution_explanation {
            Some(explanation) => format!("{message} {explanation}"),
            None => message,
        });
    }

    pub(crate) fn workspace_error(&mut self, message: String) {
        self.workspace_notice = message.clone();
        self.workspace_notice_warning = true;
        self.workspace_notice_timer = 5.0;
        self.note(message);
    }

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
                self.target_details_open = false;
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
