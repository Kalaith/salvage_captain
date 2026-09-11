//! Mission briefing: three wrecks presented as physical salvage jobs.

use super::*;
use crate::state::VoyageRecord;
use crate::ui::visual_theme;

#[cfg(test)]
mod tests;

pub fn draw_site_selection(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let frame = Rect::new(0.0, 84.0, 1280.0, 636.0);
    panel(frame, visual_theme::panel_soft());
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        42.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "MISSION BRIEFING  //  AVAILABLE WRECKS",
        frame.x + 18.0,
        frame.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "READ THE HULL. PRICE THE RETURN.",
        frame.right() - 262.0,
        frame.y + 27.0,
        11.0,
        visual_theme::amber(),
    );
    draw_text(
        "Fuel cost includes navigation discount; required fuel includes the safe-return buffer.",
        frame.x + 22.0,
        frame.y + 66.0,
        14.0,
        visual_theme::text_dim(),
    );
    draw_text(
        blueprint_progress_label(ctx.session, ctx.data),
        frame.x + 260.0,
        frame.y + 94.0,
        11.0,
        visual_theme::amber(),
    );
    if button(
        ctx,
        Rect::new(frame.x + 46.0, frame.y + 74.0, 196.0, 30.0),
        &format!("PLAN  //  {}", ctx.voyage_plan.label()),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleVoyagePlan);
    }
    draw_text(
        clipped(ctx.voyage_plan.description(), 72),
        frame.x + 260.0,
        frame.y + 76.0,
        12.0,
        visual_theme::cyan(),
    );
    crew_panel::draw_briefing_control(ctx, actions);

    for (index, site) in ctx.data.ordered_sites().into_iter().enumerate() {
        let x = 42.0 + index as f32 * 398.0;
        draw_site_card(ctx, actions, site, Rect::new(x, 194.0, 378.0, 392.0));
    }
    draw_text(
        "A site choice is a risk choice: danger is previewed, but the exact setback is seeded at departure.",
        46.0,
        608.0,
        14.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &format!(
            "PRIVATE HAUL keeps the cargo and declines the contract // OPTIONAL COVER pays {}% of an eligible setback // ROUTE INTEL persists per wreck.",
            ctx.data.config.insurance.coverage_percent,
        ),
        46.0,
        630.0,
        12.0,
        visual_theme::cyan(),
    );
}

fn blueprint_progress_label(session: &GameSession, data: &GameData) -> String {
    let unlocked = session.unlocked_module_count(data);
    let total = data.modules.iter().count();
    let next = session.next_module_unlock(data).map_or_else(
        || "ALL SYSTEMS CERTIFIED".to_owned(),
        |module| {
            format!(
                "NEXT {} @ ¢{}",
                module.display_name.to_uppercase(),
                module.unlock_credits
            )
        },
    );
    let standing = session.salvage_standing();
    let standing_progress = session.next_standing_threshold().map_or_else(
        || format!("STAND {} // REP {}", standing.label(), session.reputation),
        |threshold| {
            format!(
                "STAND {} // REP {}/{}",
                standing.label(),
                session.reputation,
                threshold
            )
        },
    );
    format!("SHIP BLUEPRINTS {unlocked}/{total}  //  {next}  //  {standing_progress}")
}

fn draw_site_card(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
    rect: Rect,
) {
    let accent = visual_theme::site_accent(&site.visual_theme);
    let progress = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(site.condition, |value| value.condition);
    let visits = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(0, |value| value.visits);
    panel(rect, visual_theme::panel());
    draw_rectangle(rect.x, rect.y, 6.0, rect.h, accent);
    draw_wreck_brief(
        rect.x + 18.0,
        rect.y + 16.0,
        rect.w - 36.0,
        92.0,
        &site.visual_theme,
        progress,
    );
    draw_text(
        &site.display_name.to_uppercase(),
        rect.x + 18.0,
        rect.y + 136.0,
        22.0,
        visual_theme::text(),
    );
    draw_text(
        &site.wreck_class.to_uppercase(),
        rect.x + 18.0,
        rect.y + 158.0,
        11.0,
        accent,
    );
    draw_text(
        clipped(&site.description, 47),
        rect.x + 18.0,
        rect.y + 184.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        site_danger_label(site, ctx.session, ctx.data, ctx.voyage_plan),
        rect.x + 18.0,
        rect.y + 218.0,
        17.0,
        danger_color(site_departure_danger(
            site,
            ctx.session,
            ctx.data,
            ctx.voyage_plan,
        )),
    );
    let cost = ctx
        .session
        .effective_fuel_cost_with_plan(&site.id, ctx.data, ctx.voyage_plan)
        .unwrap_or(site.fuel_cost);
    let required = ctx
        .session
        .departure_fuel_required_with_plan(&site.id, ctx.data, ctx.voyage_plan)
        .unwrap_or(cost);
    draw_text(
        format!("FUEL  {} TRIP  /  {} REQUIRED", cost, required),
        rect.x + 18.0,
        rect.y + 244.0,
        13.0,
        visual_theme::text(),
    );
    let recovery = ctx.session.site_recovery_status(&site.id, ctx.data);
    let clearance_label = site_clearance_label(ctx.session, &site.id, ctx.data);
    draw_text(
        format!(
            "COND {}%  //  VISITS {}  //  EXPLORED {}%  //  RECOV {}/{}  //  {}",
            progress,
            visits,
            recovery.exploration_percent,
            recovery.recovered_targets,
            recovery.total_targets,
            clearance_label
        ),
        rect.x + 18.0,
        rect.y + 266.0,
        11.0,
        visual_theme::text_dim(),
    );
    let contract_target = site
        .contract_target
        .as_deref()
        .and_then(|target_id| ctx.data.salvage_objects.get(target_id))
        .map_or_else(
            || site.contract_target.as_deref().unwrap_or("NONE").to_owned(),
            |target| target.display_name.clone(),
        );
    let contract_complete = ctx
        .session
        .site_progress
        .get(&site.id)
        .is_some_and(|value| value.contract_completed);
    let contract_failed = ctx
        .session
        .site_progress
        .get(&site.id)
        .is_some_and(|value| value.contract_failed);
    let market_outlook = site_market_outlook_label(site, ctx.session, ctx.data);
    draw_text(
        clipped(
            &format!(
                "KNOWN  {}  //  {}",
                clipped(&site.known_reward, 20),
                market_outlook
            ),
            54,
        ),
        rect.x + 18.0,
        rect.y + 280.0,
        12.0,
        visual_theme::text(),
    );
    draw_text(
        &ctx.session.route_familiarity_readout(&site.id),
        rect.x + 18.0,
        rect.y + 298.0,
        10.0,
        if ctx.session.route_familiarity(&site.id) == 0 {
            visual_theme::text_dim()
        } else {
            visual_theme::cyan()
        },
    );
    draw_text(
        clipped(
            &format!(
                "CONTRACT  {}  //  {}  //  +{} CR",
                contract_status_label(contract_complete, contract_failed),
                contract_target.to_uppercase(),
                site.contract_reward,
            ),
            56,
        ),
        rect.x + 18.0,
        rect.y + 316.0,
        10.0,
        if contract_complete {
            visual_theme::safe()
        } else if contract_failed {
            visual_theme::warning()
        } else {
            accent
        },
    );
    draw_text(
        clipped(
            &format!(
                "MOMENTUM  //  {}",
                contract_streak_label(ctx.session, contract_failed)
            ),
            56,
        ),
        rect.x + 18.0,
        rect.y + 332.0,
        10.0,
        if contract_failed {
            visual_theme::warning()
        } else {
            visual_theme::amber()
        },
    );
    let section_count = site.sections.len();
    let gated_sections = site
        .sections
        .iter()
        .filter(|section| section.required_capability.is_some())
        .count();
    let stats = ctx.session.module_stats(ctx.data);
    let offline = ctx.session.damaged_modules.len();
    draw_text(
        format!(
            "SEC {}  //  GATE {}  //  OFF {}  //  SCAN+{}  HULL+{}  DRONE+{}  //  {}",
            section_count,
            gated_sections,
            offline,
            stats.scanning,
            stats.hull,
            stats.drone_support,
            site_reconnaissance_label(ctx.session, &site.id, ctx.data),
        ),
        rect.x + 18.0,
        rect.y + 348.0,
        11.0,
        visual_theme::text_dim(),
    );
    let last_run = ctx
        .session
        .voyage_log
        .iter()
        .rev()
        .find(|record| record.site_id == site.id);
    let log_count = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(0, |value| value.operation_log.len());
    let survey_count = ctx.session.site_survey_count(&site.id);
    let unlocked_blueprints = ctx.session.unlocked_module_count(ctx.data);
    let total_blueprints = ctx.data.modules.iter().count();
    let standing_progress = site_standing_progress_label(ctx.session);
    draw_text(
        clipped(&site_last_run_label(last_run), 56),
        rect.x + 18.0,
        rect.y + 366.0,
        10.0,
        last_run.map_or(visual_theme::text_dim(), |record| {
            if record.risk_outcome == RiskOutcome::OrdinaryReturn {
                visual_theme::safe()
            } else {
                visual_theme::warning()
            }
        }),
    );
    draw_text(
        clipped(
            &site_last_run_memory_label(
                last_run,
                log_count,
                survey_count,
                unlocked_blueprints,
                total_blueprints,
                &standing_progress,
            ),
            64,
        ),
        rect.x + 18.0,
        rect.y + 384.0,
        10.0,
        visual_theme::text_dim(),
    );
    let can_depart = ctx
        .session
        .can_depart_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let insurance_quote =
        ctx.session
            .insurance_quote_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let can_depart_insured =
        ctx.session
            .can_depart_insured_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let reconnaissance_quote = ctx.session.reconnaissance_quote(&site.id, ctx.data);
    let can_buy_reconnaissance = ctx.session.can_buy_reconnaissance(&site.id, ctx.data);
    let button_gap = 8.0;
    let button_width = (rect.w - 36.0 - button_gap) / 2.0;
    if button(
        ctx,
        Rect::new(rect.x + 18.0, rect.bottom() - 78.0, button_width, 34.0),
        if can_depart {
            "DEPART"
        } else {
            "NOT ENOUGH FUEL"
        },
        can_depart,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Depart(site.id.clone()));
    }
    if button(
        ctx,
        Rect::new(
            rect.x + 18.0 + button_width + button_gap,
            rect.bottom() - 78.0,
            button_width,
            34.0,
        ),
        "PRIVATE HAUL",
        can_depart,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::DepartPrivate(site.id.clone()));
    }
    if button(
        ctx,
        Rect::new(rect.x + 18.0, rect.bottom() - 42.0, button_width, 34.0),
        &insurance_button_label(insurance_quote, can_depart, can_depart_insured),
        can_depart_insured,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::DepartInsured(site.id.clone()));
    }
    if button(
        ctx,
        Rect::new(
            rect.x + 18.0 + button_width + button_gap,
            rect.bottom() - 42.0,
            button_width,
            34.0,
        ),
        &reconnaissance_button_label(reconnaissance_quote, can_depart, can_buy_reconnaissance),
        can_buy_reconnaissance,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BuyReconnaissance(site.id.clone()));
    }
}

fn contract_streak_label(session: &GameSession, contract_failed: bool) -> String {
    if contract_failed {
        "STREAK RESET  //  REBUILD".to_owned()
    } else if session.contract_streak() == 0 {
        "STREAK READY".to_owned()
    } else {
        format!(
            "STREAK x{}  //  NEXT +¢{}",
            session.contract_streak(),
            session.next_contract_streak_bonus()
        )
    }
}

fn site_clearance_label(session: &GameSession, site_id: &str, data: &GameData) -> String {
    let (cleared, total) = session.site_clearance_summary(site_id, data);
    let (paid_reward, remaining_reward) = session.site_clearance_rewards(site_id, data);
    if remaining_reward == 0 {
        format!("CLR {cleared}/{total} // PAID +¢{paid_reward}")
    } else if paid_reward > 0 {
        format!("CLR {cleared}/{total} // PAID +¢{paid_reward} // LEFT +¢{remaining_reward}")
    } else {
        format!("CLR {cleared}/{total} // +¢{remaining_reward} LEFT")
    }
}

fn site_last_run_label(last_run: Option<&VoyageRecord>) -> String {
    last_run.map_or_else(
        || "LAST RUN  NONE".to_owned(),
        |record| {
            format!(
                "LAST {}  //  TGT {}  //  HOME {} FUEL",
                risk_label(record.risk_outcome),
                record.recovered_count,
                record.return_fuel,
            )
        },
    )
}

fn site_last_run_memory_label(
    last_run: Option<&VoyageRecord>,
    log_count: usize,
    survey_count: usize,
    unlocked_blueprints: usize,
    total_blueprints: usize,
    standing_progress: &str,
) -> String {
    let progress = format!(
        "LOG {:02}  //  SURV {:02}  //  BP {:02}/{:02}  //  {}",
        log_count, survey_count, unlocked_blueprints, total_blueprints, standing_progress
    );
    last_run.map_or(progress.clone(), |record| {
        format!("VALUE ¢{}  //  {progress}", record.recovered_value)
    })
}

fn insurance_button_label(
    quote: Option<crate::engine::InsuranceQuote>,
    can_depart: bool,
    can_depart_insured: bool,
) -> String {
    let Some(quote) = quote else {
        return "NO COVER".to_owned();
    };
    if !can_depart {
        return "NO FUEL".to_owned();
    }
    if !can_depart_insured {
        return format!("LOW CR ¢{}", quote.premium);
    }
    format!("COVER ¢{}", quote.premium)
}

fn reconnaissance_button_label(
    quote: Option<crate::engine::ReconnaissanceQuote>,
    can_depart: bool,
    can_buy: bool,
) -> String {
    if !can_depart {
        return "NO FUEL".to_owned();
    }
    let Some(quote) = quote else {
        return "INTEL MAX".to_owned();
    };
    if !can_buy {
        return format!("LOW CR ¢{}", quote.cost);
    }
    format!("INTEL ¢{}", quote.cost)
}

fn site_reconnaissance_label(session: &GameSession, site_id: &str, data: &GameData) -> String {
    let level = session.reconnaissance_level(site_id);
    let max = data.config.reconnaissance.max_level;
    format!("INTEL {level}/{max}")
}

fn site_standing_progress_label(session: &GameSession) -> String {
    session.next_standing_threshold().map_or_else(
        || format!("REP {}", session.reputation),
        |threshold| format!("REP {}/{}", session.reputation, threshold),
    )
}

fn site_market_outlook_label(
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
        return "DEMAND UNKNOWN".to_owned();
    };
    format!(
        "MKT {} {:+}%",
        quote.band.label(),
        quote.signed_multiplier()
    )
}

fn site_departure_danger(
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

fn site_danger_label(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
    voyage_plan: crate::engine::VoyagePlan,
) -> String {
    let level = session.reconnaissance_level(&site.id);
    let familiarity = session.route_familiarity(&site.id);
    let danger = site_departure_danger(site, session, data, voyage_plan);
    let plan_delta = voyage_plan.danger_delta(&data.config.voyage_plan);
    let crew_delta = session.crew_danger_delta();
    let maintenance_delta = session.maintenance_danger_delta(data);
    if level == 0
        && familiarity == 0
        && plan_delta == 0
        && crew_delta == 0
        && maintenance_delta == 0
    {
        format!("DANGER  {:02}%", site.danger)
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
            "DANGER  {:02}% -> {:02}%  //  {}",
            site.danger,
            danger,
            adjustments.join("  //  ")
        )
    }
}

fn draw_wreck_brief(x: f32, y: f32, width: f32, height: f32, theme: &str, condition: i32) {
    let accent = visual_theme::site_accent(theme);
    draw_rectangle(x, y, width, height, visual_theme::space());
    draw_rectangle_lines(x, y, width, height, 2.0, accent);
    draw_line(
        x + 26.0,
        y + height * 0.28,
        x + width - 32.0,
        y + height * 0.48,
        5.0,
        visual_theme::structure_light(),
    );
    draw_line(
        x + 36.0,
        y + height * 0.67,
        x + width - 56.0,
        y + height * 0.58,
        3.0,
        visual_theme::structure(),
    );
    draw_line(
        x + width * 0.35,
        y + 12.0,
        x + width * 0.44,
        y + height - 12.0,
        2.0,
        accent,
    );
    draw_line(
        x + width * 0.7,
        y + 16.0,
        x + width * 0.62,
        y + height - 16.0,
        2.0,
        accent,
    );
    for index in 0..5 {
        let light_x = x + 28.0 + index as f32 * (width - 70.0) / 4.0;
        draw_circle(light_x, y + height * 0.31, 3.0, accent);
    }
    draw_profile_hint(x, y, width, height, theme);
    draw_rectangle(
        x + 18.0,
        y + height - 25.0,
        102.0,
        14.0,
        visual_theme::with_alpha(accent, 0.18),
    );
    draw_text(
        format!("COND {}%", condition),
        x + 25.0,
        y + height - 14.0,
        10.0,
        accent,
    );
    draw_text(
        "VISUAL SCAN",
        x + width - 102.0,
        y + height - 14.0,
        10.0,
        visual_theme::text_dim(),
    );
}

fn draw_profile_hint(x: f32, y: f32, width: f32, height: f32, theme: &str) {
    match theme {
        "military" => {
            for index in 0..2 {
                let start_x = x + width * (0.16 + index as f32 * 0.42);
                draw_line(
                    start_x,
                    y + height * 0.52,
                    start_x + width * 0.18,
                    y + height * 0.84,
                    5.0,
                    visual_theme::with_alpha(visual_theme::warning(), 0.72),
                );
            }
        }
        "research" => {
            draw_rectangle_lines(
                x + width * 0.12,
                y + height * 0.55,
                width * 0.28,
                height * 0.2,
                2.0,
                visual_theme::with_alpha(visual_theme::cyan(), 0.72),
            );
            draw_circle_lines(
                x + width * 0.74,
                y + height * 0.6,
                height * 0.16,
                2.0,
                visual_theme::with_alpha(visual_theme::cyan(), 0.72),
            );
        }
        _ => {
            for index in 0..3 {
                let crate_x = x + width * (0.14 + index as f32 * 0.25);
                draw_rectangle(
                    crate_x,
                    y + height * 0.6,
                    width * 0.16,
                    height * 0.16,
                    visual_theme::with_alpha(visual_theme::amber(), 0.5),
                );
            }
        }
    }
}
