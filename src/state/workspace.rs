//! Authoritative workspace discovery, capability gates, and wreck condition changes.

use super::{CargoItem, CargoStatus, GameSession};
use crate::data::{GameData, SalvageObjectData, WreckSectionData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractionPhase {
    Alignment,
    Connection,
    Strain,
    Separation,
    Retrieval,
    Capture,
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
        Ok(format!(
            "Scan complete: {} target(s) and their hazards are readable.",
            visible.len()
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
        if section.id != current
            && !self
                .workspace_section(data)?
                .connected_sections
                .iter()
                .any(|neighbor| neighbor == section_id)
        {
            return Err("that section is not connected to the current frame".to_owned());
        }
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        expedition.workspace_section = section.id.clone();
        expedition.workspace_scanned = false;
        expedition.revealed_targets.clear();
        Ok(format!(
            "Camera moved to {}. Scan the section before working.",
            section.display_name
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
            let has_capability = self
                .ship_layout
                .placements
                .iter()
                .filter(|item| item.permanent)
                .filter_map(|item| data.modules.get(&item.id))
                .any(|module| module.capability.as_deref() == Some(required.as_str()));
            if !has_capability {
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
        Ok(None)
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
        let name = if target.workspace_name.is_empty() {
            target.display_name.as_str()
        } else {
            target.workspace_name.as_str()
        };
        Ok(format!("{} recovered into the salvage hold.", name))
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
