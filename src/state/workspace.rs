//! Authoritative workspace discovery, capability gates, and wreck condition changes.

use super::{CargoItem, CargoStatus, GameSession};
use crate::data::{GameData, SalvageObjectData, WreckSectionData};
use crate::engine::{resolve_extraction, WorkspaceRiskReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractionPhase {
    Alignment,
    Connection,
    Strain,
    Separation,
    Retrieval,
    Capture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    InternalCargo,
    ExternalClamp,
    Tow,
}

impl TransferMode {
    pub fn from_value(value: &str) -> Self {
        match value {
            "external_clamp" => Self::ExternalClamp,
            "tow" => Self::Tow,
            _ => Self::InternalCargo,
        }
    }

    pub fn from_target(target: &SalvageObjectData) -> Self {
        Self::from_value(&target.transfer_mode)
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "CARGO",
            Self::ExternalClamp => "CLAMP",
            Self::Tow => "TOW",
        }
    }

    pub const fn uses_external_rig(self) -> bool {
        !matches!(self, Self::InternalCargo)
    }

    pub const fn destination_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "SALVAGE HOLD",
            Self::ExternalClamp => "EXTERNAL CLAMP",
            Self::Tow => "TOW RIG",
        }
    }

    pub const fn destination_message(self) -> &'static str {
        match self {
            Self::InternalCargo => "the salvage hold",
            Self::ExternalClamp => "the external clamp queue",
            Self::Tow => "the tow rig queue",
        }
    }

    pub const fn command_label(self) -> &'static str {
        match self {
            Self::InternalCargo => "LOAD CARGO",
            Self::ExternalClamp => "LOCK CLAMP",
            Self::Tow => "ENGAGE TOW",
        }
    }

    pub const fn engaged_message(self) -> &'static str {
        match self {
            Self::InternalCargo => "Cargo intake engaged.",
            Self::ExternalClamp => "External clamp engaged.",
            Self::Tow => "Tow rig engaged.",
        }
    }

    pub const fn phase_label(self, phase: ExtractionPhase) -> &'static str {
        match (self, phase) {
            (Self::InternalCargo, ExtractionPhase::Alignment) => "ALIGNING INTAKE",
            (Self::InternalCargo, ExtractionPhase::Connection) => "CARGO INTAKE LIVE",
            (Self::InternalCargo, ExtractionPhase::Strain) => "LOADING UNDER STRAIN",
            (Self::InternalCargo, ExtractionPhase::Separation) => "BREAKING CARGO FREE",
            (Self::InternalCargo, ExtractionPhase::Retrieval) => "RETRIEVING TO HOLD",
            (Self::InternalCargo, ExtractionPhase::Capture) => "CARGO LOCKED",
            (Self::ExternalClamp, ExtractionPhase::Alignment) => "ALIGNING CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Connection) => "CLAMP CONTACT",
            (Self::ExternalClamp, ExtractionPhase::Strain) => "CLAMP UNDER LOAD",
            (Self::ExternalClamp, ExtractionPhase::Separation) => "SEATING CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Retrieval) => "RETRIEVING TO CLAMP",
            (Self::ExternalClamp, ExtractionPhase::Capture) => "CLAMP SEALED",
            (Self::Tow, ExtractionPhase::Alignment) => "ALIGNING TOW HEAD",
            (Self::Tow, ExtractionPhase::Connection) => "TOW LINE CONNECTED",
            (Self::Tow, ExtractionPhase::Strain) => "TOW LINE UNDER LOAD",
            (Self::Tow, ExtractionPhase::Separation) => "BREAKING TOW FREE",
            (Self::Tow, ExtractionPhase::Retrieval) => "RETRIEVING TO RIG",
            (Self::Tow, ExtractionPhase::Capture) => "TOW COUPLED",
        }
    }
}

/// Persistent condition as seen from the currently selected wreck section.
///
/// The frame condition comes from the saved site progress. Section condition
/// also accounts for targets already removed from this frame, so revisiting a
/// partially salvaged wreck has a stable, data-backed visual state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceConditionStatus {
    pub frame_condition: i32,
    pub section_condition: i32,
    pub recovered_targets: usize,
    pub total_targets: usize,
    pub discovered: bool,
}

impl WorkspaceConditionStatus {
    pub fn unknown() -> Self {
        Self {
            frame_condition: 0,
            section_condition: 0,
            recovered_targets: 0,
            total_targets: 0,
            discovered: false,
        }
    }

    pub fn label(self) -> &'static str {
        if !self.discovered {
            "UNMAPPED"
        } else if self.section_condition <= 30 {
            "CRITICAL"
        } else if self.recovered_targets > 0 {
            "STRESSED"
        } else {
            "STABLE"
        }
    }

    pub fn structural_stress(self) -> i32 {
        (100 - self.section_condition).clamp(0, 100)
    }
}

#[derive(Debug, Clone)]
pub struct ExtractionRuntime {
    pub target_id: String,
    pub elapsed: f32,
    pub duration: f32,
    pub resolved: bool,
}

impl ExtractionRuntime {
    pub fn new(target_id: impl Into<String>, duration: f32) -> Self {
        Self {
            target_id: target_id.into(),
            elapsed: 0.0,
            duration: duration.max(1.0),
            resolved: false,
        }
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    pub fn phase(&self) -> ExtractionPhase {
        match self.progress() {
            value if value < 0.12 => ExtractionPhase::Alignment,
            value if value < 0.28 => ExtractionPhase::Connection,
            value if value < 0.55 => ExtractionPhase::Strain,
            value if value < 0.68 => ExtractionPhase::Separation,
            value if value < 0.92 => ExtractionPhase::Retrieval,
            _ => ExtractionPhase::Capture,
        }
    }
}

impl GameSession {
    pub fn workspace_condition_status(
        &self,
        data: &GameData,
    ) -> Result<WorkspaceConditionStatus, String> {
        let site = self.workspace_site(data)?;
        let section = self.workspace_section(data)?;
        self.site_section_condition_status(&site.id, &section.id, data)
            .ok_or_else(|| format!("unknown wreck section '{}'", section.id))
    }

    pub fn site_section_condition_status(
        &self,
        site_id: &str,
        section_id: &str,
        data: &GameData,
    ) -> Option<WorkspaceConditionStatus> {
        let site = data.sites.get(site_id)?;
        let section = site
            .sections
            .iter()
            .find(|section| section.id == section_id)?;
        let frame_condition = self
            .site_progress
            .get(site_id)
            .map_or(site.condition, |progress| progress.condition)
            .clamp(0, 100);
        let (recovered_targets, discovered) =
            self.site_progress
                .get(site_id)
                .map_or((0, false), |progress| {
                    (
                        section
                            .candidate_targets
                            .iter()
                            .filter(|target_id| {
                                progress
                                    .removed_targets
                                    .iter()
                                    .any(|removed_id| removed_id == *target_id)
                            })
                            .count(),
                        progress
                            .discovered_sections
                            .iter()
                            .any(|section_id| section_id == &section.id),
                    )
                });
        let section_condition = (frame_condition - recovered_targets as i32 * 8).max(0);
        Some(WorkspaceConditionStatus {
            frame_condition,
            section_condition,
            recovered_targets,
            total_targets: section.candidate_targets.len(),
            discovered,
        })
    }

    pub fn workspace_energy(&self) -> Option<(i32, i32)> {
        self.expedition.as_ref().map(|expedition| {
            (
                expedition.workspace_energy,
                expedition.workspace_energy_capacity,
            )
        })
    }

    pub fn workspace_site<'a>(
        &self,
        data: &'a GameData,
    ) -> Result<&'a crate::data::SiteData, String> {
        let site_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.site_id.as_str())
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        data.sites
            .get(site_id)
            .ok_or_else(|| format!("unknown salvage site '{site_id}'"))
    }

    pub fn workspace_section<'a>(
        &self,
        data: &'a GameData,
    ) -> Result<&'a WreckSectionData, String> {
        let site = self.workspace_site(data)?;
        let section_id = self
            .expedition
            .as_ref()
            .map(|expedition| expedition.workspace_section.as_str())
            .unwrap_or_default();
        site.sections
            .iter()
            .find(|section| section.id == section_id)
            .or_else(|| site.sections.first())
            .ok_or_else(|| format!("site '{}' has no workspace sections", site.id))
    }

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
        let visible: Vec<String> = target_ids
            .into_iter()
            .filter(|target| !removed.iter().any(|removed_id| removed_id == target))
            .collect();
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_scanned = true;
        expedition.revealed_targets = visible.clone();
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.discovered_sections.contains(&section_id) {
                progress.discovered_sections.push(section_id);
            }
        }
        let (recovered, total_targets) = self.site_recovery_summary(&site_id, data);
        let remaining_targets = total_targets.saturating_sub(recovered);
        Ok(format!(
            "Scan complete: {} target(s) remain readable. Power {}/{}. Site recovery is {}/{}; {} remain.",
            visible.len(),
            self.workspace_energy()
                .map_or(0, |(remaining, _)| remaining),
            self.workspace_energy().map_or(0, |(_, capacity)| capacity),
            recovered,
            total_targets,
            remaining_targets
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

    pub fn workspace_risk_preview(
        &self,
        target_id: &str,
        data: &GameData,
    ) -> Result<WorkspaceRiskReport, String> {
        let site = self.workspace_site(data)?;
        let section = self.workspace_section(data)?;
        let target = self.workspace_target(target_id, data)?;
        let stats = self.module_stats(data);
        Ok(resolve_extraction(
            self.expedition
                .as_ref()
                .map_or(7, |expedition| expedition.seed),
            site.danger,
            &section.hazard_tags,
            target,
            stats,
            self.has_capability("stabilizer", data),
            self.has_capability("scanner_array", data),
            stats.drone_support,
        ))
    }

    pub fn extraction_duration(&self, target_id: &str, data: &GameData) -> Result<f32, String> {
        let target = self.workspace_target(target_id, data)?;
        let drone_reduction =
            (self.module_stats(data).drone_support.max(0) as f32 * 0.12).min(0.35);
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
        let target = self.workspace_target(target_id, data)?;
        if let Some(expedition) = self.expedition.as_mut() {
            expedition.revealed_targets.retain(|id| id != target_id);
        }
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.removed_targets.iter().any(|id| id == target_id) {
                progress.removed_targets.push(target_id.to_owned());
            }
        }
        let mut message = format!(
            "{} lost in the wreckage; the mount is now empty.",
            workspace_name(target)
        );
        if let Some(contract_message) = self.complete_site_contract(&site_id, &[], data) {
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
        if let Some(reason) = self.extraction_block_reason(target_id, data)? {
            return Err(reason);
        }
        let target = self.workspace_target(target_id, data)?.clone();
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.revealed_targets.retain(|id| id != target_id);
        if !expedition
            .cargo
            .iter()
            .any(|item| item.object_id == target_id && item.status != CargoStatus::Lost)
        {
            expedition.cargo.push(CargoItem {
                object_id: target_id.to_owned(),
                status: CargoStatus::Pending,
                position: None,
                rotation: 0,
            });
        }
        let site_id = expedition.site_id.clone();
        if let Some(progress) = self.site_progress.get_mut(&site_id) {
            if !progress.removed_targets.iter().any(|id| id == target_id) {
                progress.removed_targets.push(target_id.to_owned());
            }
        }
        let name = workspace_name(&target);
        let destination = TransferMode::from_target(&target).destination_message();
        Ok(format!(
            "{name} recovered; marked for {destination} during packing."
        ))
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
}

fn workspace_name(target: &SalvageObjectData) -> &str {
    if target.workspace_name.is_empty() {
        target.display_name.as_str()
    } else {
        target.workspace_name.as_str()
    }
}

fn capability_label(value: &str) -> String {
    value
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests;
