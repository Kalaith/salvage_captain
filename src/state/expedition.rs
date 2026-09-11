//! Departure setup for fuel, seeded risk, salvage manifests, and coverage.

use super::{
    CargoItem, CargoStatus, ExpeditionState, GameSession, ReturnPolicy, WorkspaceLogEvent,
};
use crate::data::GameData;
use crate::engine::{danger_after_intel, generate_salvage, resolve_risk, VoyagePlan};
use crate::state::{DroneDirective, WorkspaceScanProfile};

pub(super) fn begin_expedition(
    session: &mut GameSession,
    site_id: &str,
    data: &GameData,
    insured: bool,
    voyage_plan: VoyagePlan,
) -> Result<String, String> {
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
    let fuel_cost = session
        .effective_fuel_cost_with_plan(site_id, data, voyage_plan)
        .unwrap_or(site.fuel_cost);
    session.economy.fuel -= fuel_cost;
    session.economy.credits -= insurance_premium;
    let seed = session.seed;
    session.seed = session.seed.wrapping_add(1);
    let stats = session.module_stats(data);
    let scan_profile =
        WorkspaceScanProfile::from_capability(session.has_capability("scanner_array", data));
    let workspace_energy_capacity = stats.power.max(1) * 6;
    let condition = session
        .site_progress
        .get(site_id)
        .map_or(site.condition, |progress| progress.condition);
    let removed_targets = session
        .site_progress
        .get(site_id)
        .map_or_else(Vec::new, |progress| progress.removed_targets.clone());
    let condition_penalty = (100 - condition).max(0) / 4;
    let reconnaissance_level = session.reconnaissance_level(site_id);
    let route_danger = danger_after_intel(
        site.danger + condition_penalty,
        reconnaissance_level,
        &data.config.reconnaissance,
    )
    .saturating_sub(session.route_familiarity_danger_reduction(site_id));
    let departure_danger = session.maintenance_adjusted_danger(
        session.crew_adjusted_danger(
            voyage_plan.adjust_danger(route_danger, &data.config.voyage_plan),
        ),
        data,
    );
    let risk = resolve_risk(
        seed,
        departure_danger,
        session.hull,
        stats,
        &data.config.risk,
    );
    let salvage_manifest = generate_salvage(site, seed, &removed_targets);
    let salvage_count = salvage_manifest.len();
    session.expedition = Some(ExpeditionState {
        site_id: site_id.to_owned(),
        cargo: salvage_manifest
            .into_iter()
            .map(|object_id| CargoItem {
                object_id,
                status: CargoStatus::Pending,
                position: None,
                rotation: 0,
            })
            .collect(),
        risk,
        seed,
        workspace_section: site
            .sections
            .first()
            .map_or_else(String::new, |section| section.id.clone()),
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
        voyage_plan,
        return_policy: ReturnPolicy::default(),
    });
    let first_section = session
        .expedition
        .as_ref()
        .map(|expedition| expedition.workspace_section.clone());
    session.append_workspace_log(
        site_id,
        WorkspaceLogEvent::Departed,
        first_section.as_deref(),
        None,
    );
    session.selected_site = Some(site_id.to_owned());
    let coverage_message = if insured {
        format!(" Coverage secured for {insurance_premium} credits.")
    } else {
        String::new()
    };
    let intelligence_message = if reconnaissance_level == 0 {
        String::new()
    } else {
        format!(" Route intel level {reconnaissance_level} reduced departure danger.")
    };
    let crew_message = format!(
        " Crew: {} // {}.",
        session.crew_role().label(),
        session.crew_role().description()
    );
    let plan_message = format!(" Operating plan: {}.", voyage_plan.label());
    Ok(format!(
        "Travelled to {} for {fuel_cost} fuel. Manifest: {salvage_count} target(s) remain.{coverage_message}{intelligence_message}{plan_message}{crew_message}",
        site.display_name,
    ))
}

#[cfg(test)]
mod tests;
