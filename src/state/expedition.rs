//! Departure setup for fuel, seeded risk, salvage manifests, and coverage.

use super::{ExpeditionState, GameSession, ReturnPolicy, WorkspaceLogEvent};
use crate::data::{GameData, SiteData};
use crate::engine::{danger_after_intel, generate_salvage, resolve_risk, VoyagePlan};
use crate::state::{DroneDirective, WorkspaceScanProfile};

struct DepartureCosts {
    fuel: i32,
    insurance_premium: i64,
}

pub(super) fn begin_expedition(
    session: &mut GameSession,
    site_id: &str,
    data: &GameData,
    insured: bool,
    voyage_plan: VoyagePlan,
    contract_accepted: bool,
) -> Result<String, String> {
    let site = data
        .sites
        .get(site_id)
        .ok_or_else(|| format!("unknown salvage site '{site_id}'"))?;
    let costs = departure_costs(session, site_id, data, insured, voyage_plan)?;
    session.economy.fuel -= costs.fuel;
    session.economy.credits -= costs.insurance_premium;

    let seed = session.seed;
    session.seed = session.seed.wrapping_add(1);
    let stats = session.module_stats(data);
    let scan_profile =
        WorkspaceScanProfile::from_capability(session.has_capability("scanner_array", data));
    let workspace_energy_capacity = stats.power.max(1) * 6;
    let condition = site_condition(session, site);
    let removed_targets = removed_targets(session, site_id);
    let reconnaissance_level = session.reconnaissance_level(site_id);
    let departure_danger = departure_danger(
        session,
        site,
        data,
        voyage_plan,
        condition,
        reconnaissance_level,
    );
    let risk = resolve_risk(
        seed,
        departure_danger,
        session.hull,
        stats,
        &data.config.risk,
    );
    let salvage_count = generate_salvage(site, seed, &removed_targets).len();
    let first_section = site
        .sections
        .first()
        .map_or_else(String::new, |section| section.id.clone());
    session.expedition = Some(ExpeditionState {
        site_id: site_id.to_owned(),
        cargo: Vec::new(),
        workspace_transfer: None,
        risk,
        seed,
        workspace_section: first_section.clone(),
        workspace_scanned: false,
        revealed_targets: Vec::new(),
        stabilized_targets: Vec::new(),
        scan_profile,
        drones_deployed: false,
        drone_directive: DroneDirective::default(),
        workspace_energy: workspace_energy_capacity,
        workspace_energy_capacity,
        power_cycles_used: 0,
        insured,
        contract_accepted,
        voyage_plan,
        return_policy: ReturnPolicy::default(),
    });
    session.append_workspace_log(
        site_id,
        WorkspaceLogEvent::Departed,
        Some(first_section.as_str()),
        None,
    );
    session.selected_site = Some(site_id.to_owned());
    Ok(departure_message(
        site,
        costs,
        salvage_count,
        reconnaissance_level,
        voyage_plan,
        session,
        contract_accepted,
    ))
}

fn departure_costs(
    session: &GameSession,
    site_id: &str,
    data: &GameData,
    insured: bool,
    voyage_plan: VoyagePlan,
) -> Result<DepartureCosts, String> {
    if session.wreck_depleted(site_id, data) {
        return Err(data.discovery.copy.depleted_departure.clone());
    }
    if !session.can_depart_with_plan(site_id, data, voyage_plan) {
        return Err("you need enough fuel for the trip and a safe return".to_owned());
    }
    let site = data
        .sites
        .get(site_id)
        .ok_or_else(|| format!("unknown salvage site '{site_id}'"))?;
    let insurance_premium = if insured {
        let quote = session
            .insurance_quote_with_plan(site_id, data, voyage_plan)
            .ok_or_else(|| "that wreck cannot be insured".to_owned())?;
        if session.economy.credits < quote.premium {
            return Err(format!(
                "insurance requires {} credits before departure",
                quote.premium
            ));
        }
        quote.premium
    } else {
        0
    };
    let fuel = session
        .effective_fuel_cost_with_plan(site_id, data, voyage_plan)
        .unwrap_or(site.fuel_cost);
    Ok(DepartureCosts {
        fuel,
        insurance_premium,
    })
}

fn site_condition(session: &GameSession, site: &SiteData) -> i32 {
    session
        .site_progress
        .get(&site.id)
        .map_or(site.condition, |progress| progress.condition)
}

fn removed_targets(session: &GameSession, site_id: &str) -> Vec<String> {
    session
        .site_progress
        .get(site_id)
        .map_or_else(Vec::new, |progress| progress.removed_targets.clone())
}

fn departure_danger(
    session: &GameSession,
    site: &SiteData,
    data: &GameData,
    voyage_plan: VoyagePlan,
    condition: i32,
    reconnaissance_level: u8,
) -> i32 {
    let condition_penalty = (100 - condition).max(0) / 4;
    let route_danger = danger_after_intel(
        site.danger + condition_penalty,
        reconnaissance_level,
        &data.config.reconnaissance,
    )
    .saturating_sub(session.route_familiarity_danger_reduction(&site.id));
    session.maintenance_adjusted_danger(
        session.crew_adjusted_danger(
            voyage_plan.adjust_danger(route_danger, &data.config.voyage_plan),
        ),
        data,
    )
}

fn departure_message(
    site: &SiteData,
    costs: DepartureCosts,
    salvage_count: usize,
    reconnaissance_level: u8,
    voyage_plan: VoyagePlan,
    session: &GameSession,
    contract_accepted: bool,
) -> String {
    let coverage_message = if costs.insurance_premium > 0 {
        format!(" Coverage secured for {} credits.", costs.insurance_premium)
    } else {
        String::new()
    };
    let intelligence_message = if reconnaissance_level == 0 {
        String::new()
    } else {
        format!(" Route intel level {reconnaissance_level} reduced departure danger.")
    };
    let contract_message = if contract_accepted {
        String::new()
    } else {
        " Private haul; client contract declined.".to_owned()
    };
    format!(
        "Travelled to {} for {} fuel. Manifest: {} target(s) remain.{}{} Operating plan: {}. Crew: {} // {}.{}",
        site.display_name,
        costs.fuel,
        salvage_count,
        coverage_message,
        intelligence_message,
        voyage_plan.label(),
        session.crew_role().label(),
        session.crew_role().description(),
        contract_message,
    )
}

impl GameSession {
    pub fn begin_expedition_with_plan_and_contract(
        &mut self,
        site_id: &str,
        data: &GameData,
        insured: bool,
        plan: VoyagePlan,
        contract_accepted: bool,
    ) -> Result<String, String> {
        begin_expedition(self, site_id, data, insured, plan, contract_accepted)
    }
}

impl GameSession {
    pub fn begin_expedition(&mut self, site_id: &str, data: &GameData) -> Result<String, String> {
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
        begin_expedition(self, site_id, data, insured, plan, true)
    }
}
