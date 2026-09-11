//! Save-state integrity checks for layouts, cargo phases, and progression.

use super::*;
use std::collections::HashSet;

pub(super) fn validate_saved_runtime(
    session: &GameSession,
    checked: &ShipLayout,
    data: &GameData,
) -> Result<(), String> {
    if let Some(site_id) = &session.selected_site {
        if !data.sites.contains(site_id) {
            return Err(format!("save references missing selected site '{site_id}'"));
        }
    }
    if session
        .site_progress
        .keys()
        .any(|site_id| !data.sites.contains(site_id))
    {
        return Err("save contains progress for an unknown site".to_owned());
    }
    if let Some(expedition) = &session.expedition {
        if expedition.power_cycles_used > 1
            || expedition.workspace_energy < 0
            || expedition.workspace_energy > expedition.workspace_energy_capacity
            || expedition.workspace_energy_capacity <= 0
        {
            return Err("save contains an invalid workspace power reserve".to_owned());
        }
    }
    for (site_id, progress) in &session.site_progress {
        if !(0..=100).contains(&progress.condition) {
            return Err("save contains an invalid site condition".to_owned());
        }
        if progress.reconnaissance_level > data.config.reconnaissance.max_level {
            return Err("save contains an invalid reconnaissance level".to_owned());
        }
        if progress.contract_completed && progress.contract_failed {
            return Err("save contains a contract marked complete and failed".to_owned());
        }
        let site = data.sites.get(site_id).expect("site keys validated above");
        let mut discovered_sections = HashSet::new();
        let mut removed_targets = HashSet::new();
        for section_id in &progress.discovered_sections {
            if !site
                .sections
                .iter()
                .any(|section| &section.id == section_id)
                || !discovered_sections.insert(section_id)
            {
                return Err(format!(
                    "save references unknown or duplicate discovered section '{section_id}'"
                ));
            }
        }
        for target_id in &progress.removed_targets {
            if !data.salvage_objects.contains(target_id)
                || !site.sections.iter().any(|section| {
                    section
                        .candidate_targets
                        .iter()
                        .any(|target| target == target_id)
                })
                || !removed_targets.insert(target_id)
            {
                return Err(format!(
                    "save references unknown removed target '{target_id}'"
                ));
            }
        }
        let mut operation_sequences = HashSet::new();
        for entry in &progress.operation_log {
            if entry.sequence == 0 || !operation_sequences.insert(entry.sequence) {
                return Err(format!(
                    "save contains an invalid operation log sequence at site '{site_id}'"
                ));
            }
            if !entry.section_id.is_empty()
                && !site
                    .sections
                    .iter()
                    .any(|section| section.id == entry.section_id)
            {
                return Err(format!(
                    "save operation log references unknown section '{}'",
                    entry.section_id
                ));
            }
            if entry.event.is_target_event() != entry.target_id.is_some() {
                return Err(format!(
                    "save operation log has mismatched target context at site '{site_id}'"
                ));
            }
            if let Some(target_id) = &entry.target_id {
                if !data.salvage_objects.contains(target_id)
                    || !site.sections.iter().any(|section| {
                        section
                            .candidate_targets
                            .iter()
                            .any(|candidate| candidate == target_id)
                    })
                {
                    return Err(format!(
                        "save operation log references unknown target '{target_id}'"
                    ));
                }
            }
        }
        let mut survey_keys = HashSet::new();
        for note in &progress.surveyed_targets {
            if !data.salvage_objects.contains(&note.target_id)
                || !site.sections.iter().any(|section| {
                    section.id == note.section_id
                        && section
                            .candidate_targets
                            .iter()
                            .any(|target_id| target_id == &note.target_id)
                })
                || note.scan_count == 0
                || !(0..=100).contains(&note.integrity)
                || !(0..=100).contains(&note.extraction_difficulty)
                || !note.mass_tons.is_finite()
                || note.mass_tons < 0.0
                || !matches!(
                    note.transfer_mode.as_str(),
                    "internal_cargo" | "external_clamp" | "tow"
                )
                || note.hazard.as_deref().is_some_and(|hazard| {
                    crate::engine::WorkspaceHazard::from_value(hazard).is_none()
                })
                || !survey_keys.insert((&note.section_id, &note.target_id))
            {
                return Err(format!(
                    "save contains an invalid surveyed target note for site '{site_id}'"
                ));
            }
        }
    }
    for record in &session.voyage_log {
        let Some(site) = data.sites.get(&record.site_id) else {
            return Err(format!(
                "save voyage log references unknown site '{}'",
                record.site_id
            ));
        };
        if !(0..=100).contains(&record.danger_score)
            || !(0..=100).contains(&record.condition_after)
            || record.reconnaissance_level > data.config.reconnaissance.max_level
            || record.recovered_value < 0
            || record.recovered_alloy < 0
            || record.recovered_electronics < 0
            || record.insurance_premium < 0
            || record.insurance_payout < 0
            || record.return_fuel < 0
            || (!record.insured && (record.insurance_premium > 0 || record.insurance_payout > 0))
            || (record.contract_completed && record.contract_failed)
        {
            return Err("save contains an invalid voyage log measurement".to_owned());
        }
        let target_count = site
            .sections
            .iter()
            .flat_map(|section| section.candidate_targets.iter())
            .collect::<HashSet<_>>()
            .len() as u32;
        if record.recovered_count > target_count {
            return Err(format!(
                "save voyage log overstates recovery at site '{}'",
                record.site_id
            ));
        }
    }
    let mut unlocked = HashSet::new();
    for module_id in &session.unlocked_modules {
        if !data.modules.contains(module_id) || !unlocked.insert(module_id) {
            return Err(format!(
                "save contains an invalid unlocked module '{module_id}'"
            ));
        }
    }
    let mut damaged_modules = HashSet::new();
    for module_id in &session.damaged_modules {
        if !data.modules.contains(module_id)
            || !checked
                .placements
                .iter()
                .any(|item| item.permanent && item.id == *module_id)
            || !damaged_modules.insert(module_id)
        {
            return Err(format!(
                "save contains an invalid damaged module '{module_id}'"
            ));
        }
    }
    if session.external_cargo_count(data, None) > session.external_capacity(data) {
        return Err("save exceeds the ship's external clamp capacity".to_owned());
    }
    if session.expedition.is_some() && !session.returned.is_empty() {
        return Err("save cannot contain both an active expedition and returned cargo".to_owned());
    }
    let mut cargo_ids = HashSet::new();
    if let Some(expedition) = &session.expedition {
        let site = data
            .sites
            .get(&expedition.site_id)
            .expect("expedition site validated above");
        if !site
            .sections
            .iter()
            .any(|section| section.id == expedition.workspace_section)
        {
            return Err("save contains an invalid workspace section".to_owned());
        }
        if expedition
            .revealed_targets
            .iter()
            .any(|target| !data.salvage_objects.contains(target))
        {
            return Err("save contains an unknown revealed target".to_owned());
        }
        if expedition.workspace_energy_capacity <= 0
            || expedition.workspace_energy < 0
            || expedition.workspace_energy > expedition.workspace_energy_capacity
        {
            return Err("save contains an invalid workspace power reserve".to_owned());
        }
        let mut stabilized_targets = HashSet::new();
        for target_id in &expedition.stabilized_targets {
            if !data.salvage_objects.contains(target_id)
                || !site.sections.iter().any(|section| {
                    section
                        .candidate_targets
                        .iter()
                        .any(|target| target == target_id)
                })
                || !stabilized_targets.insert(target_id)
            {
                return Err(format!(
                    "save references unknown or duplicate stabilized target '{target_id}'"
                ));
            }
        }
        for cargo in &expedition.cargo {
            if !cargo_ids.insert(&cargo.object_id) {
                return Err(format!(
                    "save contains duplicate cargo '{}'",
                    cargo.object_id
                ));
            }
            let layout_item = checked
                .placements
                .iter()
                .find(|item| item.id == cargo_layout_id(&cargo.object_id));
            match cargo.status {
                CargoStatus::Packed => {
                    let Some(position) = cargo.position else {
                        return Err(format!(
                            "packed cargo '{}' has no position",
                            cargo.object_id
                        ));
                    };
                    let Some(layout_item) = layout_item else {
                        return Err(format!(
                            "packed cargo '{}' is missing from the layout",
                            cargo.object_id
                        ));
                    };
                    if layout_item.permanent
                        || layout_item.position != position
                        || layout_item.rotation != cargo.rotation % 2
                    {
                        return Err(format!(
                            "packed cargo '{}' does not match the layout",
                            cargo.object_id
                        ));
                    }
                }
                CargoStatus::Pending | CargoStatus::LeftBehind | CargoStatus::Discarded => {
                    if cargo.position.is_some() || layout_item.is_some() {
                        return Err(format!(
                            "unpacked cargo '{}' occupies the layout",
                            cargo.object_id
                        ));
                    }
                }
                CargoStatus::Lost => {
                    return Err("active expedition contains lost cargo".to_owned());
                }
            }
        }
    }
    let mut returned_ids = HashSet::new();
    for returned in &session.returned {
        if !returned_ids.insert(&returned.object_id) {
            return Err(format!(
                "save contains duplicate returned cargo '{}'",
                returned.object_id
            ));
        }
        let Some(layout_item) = checked
            .placements
            .iter()
            .find(|item| item.id == cargo_layout_id(&returned.object_id))
        else {
            return Err(format!(
                "returned cargo '{}' is missing from the layout",
                returned.object_id
            ));
        };
        if layout_item.permanent
            || layout_item.position != returned.position
            || layout_item.rotation != returned.rotation % 2
        {
            return Err(format!(
                "returned cargo '{}' does not match the layout",
                returned.object_id
            ));
        }
    }
    for item in checked.placements.iter().filter(|item| !item.permanent) {
        let Some(object_id) = item.id.strip_prefix("cargo:") else {
            return Err(format!("temporary layout item '{}' is not cargo", item.id));
        };
        let valid_active = session.expedition.as_ref().is_some_and(|expedition| {
            expedition
                .cargo
                .iter()
                .any(|cargo| cargo.object_id == object_id && cargo.status == CargoStatus::Packed)
        });
        let valid_returned = session
            .returned
            .iter()
            .any(|cargo| cargo.object_id == object_id);
        if !valid_active && !valid_returned {
            return Err(format!(
                "layout contains cargo not present in the save state: '{object_id}'"
            ));
        }
    }
    Ok(())
}

fn cargo_layout_id(object_id: &str) -> String {
    format!("cargo:{object_id}")
}
