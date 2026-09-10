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
    for (site_id, progress) in &session.site_progress {
        if !(0..=100).contains(&progress.condition) {
            return Err("save contains an invalid site condition".to_owned());
        }
        if progress.contract_completed && progress.contract_failed {
            return Err("save contains a contract marked complete and failed".to_owned());
        }
        let site = data.sites.get(site_id).expect("site keys validated above");
        let mut removed_targets = HashSet::new();
        for section_id in &progress.discovered_sections {
            if !site
                .sections
                .iter()
                .any(|section| &section.id == section_id)
            {
                return Err(format!(
                    "save references unknown discovered section '{section_id}'"
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
            || record.recovered_value < 0
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
