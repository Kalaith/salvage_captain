//! Authoritative drone orders and support effects for the active workspace.

use super::{DroneDirective, GameSession, WorkspaceLogEntry, WorkspaceLogEvent};
use crate::data::GameData;

impl GameSession {
    pub fn workspace_drones_deployed(&self) -> bool {
        self.expedition
            .as_ref()
            .is_some_and(|expedition| expedition.drones_deployed)
    }

    pub fn workspace_drone_directive(&self) -> DroneDirective {
        self.expedition
            .as_ref()
            .map_or(DroneDirective::default(), |expedition| {
                expedition.drone_directive
            })
    }

    pub fn workspace_drone_support(&self, data: &GameData) -> i32 {
        let module_support = self.module_stats(data).drone_support;
        self.expedition
            .as_ref()
            .map_or(module_support, |expedition| {
                expedition.drone_directive.effective_support(module_support)
            })
    }

    pub fn cycle_drone_directive(&mut self, data: &GameData) -> Result<String, String> {
        let module_support = self.module_stats(data).drone_support;
        if module_support <= 0 {
            return Err(
                "Requires Drone Bay capability before issuing a drone directive.".to_owned(),
            );
        }
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        let was_deployed = expedition.drones_deployed;
        let next = expedition.drone_directive.next();
        expedition.drone_directive = next;
        if expedition.workspace_scanned {
            expedition.drones_deployed = next.deploys_drones();
        }
        let site_id = expedition.site_id.clone();
        let section_id = expedition.workspace_section.clone();
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            let sequence = progress.operation_log.len() as u32 + 1;
            progress
                .operation_log
                .push(WorkspaceLogEntry::with_drone_directive(
                    sequence,
                    WorkspaceLogEvent::DroneDirectiveChanged,
                    Some(section_id.as_str()),
                    next,
                ));
            if expedition.workspace_scanned && was_deployed != next.deploys_drones() {
                let sequence = progress.operation_log.len() as u32 + 1;
                let event = if next.deploys_drones() {
                    WorkspaceLogEvent::DronesDeployed
                } else {
                    WorkspaceLogEvent::DronesRecalled
                };
                progress.operation_log.push(WorkspaceLogEntry::new(
                    sequence,
                    event,
                    Some(section_id.as_str()),
                    None,
                ));
            }
        }
        Ok(format!(
            "Drone directive: {}. {}",
            next.label(),
            next.description()
        ))
    }
}
