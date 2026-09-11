//! Authoritative runtime state, explicit screen states, and versioned saves.

pub mod career;
pub mod contracts;
pub mod crew;
pub mod drone_directive;
pub mod expedition;
mod expedition_state;
pub mod insurance;
pub mod ledger;
pub mod main_menu;
pub mod maintenance;
pub mod market;
pub mod pause;
pub mod port;
pub mod progression;
pub mod reconnaissance;
pub mod refinery;
pub mod reputation;
pub mod results;
pub mod return_policy;
pub mod salvage_packing;
pub mod scan_profile;
pub mod section_clearance;
pub mod site_selection;
pub mod survey;
pub mod validation;
pub mod workspace;
pub mod workspace_condition;
mod workspace_drones;
pub mod workspace_energy;
pub mod workspace_records;

pub use career::{CareerAward, CareerStats};
pub use crew::CrewRole;
pub use drone_directive::DroneDirective;
pub use expedition_state::ExpeditionState;
pub use return_policy::ReturnPolicy;
pub use scan_profile::WorkspaceScanProfile;
pub use workspace_records::{TargetSurveyNote, WorkspaceLogEntry, WorkspaceLogEvent};

use crate::data::{GameData, GridPosition};
use crate::engine::VoyagePlan;
use crate::engine::{installation, progression as engine_progression};
use crate::engine::{resolve_disposition, Disposition, RiskOutcome, RiskResult, ShipLayout};
use crate::state::workspace::TransferMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Port,
    SiteSelection,
    Travel,
    SalvageWorkspace,
    SalvagePacking,
    ReturnTravel,
    Results,
    Pause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)] // Destination names make the state graph self-documenting.
pub enum StateTransition {
    ToMainMenu,
    ToPort,
    ToSiteSelection,
    ToTravel,
    ToSalvageWorkspace,
    ToPacking,
    ToReturnTravel,
    ToResults,
    ToPause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyState {
    pub credits: i64,
    pub fuel: i32,
    pub alloy: i32,
    pub electronics: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteProgress {
    pub condition: i32,
    pub visits: u32,
    #[serde(default)]
    pub discovered_sections: Vec<String>,
    #[serde(default)]
    pub removed_targets: Vec<String>,
    #[serde(default)]
    pub operation_log: Vec<WorkspaceLogEntry>,
    #[serde(default)]
    pub surveyed_targets: Vec<TargetSurveyNote>,
    #[serde(default)]
    pub contract_completed: bool,
    #[serde(default)]
    pub contract_failed: bool,
    #[serde(default)]
    pub reconnaissance_level: u8,
    #[serde(default)]
    pub cleared_sections: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CargoStatus {
    Pending,
    Packed,
    LeftBehind,
    Discarded,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItem {
    pub object_id: String,
    pub status: CargoStatus,
    pub position: Option<GridPosition>,
    pub rotation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnedItem {
    pub object_id: String,
    pub position: GridPosition,
    pub rotation: u8,
    #[serde(default)]
    pub market_cycle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub object_id: String,
    pub disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoyageRecord {
    pub site_id: String,
    pub recovered_count: u32,
    pub recovered_value: i64,
    #[serde(default)]
    pub recovered_alloy: i32,
    #[serde(default)]
    pub recovered_electronics: i32,
    pub external_load: u32,
    pub risk_outcome: RiskOutcome,
    pub danger_score: i32,
    #[serde(default)]
    pub reconnaissance_level: u8,
    #[serde(default)]
    pub voyage_plan: VoyagePlan,
    pub contract_completed: bool,
    #[serde(default)]
    pub contract_failed: bool,
    #[serde(default)]
    pub scan_profile: WorkspaceScanProfile,
    #[serde(default)]
    pub drone_directive: DroneDirective,
    pub condition_after: i32,
    #[serde(default)]
    pub cleared_sections: Vec<String>,
    #[serde(default)]
    pub clearance_payout: i64,
    #[serde(default)]
    pub return_fuel: i32,
    #[serde(default)]
    pub market_cycle: u32,
    #[serde(default)]
    pub insured: bool,
    #[serde(default)]
    pub insurance_premium: i64,
    #[serde(default)]
    pub insurance_payout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    pub economy: EconomyState,
    pub ship_layout: ShipLayout,
    pub hull: i32,
    pub max_hull: i32,
    pub damaged_modules: Vec<String>,
    pub site_progress: HashMap<String, SiteProgress>,
    pub selected_site: Option<String>,
    #[serde(default)]
    pub briefing_voyage_plan: VoyagePlan,
    pub expedition: Option<ExpeditionState>,
    pub returned: Vec<ReturnedItem>,
    pub last_risk: Option<RiskResult>,
    pub decisions: Vec<DecisionRecord>,
    #[serde(default)]
    pub voyage_log: Vec<VoyageRecord>,
    #[serde(default)]
    pub career: CareerStats,
    #[serde(default)]
    pub crew_role: CrewRole,
    pub unlocked_modules: Vec<String>,
    pub milestone_reached: bool,
    #[serde(default)]
    pub reputation: i32,
    #[serde(default)]
    pub market_cycle: u32,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub session: GameSession,
}

impl GameSession {
    pub fn new(data: &GameData) -> Self {
        let mut ship_layout = ShipLayout::new(data.config.grid_width, data.config.grid_height);
        for starting in &data.config.starting_modules {
            let module = data
                .modules
                .get(&starting.module_id)
                .expect("validated starting module");
            ship_layout
                .place(
                    module.id.clone(),
                    module.footprint,
                    starting.position,
                    starting.rotation,
                    true,
                )
                .expect("validated starting layout");
        }
        let site_progress = data
            .sites
            .iter()
            .map(|(id, site)| {
                (
                    id.clone(),
                    SiteProgress {
                        condition: site.condition,
                        visits: 0,
                        discovered_sections: Vec::new(),
                        removed_targets: Vec::new(),
                        operation_log: Vec::new(),
                        surveyed_targets: Vec::new(),
                        contract_completed: false,
                        contract_failed: false,
                        reconnaissance_level: 0,
                        cleared_sections: Vec::new(),
                    },
                )
            })
            .collect();
        let unlocked_modules = data
            .config
            .starting_modules
            .iter()
            .map(|module| module.module_id.clone())
            .collect();
        let mut session = Self {
            economy: EconomyState {
                credits: data.config.starting_credits,
                fuel: data.config.starting_fuel,
                alloy: 0,
                electronics: 0,
            },
            ship_layout,
            hull: data.config.starting_hull,
            max_hull: data.config.max_hull,
            damaged_modules: Vec::new(),
            site_progress,
            selected_site: None,
            briefing_voyage_plan: VoyagePlan::Standard,
            expedition: None,
            returned: Vec::new(),
            last_risk: None,
            decisions: Vec::new(),
            voyage_log: Vec::new(),
            career: CareerStats::default(),
            crew_role: CrewRole::default(),
            unlocked_modules,
            milestone_reached: false,
            reputation: 0,
            market_cycle: 0,
            seed: 7,
        };
        session.refresh_module_unlocks(data);
        session
    }

    pub fn from_save(save: SaveData, data: &GameData) -> Result<Self, String> {
        let mut session = save.session;
        if session.career.is_empty() && !session.voyage_log.is_empty() {
            session.career = CareerStats::from_voyage_log(&session.voyage_log);
        }
        session.career.validate()?;
        if let Some(expedition) = session.expedition.as_mut() {
            if expedition.workspace_section.is_empty() {
                expedition.workspace_section = data
                    .sites
                    .get(&expedition.site_id)
                    .and_then(|site| site.sections.first())
                    .map_or_else(String::new, |section| section.id.clone());
            }
        }
        if session.ship_layout.width != data.config.grid_width
            || session.ship_layout.height != data.config.grid_height
        {
            return Err("save uses an incompatible ship grid".to_owned());
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
        let mut checked = ShipLayout::new(session.ship_layout.width, session.ship_layout.height);
        for item in &session.ship_layout.placements {
            let footprint = if let Some(object_id) = item.id.strip_prefix("cargo:") {
                data.salvage_objects
                    .get(object_id)
                    .ok_or_else(|| format!("save references missing salvage '{object_id}'"))?
                    .footprint
            } else {
                data.modules
                    .get(&item.id)
                    .ok_or_else(|| format!("save references missing module '{}'", item.id))?
                    .footprint
            };
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
        validation::validate_saved_runtime(&session, &checked, data)?;
        if session.economy.fuel > session.max_fuel(data)
            || session.hull > session.max_hull_with_modules(data)
        {
            return Err("save exceeds the ship's current fuel or hull capacity".to_owned());
        }
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

    pub fn module_stats(&self, data: &GameData) -> engine_progression::ModuleStats {
        engine_progression::stats_from_layout(&self.ship_layout, data, &self.damaged_modules)
    }

    pub fn max_hull_with_modules(&self, data: &GameData) -> i32 {
        self.max_hull + self.module_stats(data).hull
    }

    pub fn effective_fuel_cost(&self, site_id: &str, data: &GameData) -> Option<i32> {
        self.effective_fuel_cost_with_plan(site_id, data, VoyagePlan::Standard)
    }

    pub fn effective_fuel_cost_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> Option<i32> {
        let site = data.sites.get(site_id)?;
        let stats = self.module_stats(data);
        Some(plan.adjust_fuel(
            (site.fuel_cost - stats.fuel_efficiency + self.crew_fuel_delta()).max(1),
            &data.config.voyage_plan,
        ))
    }

    pub fn departure_fuel_required(&self, site_id: &str, data: &GameData) -> Option<i32> {
        Some(self.effective_fuel_cost(site_id, data)? + data.config.safe_return_buffer)
    }

    pub fn departure_fuel_required_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> Option<i32> {
        Some(
            self.effective_fuel_cost_with_plan(site_id, data, plan)?
                + data.config.safe_return_buffer,
        )
    }

    pub fn can_depart(&self, site_id: &str, data: &GameData) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && self
                .departure_fuel_required(site_id, data)
                .is_some_and(|required| self.economy.fuel >= required)
    }

    pub fn can_depart_with_plan(&self, site_id: &str, data: &GameData, plan: VoyagePlan) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && self
                .departure_fuel_required_with_plan(site_id, data, plan)
                .is_some_and(|required| self.economy.fuel >= required)
    }

    pub fn begin_expedition(&mut self, site_id: &str, data: &GameData) -> Result<String, String> {
        if !self.can_depart(site_id, data) {
            return Err("you need enough fuel for the trip and a safe return".to_owned());
        }
        self.begin_expedition_with_coverage(site_id, data, false)
    }

    pub fn begin_expedition_with_coverage(
        &mut self,
        site_id: &str,
        data: &GameData,
        insured: bool,
    ) -> Result<String, String> {
        self.begin_expedition_with_plan(site_id, data, insured, VoyagePlan::Standard)
    }

    pub fn begin_expedition_with_plan(
        &mut self,
        site_id: &str,
        data: &GameData,
        insured: bool,
        plan: VoyagePlan,
    ) -> Result<String, String> {
        expedition::begin_expedition(self, site_id, data, insured, plan)
    }

    pub fn auto_place(&mut self, object_id: &str, data: &GameData) -> Result<String, String> {
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let layout_id = format!("cargo:{object_id}");
        let Some((position, rotation)) =
            self.ship_layout
                .first_fit(&layout_id, object.footprint, object.rotatable)
        else {
            return Err(format!(
                "{} has no open fit in the ship grid",
                object.display_name
            ));
        };
        self.place_cargo(object_id, position, rotation, data)
    }

    pub fn place_cargo(
        &mut self,
        object_id: &str,
        position: GridPosition,
        rotation: u8,
        data: &GameData,
    ) -> Result<String, String> {
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        if TransferMode::from_target(object).uses_external_rig() {
            let used = self.external_cargo_count(data, Some(object_id));
            let capacity = self.external_capacity(data);
            if used >= capacity {
                return Err(format!(
                    "No external clamp is free ({used}/{capacity}). Leave this load behind or upgrade the ship."
                ));
            }
        }
        let previous = self
            .expedition
            .as_ref()
            .and_then(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .find(|item| item.object_id == object_id)
            })
            .and_then(|item| item.position.map(|old| (old, item.rotation)));
        if previous.is_none() {
            let is_pending = self.expedition.as_ref().is_some_and(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .any(|item| item.object_id == object_id && item.status == CargoStatus::Pending)
            });
            if !is_pending {
                return Err("that salvage is no longer available".to_owned());
            }
        }
        let layout_id = cargo_layout_id(object_id);
        self.ship_layout.remove(&layout_id);
        if let Err(error) =
            self.ship_layout
                .can_place(&layout_id, object.footprint, position, rotation)
        {
            if let Some((old_position, old_rotation)) = previous {
                let _ = self.ship_layout.place(
                    layout_id.clone(),
                    object.footprint,
                    old_position,
                    old_rotation,
                    false,
                );
            }
            return Err(error.to_string());
        }
        self.ship_layout
            .place(layout_id, object.footprint, position, rotation, false)
            .map_err(|error| error.to_string())?;
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        let cargo = expedition
            .cargo
            .iter_mut()
            .find(|item| item.object_id == object_id)
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        cargo.status = CargoStatus::Packed;
        cargo.position = Some(position);
        cargo.rotation = rotation % 2;
        Ok(format!("Packed {}", object.display_name))
    }

    pub fn set_cargo_status(
        &mut self,
        object_id: &str,
        status: CargoStatus,
    ) -> Result<String, String> {
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        let cargo = expedition
            .cargo
            .iter_mut()
            .find(|item| item.object_id == object_id)
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        self.ship_layout.remove(&cargo_layout_id(object_id));
        cargo.status = status;
        cargo.position = None;
        Ok(format!("{}: {:?}", object_id, status))
    }

    pub fn rotate_cargo(&mut self, object_id: &str, data: &GameData) -> Result<String, String> {
        let (rotation, position) = self
            .expedition
            .as_ref()
            .and_then(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .find(|item| item.object_id == object_id)
            })
            .map(|item| (item.rotation, item.position))
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let next_rotation = rotation.wrapping_add(1) % 2;
        if !object.rotatable {
            return Err(format!("{} cannot rotate", object.display_name));
        }
        let Some(position) = position else {
            let expedition = self.expedition.as_mut().expect("checked above");
            let cargo = expedition
                .cargo
                .iter_mut()
                .find(|item| item.object_id == object_id)
                .expect("checked above");
            cargo.rotation = next_rotation;
            return Ok(format!("Rotated {}", object.display_name));
        };
        self.place_cargo(object_id, position, next_rotation, data)?;
        Ok(format!("Rotated {}", object.display_name))
    }

    pub fn leave_all_pending(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self
            .expedition
            .as_ref()
            .ok_or_else(|| "there is no active expedition".to_owned())?
            .cargo
            .iter()
            .filter(|item| item.status == CargoStatus::Pending)
            .map(|item| item.object_id.clone())
            .collect();
        for id in ids {
            self.set_cargo_status(&id, CargoStatus::LeftBehind)?;
        }
        Ok(())
    }

    pub fn pending_count(&self) -> usize {
        self.expedition
            .as_ref()
            .map(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .filter(|item| item.status == CargoStatus::Pending)
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn dispose(
        &mut self,
        object_id: &str,
        disposition: Disposition,
        data: &GameData,
    ) -> Result<String, String> {
        let index = self
            .returned
            .iter()
            .position(|item| item.object_id == object_id)
            .ok_or_else(|| "that item has already been resolved".to_owned())?;
        let returned = self.returned[index].clone();
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let mut delta = resolve_disposition(object, disposition)?;
        if disposition == Disposition::Sell {
            delta.credits = crate::engine::market::quote_for(
                object,
                returned.market_cycle,
                &data.config.market,
            )
            .sale_value;
        }
        match disposition {
            Disposition::Sell | Disposition::BreakDown => {
                self.ship_layout.remove(&cargo_layout_id(object_id));
                self.economy.credits += delta.credits;
                self.economy.alloy += delta.alloy;
                self.economy.electronics += delta.electronics;
                if disposition == Disposition::Sell {
                    self.career.record_sale(delta.credits);
                }
            }
            Disposition::Install => {
                let module_id = object
                    .install_module_id
                    .as_ref()
                    .ok_or_else(|| "that object has no installation effect".to_owned())?;
                let module = data
                    .modules
                    .get(module_id)
                    .ok_or_else(|| format!("missing install module '{module_id}'"))?;
                if self
                    .ship_layout
                    .placements
                    .iter()
                    .any(|item| item.id == *module_id)
                {
                    return Err(format!("{} is already installed", module.display_name));
                }
                if self.economy.credits < module.install_cost {
                    return Err(format!(
                        "installation requires {} credits",
                        module.install_cost
                    ));
                }
                installation::install_module(
                    &mut self.ship_layout,
                    &cargo_layout_id(object_id),
                    module_id,
                    module.footprint,
                    returned.position,
                    returned.rotation,
                )?;
                self.economy.credits -= module.install_cost;
                self.career
                    .record_module_change(i64::from(module.install_cost));
                if !self.unlocked_modules.contains(module_id) {
                    self.unlocked_modules.push(module_id.clone());
                }
            }
        }
        self.returned.remove(index);
        let mut label = match disposition {
            Disposition::Sell => {
                format!("Sold {} for {} credits", object.display_name, delta.credits)
            }
            Disposition::Install => format!("Installed {}", object.display_name),
            Disposition::BreakDown => format!("Broke down {} for materials", object.display_name),
        };
        self.decisions.push(DecisionRecord {
            object_id: object_id.to_owned(),
            disposition: format!("{disposition:?}"),
        });
        let newly_unlocked = self.refresh_module_unlocks(data);
        if !newly_unlocked.is_empty() {
            let names = newly_unlocked
                .iter()
                .filter_map(|id| data.modules.get(id))
                .map(|module| module.display_name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            label.push_str(&format!(". Blueprint unlocked: {names}."));
        }
        Ok(label)
    }
}

fn cargo_layout_id(object_id: &str) -> String {
    format!("cargo:{object_id}")
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: serde_json::Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);
    let mut save: SaveData = serde_json::from_value(payload)
        .map_err(|error| format!("Unsupported save format {:?}: {error}", detected_version))?;
    save.version = data.config.version.clone();
    GameSession::from_save(save.clone(), data)?;
    Ok(save)
}

#[cfg(test)]
mod tests;
