//! Hold selection commits the target destination and starts the timed pull.

use super::Game;
use crate::state::workspace::{ExtractionRuntime, TransferMode};
use crate::state::GameState;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_workspace_placement_action(&mut self, action: &UiAction) -> bool {
        if !matches!(
            action,
            UiAction::PlaceWorkspaceTarget(_)
                | UiAction::RotateWorkspacePlacement
                | UiAction::CancelWorkspacePlacement
        ) {
            return false;
        }
        if self.state != GameState::SalvageWorkspace || self.workspace_extraction.is_some() {
            return true;
        }
        let Some(rotation) = self.workspace_placement_rotation else {
            return true;
        };
        let Some(target_id) = self.workspace_selected_target.clone() else {
            return true;
        };
        match action {
            UiAction::CancelWorkspacePlacement => {
                self.workspace_placement_rotation = None;
                self.note(self.data.salvage_ui.select_hint.clone());
            }
            UiAction::RotateWorkspacePlacement => {
                if self
                    .data
                    .salvage_objects
                    .get(&target_id)
                    .is_some_and(|target| target.rotatable)
                {
                    self.workspace_placement_rotation = Some((rotation + 1) % 2);
                }
            }
            UiAction::PlaceWorkspaceTarget(position) => {
                match self
                    .session
                    .begin_workspace_transfer(&target_id, *position, rotation, &self.data)
                {
                    Ok(power_message) => {
                        let duration = self
                            .session
                            .extraction_duration(&target_id, &self.data)
                            .unwrap_or(4.0);
                        self.workspace_risk = self
                            .session
                            .workspace_risk_preview(&target_id, &self.data)
                            .ok();
                        let mode = self
                            .data
                            .salvage_objects
                            .get(&target_id)
                            .map_or(TransferMode::InternalCargo, TransferMode::from_target);
                        self.workspace_extraction =
                            Some(ExtractionRuntime::new(target_id, duration));
                        self.workspace_placement_rotation = None;
                        self.note(format!("{} {}", mode.engaged_message(), power_message));
                    }
                    Err(error) => self.workspace_error(error),
                }
            }
            _ => unreachable!("placement actions matched above"),
        }
        true
    }
}
