//! Immutable, embedded content definitions for Salvage Captain.

pub mod crew_training;
pub mod maintenance;
pub mod salvage_ui;
pub mod selection_ui;
pub mod transit_ui;
mod validation;
pub mod voyage_plan;
pub use maintenance::MaintenanceTuning;
pub use voyage_plan::VoyagePlanTuning;

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{
    load_embedded_json, load_embedded_json_labeled, DataRegistry,
};
use serde::{Deserialize, Serialize};

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
    #[serde(default = "default_module_repair_price")]
    pub module_repair_price: i32,
    pub starting_hull: i32,
    pub max_hull: i32,
    pub safe_return_buffer: i32,
    #[serde(default = "default_workspace_scan_energy_cost")]
    pub workspace_scan_energy_cost: i32,
    #[serde(default)]
    pub market: MarketTuning,
    #[serde(default)]
    pub refinery: RefineryTuning,
    #[serde(default)]
    pub insurance: InsuranceTuning,
    #[serde(default)]
    pub reconnaissance: ReconnaissanceTuning,
    #[serde(default)]
    pub voyage_plan: VoyagePlanTuning,
    #[serde(default)]
    pub maintenance: MaintenanceTuning,
    #[serde(default)]
    pub crew_training: crew_training::CrewTrainingTuning,
    pub progression_credit_threshold: i64,
    pub risk: RiskTuning,
    pub starting_modules: Vec<StartingModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTuning {
    #[serde(default = "default_market_hot_bonus")]
    pub hot_bonus_percent: i32,
    #[serde(default = "default_market_soft_penalty")]
    pub soft_penalty_percent: i32,
}

impl Default for MarketTuning {
    fn default() -> Self {
        Self {
            hot_bonus_percent: default_market_hot_bonus(),
            soft_penalty_percent: default_market_soft_penalty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefineryTuning {
    #[serde(default = "default_alloy_batch")]
    pub alloy_batch: i32,
    #[serde(default = "default_alloy_payout")]
    pub alloy_payout: i64,
    #[serde(default = "default_electronics_batch")]
    pub electronics_batch: i32,
    #[serde(default = "default_electronics_payout")]
    pub electronics_payout: i64,
}

impl Default for RefineryTuning {
    fn default() -> Self {
        Self {
            alloy_batch: default_alloy_batch(),
            alloy_payout: default_alloy_payout(),
            electronics_batch: default_electronics_batch(),
            electronics_payout: default_electronics_payout(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceTuning {
    #[serde(default = "default_insurance_premium_base")]
    pub premium_base: i64,
    #[serde(default = "default_insurance_premium_per_danger")]
    pub premium_per_danger: i64,
    #[serde(default = "default_insurance_coverage_percent")]
    pub coverage_percent: i32,
    #[serde(default = "default_insurance_damage_payout")]
    pub damaged_module_payout: i64,
}

impl Default for InsuranceTuning {
    fn default() -> Self {
        Self {
            premium_base: default_insurance_premium_base(),
            premium_per_danger: default_insurance_premium_per_danger(),
            coverage_percent: default_insurance_coverage_percent(),
            damaged_module_payout: default_insurance_damage_payout(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconnaissanceTuning {
    #[serde(default = "default_reconnaissance_first_cost")]
    pub first_cost: i64,
    #[serde(default = "default_reconnaissance_cost_step")]
    pub cost_step: i64,
    #[serde(default = "default_reconnaissance_danger_reduction")]
    pub danger_reduction_per_level: i32,
    #[serde(default = "default_reconnaissance_max_level")]
    pub max_level: u8,
}

impl Default for ReconnaissanceTuning {
    fn default() -> Self {
        Self {
            first_cost: default_reconnaissance_first_cost(),
            cost_step: default_reconnaissance_cost_step(),
            danger_reduction_per_level: default_reconnaissance_danger_reduction(),
            max_level: default_reconnaissance_max_level(),
        }
    }
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
    #[serde(default)]
    pub clearance_reward: i64,
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
    #[serde(default)]
    pub market_group: String,
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
    #[serde(default)]
    pub unlock_credits: i64,
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
    pub salvage_ui: salvage_ui::SalvageUiCopy,
    pub transit_ui: transit_ui::TransitUiCopy,
    pub selection_ui: selection_ui::SelectionUiCopy,
    pub sites: DataRegistry<SiteData>,
    pub salvage_objects: DataRegistry<SalvageObjectData>,
    pub modules: DataRegistry<ModuleData>,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let data = Self {
            salvage_ui: load_embedded_json_labeled(
                "salvage_ui.json",
                macroquad_toolkit::include_json_str!("../assets/data/salvage_ui.json"),
            )?,
            selection_ui: load_embedded_json_labeled(
                "selection_ui.json",
                macroquad_toolkit::include_json_str!("../assets/data/selection_ui.json"),
            )?,
            transit_ui: load_embedded_json_labeled(
                "transit_ui.json",
                macroquad_toolkit::include_json_str!("../assets/data/transit_ui.json"),
            )?,
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
        validation::validate(self)
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

fn default_workspace_scan_energy_cost() -> i32 {
    1
}

fn default_module_repair_price() -> i32 {
    55
}

fn default_market_hot_bonus() -> i32 {
    25
}

fn default_market_soft_penalty() -> i32 {
    15
}

fn default_alloy_batch() -> i32 {
    5
}

fn default_alloy_payout() -> i64 {
    100
}

fn default_electronics_batch() -> i32 {
    3
}

fn default_electronics_payout() -> i64 {
    120
}

fn default_insurance_premium_base() -> i64 {
    35
}

fn default_insurance_premium_per_danger() -> i64 {
    2
}

fn default_insurance_coverage_percent() -> i32 {
    75
}

fn default_insurance_damage_payout() -> i64 {
    90
}

fn default_reconnaissance_first_cost() -> i64 {
    70
}

fn default_reconnaissance_cost_step() -> i64 {
    50
}

fn default_reconnaissance_danger_reduction() -> i32 {
    8
}

fn default_reconnaissance_max_level() -> u8 {
    2
}
