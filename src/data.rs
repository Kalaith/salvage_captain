//! Immutable, embedded content definitions for Salvage Captain.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{
    load_embedded_json, load_embedded_json_labeled, DataRegistry,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const CONFIG_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
const SITES_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/sites.json");
const SALVAGE_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/salvage_objects.json");
const MODULES_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/modules.json");
const TEXTURES_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/texture_manifest.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Footprint {
    pub width: i32,
    pub height: i32,
}

impl Footprint {
    pub fn rotated(self, rotation: u8) -> Self {
        if rotation.is_multiple_of(2) {
            self
        } else {
            Self {
                width: self.height,
                height: self.width,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub grid_width: i32,
    pub grid_height: i32,
    pub starting_credits: i64,
    pub starting_fuel: i32,
    pub max_fuel: i32,
    pub refuel_price_per_unit: i32,
    pub repair_price_per_hull: i32,
    pub starting_hull: i32,
    pub max_hull: i32,
    pub safe_return_buffer: i32,
    pub progression_credit_threshold: i64,
    pub risk: RiskTuning,
    pub starting_modules: Vec<StartingModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskTuning {
    pub safe_danger_threshold: i32,
    pub ordinary_return_weight: i32,
    pub damaged_module_weight: i32,
    pub lost_salvage_weight: i32,
    pub emergency_repair_weight: i32,
    pub forced_abandon_weight: i32,
    #[serde(default = "default_external_cargo_risk_per_item")]
    pub external_cargo_risk_per_item: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingModule {
    pub module_id: String,
    pub position: GridPosition,
    pub rotation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteData {
    pub id: String,
    pub display_name: String,
    pub category: String,
    pub description: String,
    pub fuel_cost: i32,
    pub danger: i32,
    pub condition: i32,
    pub known_reward: String,
    pub candidate_salvage: Vec<String>,
    #[serde(default)]
    pub wreck_class: String,
    #[serde(default)]
    pub visual_theme: String,
    #[serde(default)]
    pub background_asset: String,
    #[serde(default)]
    pub hull_asset: String,
    #[serde(default)]
    pub arrival_text: String,
    #[serde(default)]
    pub contract_target: Option<String>,
    #[serde(default)]
    pub contract_reward: i64,
    #[serde(default)]
    pub contract_brief: String,
    #[serde(default)]
    pub sections: Vec<WreckSectionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WreckSectionData {
    pub id: String,
    pub display_name: String,
    #[serde(default)]
    pub connected_sections: Vec<String>,
    #[serde(default)]
    pub required_capability: Option<String>,
    #[serde(default)]
    pub candidate_targets: Vec<String>,
    #[serde(default)]
    pub hazard_tags: Vec<String>,
    #[serde(default)]
    pub arrival_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalvageObjectData {
    pub id: String,
    pub display_name: String,
    pub category: String,
    pub description: String,
    pub footprint: Footprint,
    pub rotatable: bool,
    pub sale_value: i64,
    pub alloy_yield: i32,
    pub electronics_yield: i32,
    pub install_module_id: Option<String>,
    pub sell_only: bool,
    #[serde(default)]
    pub workspace_name: String,
    #[serde(default = "default_mass_tons")]
    pub mass_tons: f32,
    #[serde(default = "default_integrity")]
    pub integrity: i32,
    #[serde(default = "default_extraction_difficulty")]
    pub extraction_difficulty: i32,
    #[serde(default = "default_extraction_duration")]
    pub extraction_duration: f32,
    #[serde(default)]
    pub energy_cost: i32,
    #[serde(default)]
    pub required_capability: Option<String>,
    #[serde(default)]
    pub hazard: Option<String>,
    #[serde(default)]
    pub hazard_consequence: String,
    #[serde(default)]
    pub visual_silhouette: String,
    #[serde(default = "default_transfer_mode")]
    pub transfer_mode: String,
    #[serde(default)]
    pub animation_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleData {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub footprint: Footprint,
    pub effect: ModuleEffect,
    pub install_cost: i64,
    #[serde(default)]
    pub purchase_cost: i64,
    pub remove_cost: i64,
    #[serde(default)]
    pub mount: String,
    #[serde(default)]
    pub visual_kind: String,
    #[serde(default)]
    pub external_capacity: i32,
    #[serde(default)]
    pub drone_support: i32,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ModuleEffect {
    CargoSpace,
    FuelCapacity(i32),
    FuelEfficiency(i32),
    Hull(i32),
    Power(i32),
    Scanning(i32),
    Shielding(i32),
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub sites: DataRegistry<SiteData>,
    pub salvage_objects: DataRegistry<SalvageObjectData>,
    pub modules: DataRegistry<ModuleData>,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let data = Self {
            config: load_embedded_json_labeled("game_config.json", CONFIG_JSON)?,
            sites: DataRegistry::from_embedded_json(SITES_JSON, "id")?,
            salvage_objects: DataRegistry::from_embedded_json(SALVAGE_JSON, "id")?,
            modules: DataRegistry::from_embedded_json(MODULES_JSON, "id")?,
            texture_manifest: load_embedded_json(TEXTURES_JSON)?,
        };
        data.validate()?;
        Ok(data)
    }

    pub fn validate(&self) -> Result<(), String> {
        let config = &self.config;
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
        if risk_weights.iter().any(|weight| *weight < 0) || risk_weights.iter().sum::<i32>() != 100
        {
            return Err("risk tuning weights must be non-negative and total 100".to_owned());
        }
        if config.grid_width <= 0 || config.grid_height <= 0 {
            return Err("game_config.json: grid dimensions must be positive".to_owned());
        }
        if config.starting_fuel < 0 || config.starting_fuel > config.max_fuel {
            return Err("game_config.json: starting_fuel must fit max_fuel".to_owned());
        }
        if config.starting_hull <= 0 || config.starting_hull > config.max_hull {
            return Err("game_config.json: invalid starting hull".to_owned());
        }
        if !(0..=100).contains(&config.risk.safe_danger_threshold) {
            return Err("game_config.json: invalid risk safe_danger_threshold".to_owned());
        }
        let risk_weights = [
            config.risk.ordinary_return_weight,
            config.risk.damaged_module_weight,
            config.risk.lost_salvage_weight,
            config.risk.emergency_repair_weight,
            config.risk.forced_abandon_weight,
        ];
        if risk_weights.iter().any(|weight| *weight < 0) || risk_weights.iter().sum::<i32>() != 100
        {
            return Err(
                "game_config.json: risk weights must be non-negative and sum to 100".to_owned(),
            );
        }
        for (id, object) in self.salvage_objects.iter() {
            validate_footprint(id, object.footprint, config)?;
            if object.sale_value < 0 || object.alloy_yield < 0 || object.electronics_yield < 0 {
                return Err(format!("salvage object '{id}': negative economy value"));
            }
            if let Some(module_id) = &object.install_module_id {
                if !self.modules.contains(module_id) {
                    return Err(format!(
                        "salvage object '{id}': missing install module '{module_id}'"
                    ));
                }
            }
            if !object.mass_tons.is_finite() || object.mass_tons <= 0.0 {
                return Err(format!("salvage object '{id}': mass must be positive"));
            }
            if !(0..=100).contains(&object.integrity)
                || !(0..=100).contains(&object.extraction_difficulty)
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
        }
        for (id, module) in self.modules.iter() {
            validate_footprint(id, module.footprint, config)?;
            if module.install_cost < 0
                || module.purchase_cost < 0
                || module.remove_cost < 0
                || module.external_capacity < 0
                || module.drone_support < 0
            {
                return Err(format!("module '{id}': negative module cost or capacity"));
            }
            let is_starting_module = config
                .starting_modules
                .iter()
                .any(|starting| starting.module_id == *id);
            if !is_starting_module && module.purchase_cost == 0 {
                return Err(format!(
                    "module '{id}': non-starting modules need a positive purchase cost"
                ));
            }
        }
        for (id, site) in self.sites.iter() {
            if site.fuel_cost < 0 || !(0..=100).contains(&site.danger) {
                return Err(format!("site '{id}': invalid fuel or danger"));
            }
            if site.contract_reward < 0 {
                return Err(format!("site '{id}': negative contract reward"));
            }
            if let Some(contract_target) = &site.contract_target {
                if site.contract_reward <= 0 {
                    return Err(format!(
                        "site '{id}': a contract target needs a positive reward"
                    ));
                }
                if site.contract_brief.trim().is_empty() {
                    return Err(format!("site '{id}': a contract target needs a briefing"));
                }
                if !self.salvage_objects.contains(contract_target) {
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
            let mut seen = HashSet::new();
            for object_id in &site.candidate_salvage {
                if !self.salvage_objects.contains(object_id) {
                    return Err(format!("site '{id}': missing salvage object '{object_id}'"));
                }
                if !seen.insert(object_id) {
                    return Err(format!("site '{id}': duplicate salvage '{object_id}'"));
                }
            }
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
                if let Some(capability) = &section.required_capability {
                    if !self.modules.iter().any(|(_, module)| {
                        module.capability.as_deref() == Some(capability.as_str())
                    }) {
                        return Err(format!(
                            "site '{id}' section '{}': missing capability '{}'",
                            section.id, capability
                        ));
                    }
                }
                let mut neighbors = HashSet::new();
                for neighbor in &section.connected_sections {
                    if !section_ids.contains(neighbor.as_str()) {
                        return Err(format!(
                            "site '{id}' section '{}': missing connected section '{neighbor}'",
                            section.id
                        ));
                    }
                    if neighbor == &section.id {
                        return Err(format!(
                            "site '{id}' section '{}': cannot connect to itself",
                            section.id
                        ));
                    }
                    if !neighbors.insert(neighbor.as_str()) {
                        return Err(format!(
                            "site '{id}' section '{}': duplicate connected section '{neighbor}'",
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
                            "site '{id}' section '{}' connection to '{neighbor}' is not reciprocal",
                            section.id
                        ));
                    }
                }
                let mut section_targets = HashSet::new();
                for target in &section.candidate_targets {
                    if !self.salvage_objects.contains(target) {
                        return Err(format!(
                            "site '{id}' section '{}': missing target '{target}'",
                            section.id
                        ));
                    }
                    if !section_targets.insert(target.as_str()) {
                        return Err(format!(
                            "site '{id}' section '{}': duplicate target '{target}'",
                            section.id
                        ));
                    }
                }
            }
        }
        let mut occupied = Vec::new();
        for start in &config.starting_modules {
            let module = self.modules.get(&start.module_id).ok_or_else(|| {
                format!(
                    "game_config.json: missing starting module '{}'",
                    start.module_id
                )
            })?;
            let footprint = module.footprint.rotated(start.rotation);
            for y in start.position.y..start.position.y + footprint.height {
                for x in start.position.x..start.position.x + footprint.width {
                    if x < 0 || y < 0 || x >= config.grid_width || y >= config.grid_height {
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
                }
            }
        }
        Ok(())
    }

    pub fn ordered_sites(&self) -> Vec<&SiteData> {
        let mut sites: Vec<_> = self.sites.iter().map(|(_, site)| site).collect();
        sites.sort_by(|left, right| left.id.cmp(&right.id));
        sites
    }
}

fn default_mass_tons() -> f32 {
    1.0
}

fn default_integrity() -> i32 {
    75
}

fn default_extraction_difficulty() -> i32 {
    25
}

fn default_extraction_duration() -> f32 {
    4.0
}

fn default_transfer_mode() -> String {
    "internal_cargo".to_owned()
}

fn default_external_cargo_risk_per_item() -> i32 {
    8
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

#[cfg(test)]
mod tests;
