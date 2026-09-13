//! Scanning, navigation, stabilization, and extraction operations.

use super::*;

impl GameSession {
    pub fn scan_workspace(&mut self, data: &GameData) -> Result<String, String> {
        if let Some((remaining, capacity)) = self.workspace_energy() {
            if self
                .expedition
                .as_ref()
                .is_some_and(|expedition| expedition.workspace_scanned)
            {
                return Ok(format!(
                    "Section already scanned. Power {remaining}/{capacity} remains available."
                ));
            }
        }
        let scan_cost = data.config.workspace_scan_energy_cost;
        self.spend_workspace_energy(scan_cost)?;
        let module_support = self.module_stats(data).drone_support;
        let drone_directive = self.workspace_drone_directive();
        let scan_profile =
            WorkspaceScanProfile::from_capability(self.has_capability("scanner_array", data));
        let drones_were_deployed = self.workspace_drones_deployed();
        let (site_id, section_id, target_ids) = {
            let site = self.workspace_site(data)?;
            let section = self.workspace_section(data)?;
            (
                site.id.clone(),
                section.id.clone(),
                section.candidate_targets.clone(),
            )
        };
        let removed = self
            .site_progress
            .get(&site_id)
            .map(|progress| progress.removed_targets.clone())
            .unwrap_or_default();
        self.record_survey_notes(&site_id, &section_id, &target_ids, data);
        let visible: Vec<String> = target_ids
            .iter()
            .filter(|target| !removed.iter().any(|removed_id| removed_id == *target))
            .cloned()
            .collect();
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_scanned = true;
        expedition.revealed_targets = visible.clone();
        expedition.scan_profile = scan_profile;
        expedition.drones_deployed = module_support > 0 && drone_directive.deploys_drones();
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.discovered_sections.contains(&section_id) {
                progress.discovered_sections.push(section_id.clone());
            }
        }
        self.append_workspace_log(
            &site_id,
            WorkspaceLogEvent::SectionScanned,
            Some(section_id.as_str()),
            None,
        );
        if module_support > 0 && drone_directive.deploys_drones() && !drones_were_deployed {
            self.append_workspace_log(
                &site_id,
                WorkspaceLogEvent::DronesDeployed,
                Some(section_id.as_str()),
                None,
            );
        }
        let (recovered, total_targets) = self.site_recovery_summary(&site_id, data);
        let remaining_targets = total_targets.saturating_sub(recovered);
        let drone_notice = if module_support > 0 {
            match drone_directive {
                DroneDirective::Standby => " Drone bay on standby; no field assist is active.",
                DroneDirective::Survey => {
                    " Survey net deployed; hazard mitigation is prioritized over speed."
                }
                DroneDirective::PullSupport => " Survey drones deployed; pull support is active.",
            }
        } else {
            ""
        };
        Ok(format!(
            "{} complete: {} target(s) remain readable. Power {}/{}. Site recovery is {}/{}; {} remain.{}",
            scan_profile.result_label(),
            visible.len(),
            self.workspace_energy()
                .map_or(0, |(remaining, _)| remaining),
            self.workspace_energy().map_or(0, |(_, capacity)| capacity),
            recovered,
            total_targets,
            remaining_targets,
            drone_notice
        ))
    }

    pub fn switch_workspace_section(
        &mut self,
        section_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let site = self.workspace_site(data)?;
        let current = self.workspace_section(data)?.id.clone();
        let section = site
            .sections
            .iter()
            .find(|section| section.id == section_id)
            .ok_or_else(|| format!("unknown wreck section '{section_id}'"))?;
        let required_capability = section.required_capability.clone();
        let arrival_text = section.arrival_text.clone();
        let section_targets = section.candidate_targets.clone();
        let known_section = self.site_progress.get(&site.id).is_some_and(|progress| {
            progress
                .discovered_sections
                .iter()
                .any(|discovered| discovered == &section.id)
        });
        if section.id != current
            && !self
                .workspace_section(data)?
                .connected_sections
                .iter()
                .any(|neighbor| neighbor == section_id)
        {
            return Err("that section is not connected to the current frame".to_owned());
        }
        if let Some(capability) = required_capability {
            if !self.has_capability(&capability, data) {
                return Err(format!(
                    "Requires {} capability to enter this section.",
                    capability_label(&capability)
                ));
            }
        }
        let visible_targets = if known_section {
            let removed = self
                .site_progress
                .get(&site.id)
                .map(|progress| progress.removed_targets.clone())
                .unwrap_or_default();
            section_targets
                .into_iter()
                .filter(|target| !removed.iter().any(|removed_id| removed_id == target))
                .collect()
        } else {
            Vec::new()
        };
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_section = section.id.clone();
        expedition.workspace_scanned = known_section;
        expedition.revealed_targets = visible_targets;
        let entered_new_section = section.id != current;
        let entered_section_id = section.id.clone();
        let briefing = if arrival_text.is_empty() {
            String::new()
        } else {
            format!(" {}", arrival_text)
        };
        let scan_instruction = if known_section {
            " Known scan restored; targets are readable."
        } else {
            " Scan the section before working."
        };
        if entered_new_section {
            self.append_workspace_log(
                &site.id,
                WorkspaceLogEvent::EnteredSection,
                Some(entered_section_id.as_str()),
                None,
            );
        }
        Ok(format!(
            "Camera moved to {}.{}{}",
            section.display_name, briefing, scan_instruction
        ))
    }

    pub fn workspace_target<'a>(
        &self,
        target_id: &str,
        data: &'a GameData,
    ) -> Result<&'a SalvageObjectData, String> {
        data.salvage_objects
            .get(target_id)
            .ok_or_else(|| format!("unknown salvage target '{target_id}'"))
    }

    pub fn target_is_removed(&self, target_id: &str) -> bool {
        let Some(expedition) = &self.expedition else {
            return false;
        };
        self.site_progress
            .get(&expedition.site_id)
            .is_some_and(|progress| progress.removed_targets.iter().any(|id| id == target_id))
    }

    pub fn target_is_revealed(&self, target_id: &str) -> bool {
        self.expedition.as_ref().is_some_and(|expedition| {
            expedition.workspace_scanned
                && expedition.revealed_targets.iter().any(|id| id == target_id)
        })
    }

    pub fn extraction_duration(&self, target_id: &str, data: &GameData) -> Result<f32, String> {
        let target = self.workspace_target(target_id, data)?;
        let drone_reduction = (self.workspace_drone_directive().extraction_reduction()
            * self.module_stats(data).drone_support.max(0) as f32)
            .min(0.35);
        Ok((target.extraction_duration * (1.0 - drone_reduction)).max(1.0))
    }

    pub fn has_capability(&self, capability: &str, data: &GameData) -> bool {
        self.ship_layout
            .placements
            .iter()
            .filter(|item| item.permanent)
            .filter(|item| !self.damaged_modules.iter().any(|id| id == &item.id))
            .filter_map(|item| data.modules.get(&item.id))
            .any(|module| module.capability.as_deref() == Some(capability))
    }

    pub fn target_is_stabilized(&self, target_id: &str) -> bool {
        self.expedition.as_ref().is_some_and(|expedition| {
            expedition
                .stabilized_targets
                .iter()
                .any(|id| id == target_id)
        })
    }

    pub fn stabilize_workspace_target(
        &mut self,
        target_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let target = self.workspace_target(target_id, data)?;
        if !self.target_is_revealed(target_id) {
            return Err("Scan this section before stabilizing the target.".to_owned());
        }
        if self.target_is_removed(target_id) {
            return Err("The mount is empty; this target is already recovered.".to_owned());
        }
        if target.hazard.is_none() {
            return Err("This target has no authored hazard to stabilize.".to_owned());
        }
        if self.target_is_stabilized(target_id) {
            return Ok("Target is already stabilized; the lock is holding.".to_owned());
        }
        if !self.has_capability("stabilizer", data) {
            return Err("Requires Stabilizer capability.".to_owned());
        }
        let name = workspace_name(target).to_owned();
        let command = TransferMode::from_target(target).command_label();
        self.spend_workspace_energy(WORKSPACE_STABILIZATION_ENERGY_COST)?;
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.stabilized_targets.push(target_id.to_owned());
        self.record_workspace_event(WorkspaceLogEvent::TargetStabilized, Some(target_id));
        let (remaining, capacity) = self.workspace_energy().unwrap_or((0, 0));
        Ok(format!(
            "Stabilizer locked on {name}. Exposure -20. Power {remaining}/{capacity}. Tap {command}."
        ))
    }

    pub fn apply_workspace_damage(&mut self, data: &GameData) -> String {
        self.hull = (self.hull - 1).max(1);
        let damaged = self
            .ship_layout
            .placements
            .iter()
            .find(|item| item.permanent && !self.damaged_modules.contains(&item.id))
            .map(|item| item.id.clone());
        if let Some(module_id) = damaged {
            self.damaged_modules.push(module_id.clone());
            self.economy.fuel = self.economy.fuel.min(self.max_fuel(data));
            let display_name = data
                .modules
                .get(&module_id)
                .map_or(module_id.as_str(), |module| module.display_name.as_str());
            format!(" Hull -1. {display_name} is marked damaged.")
        } else {
            " Hull -1. Existing damage held; no new system was marked.".to_owned()
        }
    }

    pub fn lose_workspace_target(
        &mut self,
        target_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        if !self.target_is_revealed(target_id) {
            return Err("Scan this section before selecting the target.".to_owned());
        }
        let site_id = self
            .expedition
            .as_ref()
            .ok_or_else(|| "there is no active expedition".to_owned())?
            .site_id
            .clone();
        let contract_accepted = self
            .expedition
            .as_ref()
            .is_some_and(|expedition| expedition.contract_accepted);
        let target = self.workspace_target(target_id, data)?;
        if let Some(expedition) = self.expedition.as_mut() {
            if expedition
                .workspace_transfer
                .as_ref()
                .is_some_and(|transfer| transfer.target_id == target_id)
            {
                expedition.workspace_transfer = None;
            }
            expedition.revealed_targets.retain(|id| id != target_id);
            expedition.stabilized_targets.retain(|id| id != target_id);
        }
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.removed_targets.iter().any(|id| id == target_id) {
                progress.removed_targets.push(target_id.to_owned());
            }
        }
        let section_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.workspace_section.clone());
        self.append_workspace_log(
            &site_id,
            WorkspaceLogEvent::TargetLost,
            section_id.as_deref(),
            Some(target_id),
        );
        let mut message = format!(
            "{} lost in the wreckage; the mount is now empty.",
            workspace_name(target)
        );
        if let Some(contract_message) =
            self.complete_site_contract_for_run(&site_id, &[], data, contract_accepted)
        {
            message.push_str(&contract_message);
        }
        Ok(message)
    }

    pub fn tractor_capacity_tons(&self, data: &GameData) -> f32 {
        (self.module_stats(data).power * 4) as f32
    }

    pub fn extraction_block_reason(
        &self,
        target_id: &str,
        data: &GameData,
    ) -> Result<Option<String>, String> {
        let target = self.workspace_target(target_id, data)?;
        if !self.target_is_revealed(target_id) {
            return Ok(Some(
                "Scan this section before selecting the target.".to_owned(),
            ));
        }
        if self.target_is_removed(target_id) {
            return Ok(Some(
                "The mount is empty; this target is already recovered.".to_owned(),
            ));
        }
        if let Some(required) = &target.required_capability {
            if !self.has_capability(required, data) {
                return Ok(Some(format!(
                    "Requires {} capability.",
                    capability_label(required)
                )));
            }
        }
        if target.mass_tons > self.tractor_capacity_tons(data) {
            return Ok(Some(format!(
                "Tractor capacity insufficient: {:.0} / {:.0}t.",
                self.tractor_capacity_tons(data),
                target.mass_tons
            )));
        }
        if let Some(reason) = self.workspace_energy_block_reason(target.energy_cost) {
            return Ok(Some(reason));
        }
        Ok(None)
    }

    pub fn reserve_workspace_energy(
        &mut self,
        target_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let target = self.workspace_target(target_id, data)?;
        if let Some(reason) = self.extraction_block_reason(target_id, data)? {
            return Err(reason);
        }
        self.spend_workspace_energy(target.energy_cost)?;
        self.record_workspace_event(WorkspaceLogEvent::ExtractionStarted, Some(target_id));
        Ok(format!(
            "Power reserve -{}; {} remaining.",
            target.energy_cost,
            self.workspace_energy()
                .map_or(0, |(remaining, _)| remaining)
        ))
    }

    pub fn recover_workspace_target(
        &mut self,
        target_id: &str,
        data: &GameData,
    ) -> Result<String, String> {
        let transfer = self
            .workspace_transfer()
            .filter(|transfer| transfer.target_id == target_id)
            .cloned()
            .ok_or_else(|| data.salvage_ui.choose_destination.clone())?;
        if !self.target_is_revealed(target_id) || self.target_is_removed(target_id) {
            return Err("that salvage is no longer available".to_owned());
        }
        self.validate_workspace_placement(target_id, transfer.position, transfer.rotation, data)?;
        let target = self.workspace_target(target_id, data)?.clone();
        self.ship_layout
            .place(
                super::super::cargo_layout_id(target_id),
                target.footprint,
                transfer.position,
                transfer.rotation,
                false,
            )
            .map_err(|error| error.to_string())?;
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.revealed_targets.retain(|id| id != target_id);
        expedition.stabilized_targets.retain(|id| id != target_id);
        expedition.workspace_transfer = None;
        expedition.cargo.retain(|item| item.object_id != target_id);
        expedition.cargo.push(CargoItem {
            object_id: target_id.to_owned(),
            status: CargoStatus::Packed,
            position: Some(transfer.position),
            rotation: transfer.rotation,
        });
        let site_id = expedition.site_id.clone();
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.removed_targets.iter().any(|id| id == target_id) {
                progress.removed_targets.push(target_id.to_owned());
            }
        }
        let section_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.workspace_section.clone());
        self.append_workspace_log(
            &site_id,
            WorkspaceLogEvent::TargetRecovered,
            section_id.as_deref(),
            Some(target_id),
        );
        let name = workspace_name(&target);
        Ok(format!("{name}: {}", data.salvage_ui.transfer_complete))
    }

    fn workspace_energy_block_reason(&self, energy_cost: i32) -> Option<String> {
        let (remaining, capacity) = self.workspace_energy()?;
        if energy_cost > remaining {
            Some(format!(
                "Power reserve insufficient: need {}, have {} / {}.",
                energy_cost, remaining, capacity
            ))
        } else {
            None
        }
    }

    fn spend_workspace_energy(&mut self, energy_cost: i32) -> Result<(), String> {
        if let Some(reason) = self.workspace_energy_block_reason(energy_cost) {
            return Err(reason);
        }
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_energy -= energy_cost.max(0);
        Ok(())
    }

    pub(crate) fn append_workspace_log(
        &mut self,
        site_id: &str,
        event: WorkspaceLogEvent,
        section_id: Option<&str>,
        target_id: Option<&str>,
    ) {
        if let Some(progress) = self.site_progress.get_mut(site_id) {
            let sequence = progress.operation_log.len() as u32 + 1;
            progress.operation_log.push(WorkspaceLogEntry::new(
                sequence, event, section_id, target_id,
            ));
        }
    }
}
