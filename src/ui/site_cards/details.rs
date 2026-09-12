//! On-demand wreck records and planning details.

use super::*;

pub(super) fn site_departure_danger(
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

pub(super) fn draw(ctx: &UiContext<'_>, site: &crate::data::SiteData) {
    let copy = &ctx.data.selection_ui;
    text(
        &copy.risk_detail,
        44.0,
        434.0,
        600.0,
        36.0,
        28.0,
        visual_theme::text(),
    );
    let danger = site_departure_danger(site, ctx.session, ctx.data, ctx.voyage_plan);
    let description = copy
        .risk_hint
        .replace("{base}", &site.danger.to_string())
        .replace("{danger}", &danger.to_string());
    text(
        &description,
        44.0,
        486.0,
        400.0,
        100.0,
        22.0,
        visual_theme::text_dim(),
    );
    text(
        &format!(
            "{} {}% · {}",
            copy.readiness,
            ctx.session.crew_readiness(),
            ctx.session.crew_expertise_label()
        ),
        44.0,
        600.0,
        410.0,
        40.0,
        22.0,
        visual_theme::text(),
    );
    text(
        &ctx.session.route_familiarity_readout(&site.id),
        44.0,
        650.0,
        410.0,
        30.0,
        20.0,
        visual_theme::text_dim(),
    );
    draw_record(ctx, site);
}

fn draw_record(ctx: &UiContext<'_>, site: &crate::data::SiteData) {
    let copy = &ctx.data.selection_ui;
    let recovery = ctx.session.site_recovery_status(&site.id, ctx.data);
    let progress = ctx.session.site_progress.get(&site.id);
    let summary = copy
        .progress_hint
        .replace("{explored}", &recovery.exploration_percent.to_string())
        .replace("{recovered}", &recovery.recovered_targets.to_string())
        .replace("{total}", &recovery.total_targets.to_string());
    text(
        &copy.progress,
        476.0,
        484.0,
        440.0,
        32.0,
        24.0,
        visual_theme::text(),
    );
    text(
        &summary,
        476.0,
        522.0,
        440.0,
        34.0,
        22.0,
        visual_theme::text_dim(),
    );
    text(
        &format!(
            "{} {}% · {} {}",
            copy.condition,
            progress.map_or(site.condition, |p| p.condition),
            copy.visits,
            progress.map_or(0, |p| p.visits)
        ),
        476.0,
        558.0,
        440.0,
        35.0,
        22.0,
        visual_theme::text_dim(),
    );
    let history = ctx
        .session
        .voyage_log
        .iter()
        .rev()
        .find(|r| r.site_id == site.id)
        .map_or_else(
            || copy.no_history.clone(),
            |r| {
                copy.history_hint
                    .replace("{count}", &r.recovered_count.to_string())
                    .replace("{value}", &r.recovered_value.to_string())
            },
        );
    text(
        &copy.history,
        476.0,
        606.0,
        440.0,
        30.0,
        23.0,
        visual_theme::text(),
    );
    text(
        &history,
        476.0,
        644.0,
        440.0,
        36.0,
        22.0,
        visual_theme::text_dim(),
    );
}
