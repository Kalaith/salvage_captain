//! Automatic transit scene with a skippable arrival briefing.

use super::scene_layout;
use super::ship_visual;
use super::visual_theme;
use super::*;
mod visual;

pub(crate) const TRAVEL_DURATION_SECONDS: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TravelPhase {
    Departure,
    Cruise,
    FinalApproach,
    Docked,
}

pub fn draw_travel(ctx: &UiContext<'_>, _actions: &mut Vec<UiAction>) {
    let view = scene_layout::travel_view();
    panel(view, visual_theme::panel_soft());
    let Some(expedition) = &ctx.session.expedition else {
        draw_text(
            "NO ACTIVE TRANSIT",
            54.0,
            178.0,
            24.0,
            visual_theme::warning(),
        );
        return;
    };
    let Some(site) = ctx.data.sites.get(&expedition.site_id) else {
        return;
    };
    let progress = (ctx.travel_elapsed / TRAVEL_DURATION_SECONDS).clamp(0.0, 1.0);
    let from_fuel = ctx.session.economy.fuel
        + ctx
            .session
            .effective_fuel_cost_with_plan(&site.id, ctx.data, expedition.voyage_plan)
            .unwrap_or(site.fuel_cost);
    let danger_label = if expedition.voyage_plan == crate::engine::VoyagePlan::Standard {
        travel_danger_label(site, ctx.session, ctx.data)
    } else {
        travel_danger_label_with_plan(site, ctx.session, ctx.data, expedition.voyage_plan)
    };
    draw_travel_overview(ctx, site, expedition);
    draw_travel_memory(ctx, site, expedition);
    let phase = travel_phase(progress);
    draw_travel_scene(ctx, site, progress, phase);
    draw_arrival_brief(ctx, site, progress, phase, from_fuel, &danger_label);
    draw_text(
        travel_instruction(phase),
        54.0,
        650.0,
        14.0,
        visual_theme::text_dim(),
    );
}

fn draw_travel_overview(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    expedition: &crate::state::ExpeditionState,
) {
    draw_text("AUTOMATIC TRANSIT", 54.0, 176.0, 16.0, visual_theme::cyan());
    draw_text(
        site.display_name.to_uppercase(),
        54.0,
        214.0,
        30.0,
        visual_theme::text(),
    );
    draw_text(
        &site.wreck_class,
        56.0,
        239.0,
        15.0,
        visual_theme::text_dim(),
    );
    draw_text(
        clipped(&site.arrival_text, 66),
        54.0,
        276.0,
        15.0,
        visual_theme::text_dim(),
    );
    draw_travel_contract(ctx, site);
    draw_text(
        ctx.session.route_familiarity_readout(&site.id),
        54.0,
        352.0,
        12.0,
        if ctx.session.route_familiarity(&site.id) == 0 {
            visual_theme::text_dim()
        } else {
            visual_theme::cyan()
        },
    );
    draw_text(
        travel_market_label(site, ctx.session, ctx.data),
        54.0,
        374.0,
        12.0,
        visual_theme::cyan(),
    );
    draw_text(
        travel_coverage_label(
            expedition.insured,
            ctx.session
                .insurance_quote_with_plan(&site.id, ctx.data, expedition.voyage_plan),
            ctx.data.config.insurance.coverage_percent,
        ),
        54.0,
        396.0,
        12.0,
        if expedition.insured {
            visual_theme::safe()
        } else {
            visual_theme::text_dim()
        },
    );
    draw_text(
        format!(
            "OPERATING PLAN  {}  //  {}  //  FRAME {} SECTIONS  //  {} HAZARD SIGNALS",
            expedition.voyage_plan.label(),
            travel_crew_label(ctx.session),
            site.sections.len(),
            site_hazard_count(site)
        ),
        54.0,
        418.0,
        12.0,
        visual_theme::site_accent(&site.visual_theme),
    );
}

fn draw_travel_contract(ctx: &UiContext<'_>, site: &crate::data::SiteData) {
    if let Some(contract_target) = &site.contract_target {
        let target_name = ctx
            .data
            .salvage_objects
            .get(contract_target)
            .map_or(contract_target.as_str(), |target| {
                target.display_name.as_str()
            });
        let objective = ctx.session.contract_objective_status(&site.id, ctx.data);
        draw_text(
            format!(
                "CONTRACT  //  {} {}  //  +{} CR",
                objective
                    .as_ref()
                    .map_or("OPEN", |status| status.state.label()),
                target_name.to_uppercase(),
                site.contract_reward
            ),
            54.0,
            308.0,
            12.0,
            match objective.as_ref().map(|status| status.state) {
                Some(crate::state::contracts::ContractObjectiveState::Complete) => {
                    visual_theme::safe()
                }
                Some(crate::state::contracts::ContractObjectiveState::Failed) => {
                    visual_theme::warning()
                }
                _ => visual_theme::site_accent(&site.visual_theme),
            },
        );
        draw_text(
            clipped(&site.contract_brief, 66),
            54.0,
            330.0,
            13.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_travel_memory(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    expedition: &crate::state::ExpeditionState,
) {
    let frame_condition = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(site.condition, |progress| progress.condition);
    let recovery = ctx.session.site_recovery_status(&site.id, ctx.data);
    let survey_count = ctx.session.site_survey_count(&site.id);
    let blueprint_progress = travel_blueprint_label(ctx.session, ctx.data);
    let standing_progress = travel_standing_label(ctx.session);
    draw_text(
        format!(
            "FRAME CONDITION {:02}%  //  RECOVERY {}/{}  //  EXPLORED {:02}%  //  {}  //  {}  //  {}  //  {}",
            frame_condition,
            recovery.recovered_targets,
            recovery.total_targets,
            recovery.exploration_percent,
            travel_survey_label(survey_count),
            travel_scan_label(expedition.scan_profile),
            blueprint_progress,
            standing_progress
        ),
        54.0,
        440.0,
        12.0,
        if recovery.recovered_targets > 0 {
            visual_theme::amber()
        } else {
            visual_theme::text_dim()
        },
    );
}

fn draw_travel_scene(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    progress: f32,
    phase: TravelPhase,
) {
    visual::draw_transit_route(
        progress,
        ctx.travel_elapsed,
        site.visual_theme.as_str(),
        phase,
    );
    visual::draw_wreck_marker(
        862.0,
        330.0,
        site.visual_theme.as_str(),
        progress,
        ctx.travel_elapsed,
        phase,
    );
    let ship_rect = scene_layout::travel_ship_rect(progress);
    visual::draw_transit_wake(
        ship_rect,
        ctx.travel_elapsed,
        site.visual_theme.as_str(),
        phase,
    );
    ship_visual::draw_ship(ship_rect, ctx.session, ctx.data, ctx.travel_elapsed, false);
}

fn draw_arrival_brief(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    progress: f32,
    phase: TravelPhase,
    from_fuel: i32,
    danger_label: &str,
) {
    let brief = Rect::new(450.0, 500.0, 380.0, 142.0);
    panel(brief, visual_theme::with_alpha(visual_theme::panel(), 0.94));
    draw_text(
        "ARRIVAL BRIEF",
        brief.x + 20.0,
        brief.y + 28.0,
        17.0,
        visual_theme::text(),
    );
    draw_text(
        format!("DESTINATION  {}", site.display_name),
        brief.x + 20.0,
        brief.y + 54.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!("FUEL BEFORE  {}", from_fuel),
        brief.x + 20.0,
        brief.y + 76.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "FUEL AFTER {}  //  {}",
            ctx.session.economy.fuel, danger_label
        ),
        brief.x + 160.0,
        brief.y + 76.0,
        14.0,
        danger_color(travel_departure_danger(site, ctx.session, ctx.data)),
    );
    draw_text(
        format!("CLASS        {}", site.wreck_class),
        brief.x + 20.0,
        brief.y + 98.0,
        13.0,
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(brief.x + 160.0, brief.y + 88.0, 200.0, 24.0),
        progress,
        travel_phase_color(phase),
        &format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
    );
    draw_text(
        format!(
            "STATUS       {}  //  ETA {}",
            travel_phase_label(phase),
            travel_eta_label(progress)
        ),
        brief.x + 20.0,
        brief.y + 120.0,
        13.0,
        travel_phase_color(phase),
    );
}

fn travel_phase(progress: f32) -> TravelPhase {
    match progress.clamp(0.0, 1.0) {
        value if value < 0.18 => TravelPhase::Departure,
        value if value < 0.72 => TravelPhase::Cruise,
        value if value < 1.0 => TravelPhase::FinalApproach,
        _ => TravelPhase::Docked,
    }
}

fn travel_phase_label(phase: TravelPhase) -> &'static str {
    match phase {
        TravelPhase::Departure => "DEPARTURE",
        TravelPhase::Cruise => "CRUISE",
        TravelPhase::FinalApproach => "FINAL APPROACH",
        TravelPhase::Docked => "DOCKED",
    }
}

fn travel_eta_label(progress: f32) -> String {
    let seconds = ((1.0 - progress.clamp(0.0, 1.0)) * 4.0).ceil() as i32;
    if seconds == 0 {
        "NOW".to_owned()
    } else {
        format!("{seconds}s")
    }
}

fn wake_segment_count(phase: TravelPhase) -> usize {
    match phase {
        TravelPhase::Departure => 5,
        TravelPhase::Cruise => 4,
        TravelPhase::FinalApproach => 2,
        TravelPhase::Docked => 0,
    }
}

fn site_hazard_count(site: &crate::data::SiteData) -> usize {
    site.sections
        .iter()
        .map(|section| section.hazard_tags.len())
        .sum()
}

fn travel_survey_label(survey_count: usize) -> String {
    format!("SURV {:02}", survey_count)
}

fn travel_scan_label(profile: crate::state::WorkspaceScanProfile) -> String {
    format!("SCAN {}", profile.short_label())
}

fn travel_blueprint_label(session: &GameSession, data: &GameData) -> String {
    let unlocked = session.unlocked_module_count(data);
    let total = data.modules.iter().count();
    let next = session.next_module_unlock(data).map_or_else(
        || "ALL ONLINE".to_owned(),
        |module| {
            format!(
                "NEXT {} @ ¢{}",
                module.display_name.to_uppercase(),
                module.unlock_credits
            )
        },
    );
    format!("BP {:02}/{:02}  //  {next}", unlocked, total)
}

fn travel_standing_label(session: &GameSession) -> String {
    let standing = session.salvage_standing();
    session.next_standing_threshold().map_or_else(
        || format!("STAND {} // REP {}", standing.label(), session.reputation),
        |threshold| {
            format!(
                "STAND {} // REP {}/{}",
                standing.label(),
                session.reputation,
                threshold
            )
        },
    )
}

fn travel_crew_label(session: &GameSession) -> String {
    format!(
        "CREW {}  //  READY {}%",
        session.crew_role().short_label(),
        session.crew_readiness()
    )
}

fn travel_market_label(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
) -> String {
    let Some(quote) = site
        .candidate_salvage
        .iter()
        .filter_map(|object_id| session.market_quote(object_id, data))
        .max_by_key(|quote| (quote.signed_multiplier(), quote.sale_value))
    else {
        return "MARKET UNKNOWN".to_owned();
    };
    format!(
        "MARKET {} {} {:+}%  //  QUOTES LOCK AT RETURN",
        session.market_cycle_label(),
        quote.band.label(),
        quote.signed_multiplier()
    )
}

fn travel_coverage_label(
    insured: bool,
    quote: Option<crate::engine::InsuranceQuote>,
    coverage_percent: i32,
) -> String {
    if !insured {
        return "COVER NONE  //  SELF-INSURED RETURN".to_owned();
    }
    quote.map_or_else(
        || "COVER ACTIVE  //  CLAIM TERMS UNAVAILABLE".to_owned(),
        |quote| {
            format!(
                "COVER ACTIVE  //  PREMIUM ¢{} PAID  //  CLAIM {}%",
                quote.premium, coverage_percent
            )
        },
    )
}

fn travel_departure_danger(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
) -> i32 {
    travel_departure_danger_with_plan(site, session, data, crate::engine::VoyagePlan::Standard)
}

fn travel_departure_danger_with_plan(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
    voyage_plan: crate::engine::VoyagePlan,
) -> i32 {
    let route_danger = crate::engine::danger_after_intel(
        site.danger,
        session.reconnaissance_level(&site.id),
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

fn travel_danger_label(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
) -> String {
    travel_danger_label_with_plan(site, session, data, crate::engine::VoyagePlan::Standard)
}

fn travel_danger_label_with_plan(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
    voyage_plan: crate::engine::VoyagePlan,
) -> String {
    let route_danger = travel_departure_danger_with_plan(site, session, data, voyage_plan);
    let level = session.reconnaissance_level(&site.id);
    let familiarity = session.route_familiarity(&site.id);
    let plan_delta = voyage_plan.danger_delta(&data.config.voyage_plan);
    let crew_delta = session.crew_danger_delta();
    let maintenance_delta = session.maintenance_danger_delta(data);
    if level == 0
        && familiarity == 0
        && plan_delta == 0
        && crew_delta == 0
        && maintenance_delta == 0
    {
        format!("DANGER {:02}%", site.danger)
    } else {
        let mut adjustments = Vec::new();
        if level > 0 {
            adjustments.push(format!(
                "INTEL -{}",
                level as i32 * data.config.reconnaissance.danger_reduction_per_level
            ));
        }
        if familiarity > 0 {
            adjustments.push(format!(
                "ROUTE -{}",
                session.route_familiarity_danger_reduction(&site.id)
            ));
        }
        if plan_delta != 0 {
            adjustments.push(format!("PLAN {plan_delta:+}"));
        }
        if crew_delta != 0 {
            adjustments.push(format!("CREW {crew_delta:+}"));
        }
        if maintenance_delta != 0 {
            adjustments.push(format!("WEAR {maintenance_delta:+}"));
        }
        format!(
            "DANGER {:02}% -> {:02}%  //  {}",
            site.danger,
            route_danger,
            adjustments.join("  //  ")
        )
    }
}

fn travel_phase_color(phase: TravelPhase) -> Color {
    match phase {
        TravelPhase::Departure | TravelPhase::Cruise => visual_theme::cyan(),
        TravelPhase::FinalApproach => visual_theme::amber(),
        TravelPhase::Docked => visual_theme::safe(),
    }
}

fn travel_instruction(phase: TravelPhase) -> &'static str {
    match phase {
        TravelPhase::Departure => "Transit cleared. Tap ARRIVE whenever you are ready.",
        TravelPhase::Cruise => "Route stable. Tap ARRIVE to skip ahead to the wreck.",
        TravelPhase::FinalApproach => {
            "Final approach locked. Tap ARRIVE to enter the wreck workspace."
        }
        TravelPhase::Docked => "Tap CONTINUE in the top HUD to enter the wreck workspace.",
    }
}
