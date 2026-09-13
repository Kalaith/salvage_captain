//! Gameplay intents for the scan, target, stabilization, and extraction loop.

use super::Game;
use crate::state::workspace::TransferMode;
use crate::state::GameState;
use crate::ui;
use crate::ui::UiAction;

impl Game {
    pub(crate) fn apply_workspace_action(&mut self, action: UiAction) {
        if self.state != GameState::SalvageWorkspace || self.blocks_workspace_action(&action) {
            return;
        }
        match action {
            UiAction::Scan => self.apply_scan_action(),
            UiAction::PowerCycle => self.apply_power_cycle_action(),
            UiAction::CycleDroneDirective => self.apply_drone_action(),
            UiAction::SelectSection(section_id) => self.apply_section_action(section_id),
            UiAction::SelectTarget(target_id) => self.apply_target_action(target_id),
            UiAction::Stabilize(target_id) => self.apply_stabilize_action(target_id),
            UiAction::Extract(target_id) => self.apply_extract_action(target_id),
            UiAction::AbandonTarget => self.abandon_workspace_target(),
            UiAction::CancelExtraction => self.cancel_workspace_extraction(),
            UiAction::ReturnFromWorkspace => self.return_from_workspace(),
            _ => unreachable!("non-workspace action routed to workspace handler"),
        }
    }

    fn blocks_workspace_action(&self, action: &UiAction) -> bool {
        self.workspace_extraction.is_some()
            && matches!(
                action,
                UiAction::SelectTarget(_)
                    | UiAction::Scan
                    | UiAction::Stabilize(_)
                    | UiAction::AbandonTarget
            )
    }

    fn apply_scan_action(&mut self) {
        match self.session.scan_workspace(&self.data) {
            Ok(message) => {
                self.workspace_scan_elapsed = 0.001;
                self.workspace_selected_target = None;
                self.workspace_risk = None;
                self.note(format!("{message} Tap a bracketed target to inspect it."));
            }
            Err(error) => self.workspace_error(error),
        }
    }

    fn apply_power_cycle_action(&mut self) {
        if self.workspace_extraction.is_some() {
            self.note("Finish or cancel the active extraction before cycling field power.");
            return;
        }
        match self.session.power_cycle_workspace(&self.data) {
            Ok(message) => self.note(message),
            Err(error) => self.workspace_error(error),
        }
    }

    fn apply_drone_action(&mut self) {
        if self.workspace_extraction.is_some() {
            self.note("Finish or cancel the active extraction before changing drone orders.");
            return;
        }
        match self.session.cycle_drone_directive(&self.data) {
            Ok(message) => {
                self.refresh_selected_target_risk();
                self.note(message);
            }
            Err(error) => self.workspace_error(error),
        }
    }

    fn refresh_selected_target_risk(&mut self) {
        if let Some(target_id) = self.workspace_selected_target.clone() {
            self.refresh_target_risk(&target_id);
        } else {
            self.workspace_risk = None;
        }
    }

    fn refresh_target_risk(&mut self, target_id: &str) {
        self.workspace_risk = self
            .session
            .workspace_risk_preview(target_id, &self.data)
            .ok();
    }

    fn apply_section_action(&mut self, section_id: String) {
        if self.workspace_extraction.is_some() {
            self.note("Finish or cancel the active extraction before moving the camera.");
            return;
        }
        let moving_camera = self
            .session
            .expedition
            .as_ref()
            .is_some_and(|expedition| expedition.workspace_section != section_id);
        match self
            .session
            .switch_workspace_section(&section_id, &self.data)
        {
            Ok(message) => self.finish_section_switch(message, moving_camera),
            Err(error) => self.workspace_error(error),
        }
    }

    fn finish_section_switch(&mut self, message: String, moving_camera: bool) {
        if moving_camera {
            self.workspace_camera_shift = 0.0;
            self.workspace_arrival_flash = 0.0;
            self.workspace_elapsed = 0.0;
        }
        self.workspace_selected_target = None;
        self.workspace_scan_elapsed = 0.0;
        self.workspace_risk = None;
        self.note(ui::salvage_scene::section_switch_prompt(
            &message,
            moving_camera,
        ));
    }

    fn apply_target_action(&mut self, target_id: String) {
        if !self.session.target_is_revealed(&target_id)
            || self.session.target_is_removed(&target_id)
        {
            return;
        }
        self.workspace_selected_target = Some(target_id.clone());
        self.refresh_selected_target_risk();
        let command = self.target_command_label(&target_id);
        let command = if self.target_can_power_cycle(&target_id) {
            "POWER CYCLE"
        } else {
            command
        };
        self.note(format!(
            "Target selected. Tap {command}, then choose its hold position."
        ));
    }

    fn target_command_label(&self, target_id: &str) -> &'static str {
        self.data
            .salvage_objects
            .get(target_id)
            .map_or("EXTRACT", |target| {
                TransferMode::from_target(target).command_label()
            })
    }

    fn target_can_power_cycle(&self, target_id: &str) -> bool {
        self.session
            .extraction_block_reason(target_id, &self.data)
            .ok()
            .flatten()
            .is_some_and(|reason| {
                reason.starts_with("Power reserve insufficient")
                    && self.session.can_power_cycle_workspace(&self.data)
            })
    }

    fn apply_stabilize_action(&mut self, target_id: String) {
        match self
            .session
            .stabilize_workspace_target(&target_id, &self.data)
        {
            Ok(message) => {
                self.refresh_target_risk(&target_id);
                self.note(message);
            }
            Err(error) => self.workspace_error(error),
        }
    }

    fn apply_extract_action(&mut self, target_id: String) {
        if self.workspace_extraction.is_some() {
            return;
        }
        match self.session.extraction_block_reason(&target_id, &self.data) {
            Ok(None) => {
                self.workspace_selected_target = Some(target_id);
                self.workspace_placement_rotation = Some(0);
                self.target_details_open = false;
                self.note(self.data.salvage_ui.choose_destination.clone());
            }
            Ok(Some(reason)) => self.note(reason),
            Err(error) => self.workspace_error(error),
        }
    }

    fn abandon_workspace_target(&mut self) {
        self.workspace_selected_target = None;
        self.workspace_risk = None;
        self.note("Target abandoned. The wreck remains stable.");
    }

    fn cancel_workspace_extraction(&mut self) {
        if !self
            .workspace_extraction
            .as_ref()
            .is_some_and(|extraction| !extraction.resolved)
        {
            return;
        }
        self.session.cancel_workspace_transfer();
        self.workspace_extraction = None;
        self.workspace_risk = None;
        let command = self
            .workspace_selected_target
            .as_deref()
            .map_or("EXTRACT", |target_id| self.target_command_label(target_id));
        self.note(format!(
            "Extraction cancelled. Tap {command} to try again or RETURN TO HOLD."
        ));
    }

    fn return_from_workspace(&mut self) {
        if !self
            .workspace_extraction
            .as_ref()
            .is_none_or(|extraction| extraction.resolved)
        {
            return;
        }
        self.workspace_extraction = None;
        self.workspace_risk = None;
        self.transition(crate::state::StateTransition::ToPacking);
        self.note("Hold review. Cargo is already secured; review the haul or return to the wreck.");
    }
}
