//! Save restoration and serialization for the active game session.

use super::*;
use crate::data::Footprint;

impl GameSession {
    pub fn from_save(save: SaveData, data: &GameData) -> Result<Self, String> {
        let mut session = save.session;
        let resolved = session.resolved_data(data)?;
        let data = &resolved;
        restore_legacy_fields(&mut session, data);
        validate_session_limits(&session, data)?;
        let checked = validate_saved_layout(&session, data)?;
        validate_saved_references(&session, data)?;
        loadout::validate_saved_loadouts(&session.loadout_slots, &session, data)?;
        validation::validate_saved_runtime(&session, &checked, data)?;
        validate_current_capacities(&session, data)?;
        session.ship_layout = checked;
        session.refresh_module_unlocks(data);
        Ok(session)
    }

    pub fn to_save(&self, version: &str) -> SaveData {
        SaveData {
            version: version.to_owned(),
            session: self.clone(),
        }
    }
}

fn restore_legacy_fields(session: &mut GameSession, data: &GameData) {
    if session.career.is_empty() && !session.voyage_log.is_empty() {
        session.career = CareerStats::from_voyage_log(&session.voyage_log);
    }
    if let Some(expedition) = session.expedition.as_mut() {
        if expedition.workspace_section.is_empty() {
            expedition.workspace_section = data
                .sites
                .get(&expedition.site_id)
                .and_then(|site| site.sections.first())
                .map_or_else(String::new, |section| section.id.clone());
        }
    }
}

fn validate_session_limits(session: &GameSession, data: &GameData) -> Result<(), String> {
    session.career.validate()?;
    if session.ship_layout.width != data.config.grid_width
        || session.ship_layout.height != data.config.grid_height
    {
        return Err("save uses an incompatible ship grid".to_owned());
    }
    if session.crew_fatigue > crew_readiness::MAX_CREW_FATIGUE {
        return Err("save contains invalid crew fatigue".to_owned());
    }
    if session.ship_wear > ship_wear::MAX_SHIP_WEAR {
        return Err("save contains invalid ship wear".to_owned());
    }
    if session.field_power_cells > workspace_energy::MAX_FIELD_POWER_CELLS {
        return Err("save contains too many field power cells".to_owned());
    }
    if session.cargo_bay_level > cargo_bay::MAX_CARGO_BAY_LEVEL {
        return Err("save contains an invalid cargo bay level".to_owned());
    }
    if session.economy.credits < 0
        || session.economy.fuel < 0
        || session.economy.alloy < 0
        || session.economy.electronics < 0
        || session.reputation < 0
        || session.hull <= 0
        || session.max_hull != data.config.max_hull
    {
        return Err("save contains invalid economy or hull values".to_owned());
    }
    Ok(())
}

fn validate_saved_layout(session: &GameSession, data: &GameData) -> Result<ShipLayout, String> {
    let mut checked = ShipLayout::new(session.ship_layout.width, session.ship_layout.height);
    for item in &session.ship_layout.placements {
        let footprint = saved_item_footprint(&item.id, data)?;
        if footprint != item.footprint {
            return Err(format!("save footprint mismatch for '{}'", item.id));
        }
        checked
            .place(
                item.id.clone(),
                item.footprint,
                item.position,
                item.rotation,
                item.permanent,
            )
            .map_err(|error| format!("invalid saved layout: {error}"))?;
    }
    Ok(checked)
}

fn saved_item_footprint(id: &str, data: &GameData) -> Result<Footprint, String> {
    if let Some(object_id) = id.strip_prefix("cargo:") {
        data.salvage_objects
            .get(object_id)
            .map(|object| object.footprint)
            .ok_or_else(|| format!("save references missing salvage '{object_id}'"))
    } else {
        data.modules
            .get(id)
            .map(|module| module.footprint)
            .ok_or_else(|| format!("save references missing module '{id}'"))
    }
}

fn validate_saved_references(session: &GameSession, data: &GameData) -> Result<(), String> {
    if let Some(expedition) = &session.expedition {
        if !data.sites.contains(&expedition.site_id) {
            return Err(format!(
                "save references missing expedition site '{}'",
                expedition.site_id
            ));
        }
        for cargo in &expedition.cargo {
            if !data.salvage_objects.contains(&cargo.object_id) {
                return Err(format!(
                    "save references missing salvage '{}'",
                    cargo.object_id
                ));
            }
        }
    }
    for returned in &session.returned {
        if !data.salvage_objects.contains(&returned.object_id) {
            return Err(format!(
                "save references missing returned salvage '{}'",
                returned.object_id
            ));
        }
    }
    Ok(())
}

fn validate_current_capacities(session: &GameSession, data: &GameData) -> Result<(), String> {
    if session.economy.fuel > session.max_fuel(data)
        || session.hull > session.max_hull_with_modules(data)
    {
        return Err("save exceeds the ship's current fuel or hull capacity".to_owned());
    }
    Ok(())
}
