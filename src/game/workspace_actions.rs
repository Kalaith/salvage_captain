//! Gameplay intents for the scan, target, stabilization, and extraction loop.

use super::Game;
use crate::state::workspace::TransferMode;
use crate::state::GameState;
use crate::ui;
use crate::ui::UiAction;

impl Game {
    pub(crate) fn apply_workspace_action(&mut self, action: UiAction) {
        if self.state != GameState::SalvageWorkspace {
            return;
        }
        if self.workspace_extraction.is_some()
            && matches!(
                action,
                UiAction::SelectTarget(_)
                    | UiAction::Scan
                    | UiAction::Stabilize(_)
                    | UiAction::AbandonTarget
            )
        {
            return;
        }
        match action {
            UiAction::Scan => match self.session.scan_workspace(&self.data) {
                Ok(message) => {
                    self.workspace_scan_elapsed = 0.001;
                    self.workspace_selected_target = None;
                    self.workspace_risk = None;
                    self.note(format!("{message} Tap a bracketed target to inspect it."));
                }
                Err(error) => self.workspace_error(error),
            },
            UiAction::PowerCycle => {
                if self.workspace_extraction.is_some() {
                    self.note("Finish or cancel the active extraction before cycling field power.");
                    return;
                }
                match self.session.power_cycle_workspace(&self.data) {
                    Ok(message) => self.note(message),
                    Err(error) => self.workspace_error(error),
                }
            }
            UiAction::CycleDroneDirective => {
                if self.workspace_extraction.is_some() {
                    self.note(
                        "Finish or cancel the active extraction before changing drone orders.",
                    );
                    return;
                }
                match self.session.cycle_drone_directive(&self.data) {
                    Ok(message) => {
                        self.workspace_risk =
                            self.workspace_selected_target
                                .as_deref()
                                .and_then(|target_id| {
                                    self.session
                                        .workspace_risk_preview(target_id, &self.data)
                                        .ok()
                                });
                        self.note(message);
                    }
                    Err(error) => self.workspace_error(error),
                }
            }
            UiAction::SelectSection(section_id) => {
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
                    Ok(message) => {
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
                    Err(error) => self.workspace_error(error),
                }
            }
            UiAction::SelectTarget(target_id) => {
                if self.session.target_is_revealed(&target_id)
                    && !self.session.target_is_removed(&target_id)
                {
                    self.workspace_selected_target = Some(target_id.clone());
                    self.workspace_risk = self
                        .session
                        .workspace_risk_preview(&target_id, &self.data)
                        .ok();
                    let command = self
                        .data
                        .salvage_objects
                        .get(&target_id)
                        .map_or("EXTRACT", |target| {
                            TransferMode::from_target(target).command_label()
                        });
                    let command = if self
                        .session
                        .extraction_block_reason(&target_id, &self.data)
                        .ok()
                        .flatten()
                        .is_some_and(|reason| {
                            reason.starts_with("Power reserve insufficient")
                                && self.session.can_power_cycle_workspace(&self.data)
                        }) {
                        "POWER CYCLE"
                    } else {
                        command
                    };
                    self.note(format!(
                        "Target selected. Tap {command}, then choose its hold position."
                    ));
                }
            }
            UiAction::Stabilize(target_id) => {
                match self
                    .session
                    .stabilize_workspace_target(&target_id, &self.data)
                {
                    Ok(message) => {
                        self.workspace_risk = self
                            .session
                            .workspace_risk_preview(&target_id, &self.data)
                            .ok();
                        self.note(message);
                    }
                    Err(error) => self.workspace_error(error),
                }
            }
            UiAction::Extract(target_id) => {
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
            UiAction::AbandonTarget => {
                self.workspace_selected_target = None;
                self.workspace_risk = None;
                self.note("Target abandoned. The wreck remains stable.");
            }
            UiAction::CancelExtraction => {
                if self
                    .workspace_extraction
                    .as_ref()
                    .is_some_and(|extraction| !extraction.resolved)
                {
                    self.session.cancel_workspace_transfer();
                    self.workspace_extraction = None;
                    self.workspace_risk = None;
                    let command = self
                        .workspace_selected_target
                        .as_deref()
                        .and_then(|target_id| self.data.salvage_objects.get(target_id))
                        .map_or("EXTRACT", |target| {
                            TransferMode::from_target(target).command_label()
                        });
                    self.note(format!(
                        "Extraction cancelled. Tap {command} to try again or RETURN TO HOLD."
                    ));
                }
            }
            UiAction::ReturnFromWorkspace => {
                if self
                    .workspace_extraction
                    .as_ref()
                    .is_none_or(|extraction| extraction.resolved)
                {
                    self.workspace_extraction = None;
                    self.workspace_risk = None;
                    self.transition(crate::state::StateTransition::ToPacking);
                    self.note("Hold review. Cargo is already secured; review the haul or return to the wreck.");
                }
            }
            _ => unreachable!("non-workspace action routed to workspace handler"),
        }
    }
}
