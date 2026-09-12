//! Cross-reference and range validation for embedded game data.

use std::collections::HashSet;

use super::{
    Footprint, GameConfig, GameData, GridPosition, ModuleData, SalvageObjectData, SiteData,
    StartingModule, WreckSectionData,
};

pub(super) fn validate(data: &GameData) -> Result<(), String> {
    validate_config(&data.config)?;
    validate_salvage_objects(data)?;
    validate_modules(data)?;
    validate_sites(data)?;
    validate_starting_modules(data)?;
    Ok(())
}

fn validate_config(config: &GameConfig) -> Result<(), String> {
    if config.risk.external_cargo_risk_per_item < 0 {
        return Err("risk tuning cannot use a negative external cargo penalty".to_owned());
    }
    let risk_weights = [
        config.risk.ordinary_return_weight,
        config.risk.damaged_module_weight,
        config.risk.lost_salvage_weight,
        config.risk.emergency_repair_weight,
        config.risk.forced_abandon_weight,
    ];
    if risk_weights.iter().any(|weight| *weight < 0) || risk_weights.iter().sum::<i32>() != 100 {
        return Err("risk tuning weights must be non-negative and total 100".to_owned());
    }
    if config.grid_width <= 0 || config.grid_height <= 0 {
        return Err("game_config.json: grid dimensions must be positive".to_owned());
    }
    if config.starting_fuel < 0 || config.starting_fuel > config.max_fuel {
        return Err("game_config.json: starting_fuel must fit max_fuel".to_owned());
    }
    if config.repair_price_per_hull < 0 || config.module_repair_price < 0 {
        return Err("game_config.json: repair prices cannot be negative".to_owned());
    }
    if config.safe_return_buffer < 0 {
        return Err("game_config.json: safe_return_buffer cannot be negative".to_owned());
    }
    if config.starting_hull <= 0 || config.starting_hull > config.max_hull {
        return Err("game_config.json: invalid starting hull".to_owned());
    }
    if config.workspace_scan_energy_cost < 0 {
        return Err("game_config.json: workspace scan energy cost cannot be negative".to_owned());
    }
    config.maintenance.validate()?;
    config.crew_training.validate()?;
    validate_market(config)?;
    validate_refinery(config)?;
    validate_insurance(config)?;
    validate_reconnaissance(config)?;
    validate_voyage_plan(config)?;
    if !(0..=100).contains(&config.risk.safe_danger_threshold) {
        return Err("game_config.json: invalid risk safe_danger_threshold".to_owned());
    }
    Ok(())
}

fn validate_market(config: &GameConfig) -> Result<(), String> {
    if config.market.hot_bonus_percent < 0
        || config.market.soft_penalty_percent < 0
        || config.market.soft_penalty_percent >= 100
    {
        return Err("game_config.json: invalid market tuning".to_owned());
    }
    Ok(())
}

fn validate_refinery(config: &GameConfig) -> Result<(), String> {
    if config.refinery.alloy_batch <= 0
        || config.refinery.electronics_batch <= 0
        || config.refinery.alloy_payout <= 0
        || config.refinery.electronics_payout <= 0
    {
        return Err("game_config.json: invalid refinery tuning".to_owned());
    }
    Ok(())
}

fn validate_insurance(config: &GameConfig) -> Result<(), String> {
    if config.insurance.premium_base < 0
        || config.insurance.premium_per_danger < 0
        || !(0..=100).contains(&config.insurance.coverage_percent)
        || config.insurance.damaged_module_payout < 0
    {
        return Err("game_config.json: invalid insurance tuning".to_owned());
    }
    Ok(())
}

fn validate_reconnaissance(config: &GameConfig) -> Result<(), String> {
    if config.reconnaissance.first_cost < 0
        || config.reconnaissance.cost_step < 0
        || config.reconnaissance.danger_reduction_per_level < 0
        || config.reconnaissance.max_level == 0
    {
        return Err("game_config.json: invalid reconnaissance tuning".to_owned());
    }
    Ok(())
}

fn validate_voyage_plan(config: &GameConfig) -> Result<(), String> {
    if config.voyage_plan.cautious_fuel_delta < 0
        || config.voyage_plan.cautious_danger_delta > 0
        || config.voyage_plan.expedited_fuel_delta > 0
        || config.voyage_plan.expedited_danger_delta < 0
    {
        return Err("game_config.json: invalid voyage plan tuning".to_owned());
    }
    Ok(())
}

fn validate_salvage_objects(data: &GameData) -> Result<(), String> {
    for (id, object) in data.salvage_objects.iter() {
        validate_salvage_object(id, object, data)?;
    }
    Ok(())
}

fn validate_salvage_object(
    id: &str,
    object: &SalvageObjectData,
    data: &GameData,
) -> Result<(), String> {
    validate_footprint(id, object.footprint, &data.config)?;
    if object.sale_value < 0 || object.alloy_yield < 0 || object.electronics_yield < 0 {
        return Err(format!("salvage object '{id}': negative economy value"));
    }
    if object.market_group.trim().is_empty() {
        return Err(format!("salvage object '{id}': market group is required"));
    }
    if let Some(module_id) = &object.install_module_id {
        if !data.modules.contains(module_id) {
            return Err(format!(
                "salvage object '{id}': missing install module '{module_id}'"
            ));
        }
    }
    if !object.mass_tons.is_finite() || object.mass_tons <= 0.0 {
        return Err(format!("salvage object '{id}': mass must be positive"));
    }
    if !(0..=100).contains(&object.integrity) || !(0..=100).contains(&object.extraction_difficulty)
    {
        return Err(format!(
            "salvage object '{id}': integrity and extraction difficulty must be 0..=100"
        ));
    }
    if !object.extraction_duration.is_finite() || object.extraction_duration <= 0.0 {
        return Err(format!(
            "salvage object '{id}': extraction duration must be positive"
        ));
    }
    if object.energy_cost < 0 {
        return Err(format!("salvage object '{id}': negative energy cost"));
    }
    if !matches!(
        object.transfer_mode.as_str(),
        "internal_cargo" | "external_clamp" | "tow"
    ) {
        return Err(format!(
            "salvage object '{id}': unknown transfer mode '{}'",
            object.transfer_mode
        ));
    }
    Ok(())
}

fn validate_modules(data: &GameData) -> Result<(), String> {
    for (id, module) in data.modules.iter() {
        validate_module(id, module, data)?;
    }
    Ok(())
}

fn validate_module(id: &str, module: &ModuleData, data: &GameData) -> Result<(), String> {
    validate_footprint(id, module.footprint, &data.config)?;
    if module.install_cost < 0
        || module.purchase_cost < 0
        || module.unlock_credits < 0
        || module.remove_cost < 0
        || module.external_capacity < 0
        || module.drone_support < 0
    {
        return Err(format!(
            "module '{id}': negative module cost, unlock, or capacity"
        ));
    }
    let is_starting_module = data
        .config
        .starting_modules
        .iter()
        .any(|starting| starting.module_id == id);
    if !is_starting_module && module.purchase_cost == 0 {
        return Err(format!(
            "module '{id}': non-starting modules need a positive purchase cost"
        ));
    }
    Ok(())
}

fn validate_sites(data: &GameData) -> Result<(), String> {
    for (id, site) in data.sites.iter() {
        validate_site(id, site, data)?;
    }
    Ok(())
}

fn validate_site(id: &str, site: &SiteData, data: &GameData) -> Result<(), String> {
    validate_site_overview(id, site, data)?;
    if site.candidate_salvage.len() < 5 {
        return Err(format!(
            "site '{id}': needs at least five salvage candidates"
        ));
    }
    if site.sections.is_empty() {
        return Err(format!(
            "site '{id}': at least one wreck section is required"
        ));
    }
    let section_ids: HashSet<&str> = site
        .sections
        .iter()
        .map(|section| section.id.as_str())
        .collect();
    if section_ids.len() != site.sections.len() {
        return Err(format!("site '{id}': duplicate wreck section id"));
    }
    for section in &site.sections {
        validate_section(id, site, section, &section_ids, data)?;
    }
    Ok(())
}

fn validate_site_overview(id: &str, site: &SiteData, data: &GameData) -> Result<(), String> {
    if site.fuel_cost < 0 || !(0..=100).contains(&site.danger) {
        return Err(format!("site '{id}': invalid fuel or danger"));
    }
    if !matches!(
        site.visual_theme.as_str(),
        "merchant" | "military" | "research"
    ) {
        return Err(format!(
            "site '{id}': unknown visual theme '{}'",
            site.visual_theme
        ));
    }
    if site.wreck_class.trim().is_empty() {
        return Err(format!("site '{id}': wreck class is required"));
    }
    if site.contract_reward < 0 {
        return Err(format!("site '{id}': negative contract reward"));
    }
    validate_contract(id, site, data)?;
    let mut seen = HashSet::new();
    for object_id in &site.candidate_salvage {
        if !data.salvage_objects.contains(object_id) {
            return Err(format!("site '{id}': missing salvage object '{object_id}'"));
        }
        if !seen.insert(object_id) {
            return Err(format!("site '{id}': duplicate salvage '{object_id}'"));
        }
    }
    Ok(())
}

fn validate_contract(id: &str, site: &SiteData, data: &GameData) -> Result<(), String> {
    if let Some(contract_target) = &site.contract_target {
        if site.contract_reward <= 0 {
            return Err(format!(
                "site '{id}': a contract target needs a positive reward"
            ));
        }
        if site.contract_brief.trim().is_empty() {
            return Err(format!("site '{id}': a contract target needs a briefing"));
        }
        if !data.salvage_objects.contains(contract_target) {
            return Err(format!(
                "site '{id}': missing contract target '{contract_target}'"
            ));
        }
        if !site
            .candidate_salvage
            .iter()
            .any(|target| target == contract_target)
        {
            return Err(format!(
                "site '{id}': contract target '{contract_target}' is not a candidate"
            ));
        }
    } else if site.contract_reward > 0 {
        return Err(format!(
            "site '{id}': a contract reward needs a contract target"
        ));
    }
    Ok(())
}

fn validate_section(
    site_id: &str,
    site: &SiteData,
    section: &WreckSectionData,
    section_ids: &HashSet<&str>,
    data: &GameData,
) -> Result<(), String> {
    if section.clearance_reward < 0 {
        return Err(format!(
            "site '{site_id}' section '{}': negative clearance reward",
            section.id
        ));
    }
    if let Some(capability) = &section.required_capability {
        if !data
            .modules
            .iter()
            .any(|(_, module)| module.capability.as_deref() == Some(capability.as_str()))
        {
            return Err(format!(
                "site '{site_id}' section '{}': missing capability '{}'",
                section.id, capability
            ));
        }
    }
    validate_section_connections(site_id, site, section, section_ids)?;
    validate_section_targets(site_id, section, data)?;
    validate_section_hazards(site_id, section)
}

fn validate_section_connections(
    site_id: &str,
    site: &SiteData,
    section: &WreckSectionData,
    section_ids: &HashSet<&str>,
) -> Result<(), String> {
    let mut neighbors = HashSet::new();
    for neighbor in &section.connected_sections {
        if !section_ids.contains(neighbor.as_str()) {
            return Err(format!(
                "site '{site_id}' section '{}': missing connected section '{neighbor}'",
                section.id
            ));
        }
        if neighbor == &section.id {
            return Err(format!(
                "site '{site_id}' section '{}': cannot connect to itself",
                section.id
            ));
        }
        if !neighbors.insert(neighbor.as_str()) {
            return Err(format!(
                "site '{site_id}' section '{}': duplicate connected section '{neighbor}'",
                section.id
            ));
        }
        let reciprocal = site.sections.iter().any(|candidate| {
            candidate.id == *neighbor
                && candidate
                    .connected_sections
                    .iter()
                    .any(|back| back == &section.id)
        });
        if !reciprocal {
            return Err(format!(
                "site '{site_id}' section '{}' connection to '{neighbor}' is not reciprocal",
                section.id
            ));
        }
    }
    Ok(())
}

fn validate_section_targets(
    site_id: &str,
    section: &WreckSectionData,
    data: &GameData,
) -> Result<(), String> {
    let mut section_targets = HashSet::new();
    for target in &section.candidate_targets {
        if !data.salvage_objects.contains(target) {
            return Err(format!(
                "site '{site_id}' section '{}': missing target '{target}'",
                section.id
            ));
        }
        if !section_targets.insert(target.as_str()) {
            return Err(format!(
                "site '{site_id}' section '{}': duplicate target '{target}'",
                section.id
            ));
        }
    }
    Ok(())
}

fn validate_section_hazards(site_id: &str, section: &WreckSectionData) -> Result<(), String> {
    for hazard in &section.hazard_tags {
        if !matches!(
            hazard.as_str(),
            "unstable_fuel"
                | "electrical_arcs"
                | "radiation"
                | "reactor_instability"
                | "moving_debris"
                | "automated_defenses"
                | "decompression"
                | "magnetic_interference"
        ) {
            return Err(format!(
                "site '{site_id}' section '{}': unknown hazard tag '{hazard}'",
                section.id
            ));
        }
    }
    Ok(())
}

fn validate_starting_modules(data: &GameData) -> Result<(), String> {
    let mut occupied = Vec::new();
    for start in &data.config.starting_modules {
        let module = data.modules.get(&start.module_id).ok_or_else(|| {
            format!(
                "game_config.json: missing starting module '{}'",
                start.module_id
            )
        })?;
        let footprint = module.footprint.rotated(start.rotation);
        for y in start.position.y..start.position.y + footprint.height {
            for x in start.position.x..start.position.x + footprint.width {
                validate_starting_cell(data, start, x, y, &mut occupied)?;
            }
        }
    }
    Ok(())
}

fn validate_starting_cell(
    data: &GameData,
    start: &StartingModule,
    x: i32,
    y: i32,
    occupied: &mut Vec<GridPosition>,
) -> Result<(), String> {
    if x < 0 || y < 0 || x >= data.config.grid_width || y >= data.config.grid_height {
        return Err(format!(
            "starting module '{}' is outside the grid",
            start.module_id
        ));
    }
    let cell = GridPosition::new(x, y);
    if occupied.contains(&cell) {
        return Err(format!(
            "starting module '{}' overlaps another module",
            start.module_id
        ));
    }
    occupied.push(cell);
    Ok(())
}

fn validate_footprint(id: &str, footprint: Footprint, config: &GameConfig) -> Result<(), String> {
    if footprint.width <= 0
        || footprint.height <= 0
        || footprint.width > config.grid_width
        || footprint.height > config.grid_height
    {
        return Err(format!("'{id}': footprint does not fit the base grid"));
    }
    Ok(())
}
