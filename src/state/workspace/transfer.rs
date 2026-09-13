//! Commit a hold destination before spending power or removing a wreck target.

use super::*;
use crate::data::GridPosition;

impl GameSession {
    pub fn workspace_transfer(&self) -> Option<&WorkspaceTransfer> {
        self.expedition.as_ref()?.workspace_transfer.as_ref()
    }

    pub fn validate_workspace_placement(
        &self,
        target_id: &str,
        position: GridPosition,
        rotation: u8,
        data: &GameData,
    ) -> Result<(), String> {
        let target = self.workspace_target(target_id, data)?;
        if !target.rotatable && !rotation.is_multiple_of(2) {
            return Err(format!("{} cannot rotate", target.display_name));
        }
        if TransferMode::from_target(target).uses_external_rig() {
            if self.external_cargo_count(data, None) >= self.external_capacity(data) {
                return Err(data.salvage_ui.clamps_full.clone());
            }
        } else if self.internal_cargo_count(data, None) >= self.internal_cargo_capacity() {
            return Err(data.salvage_ui.hold_full.clone());
        }
        self.ship_layout
            .can_place(
                &super::super::cargo_layout_id(target_id),
                target.footprint,
                position,
                rotation,
            )
            .map_err(|error| error.to_string())
    }

    pub fn begin_workspace_transfer(
        &mut self,
        target_id: &str,
        position: GridPosition,
        rotation: u8,
        data: &GameData,
    ) -> Result<String, String> {
        if self.workspace_transfer().is_some() {
            return Err(data.salvage_ui.transfer_busy.clone());
        }
        self.validate_workspace_placement(target_id, position, rotation, data)?;
        let message = self.reserve_workspace_energy(target_id, data)?;
        self.expedition
            .as_mut()
            .expect("energy reservation requires expedition")
            .workspace_transfer = Some(WorkspaceTransfer {
            target_id: target_id.to_owned(),
            position,
            rotation: rotation % 2,
        });
        Ok(message)
    }

    pub fn cancel_workspace_transfer(&mut self) {
        if let Some(expedition) = self.expedition.as_mut() {
            if let Some(transfer) = expedition.workspace_transfer.take() {
                self.record_workspace_event(
                    WorkspaceLogEvent::ExtractionCancelled,
                    Some(&transfer.target_id),
                );
            }
        }
    }
}
