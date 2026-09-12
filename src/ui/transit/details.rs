//! Outbound route preparation stays available without filling the flight scene.

use super::*;

pub fn draw_details(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    let Some(site) = ctx.data.sites.get(&expedition.site_id) else {
        return;
    };
    let copy = &ctx.data.transit_ui;
    draw_rectangle(
        0.0,
        84.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 84.0,
        visual_theme::with_alpha(BLACK, 0.76),
    );
    visual_theme::surface(Rect::new(220.0, 112.0, 840.0, 548.0));
    visual_theme::body(
        &copy.route_heading,
        Rect::new(248.0, 132.0, 730.0, 38.0),
        30.0,
        visual_theme::text(),
    );
    let fuel = ctx
        .session
        .effective_fuel_cost_with_plan(&site.id, ctx.data, expedition.voyage_plan)
        .unwrap_or(site.fuel_cost);
    let snapshot = copy
        .route_snapshot
        .replace("{danger}", &expedition.risk.danger_score.to_string())
        .replace("{plan}", expedition.voyage_plan.label())
        .replace("{fuel}", &fuel.to_string())
        .replace("{route}", ctx.session.route_familiarity_label(&site.id))
        .replace("{crew}", &ctx.session.crew_readiness().to_string())
        .replace("{scan}", expedition.scan_profile.result_label());
    visual_theme::body(
        &snapshot,
        Rect::new(248.0, 196.0, 356.0, 218.0),
        24.0,
        visual_theme::text(),
    );
    let coverage = if expedition.insured {
        copy.coverage.replace(
            "{premium}",
            &ctx.session
                .insurance_quote_with_plan(&site.id, ctx.data, expedition.voyage_plan)
                .map_or(0, |quote| quote.premium)
                .to_string(),
        )
    } else {
        copy.no_coverage.clone()
    };
    visual_theme::body(
        &coverage,
        Rect::new(248.0, 424.0, 352.0, 66.0),
        23.0,
        visual_theme::text_dim(),
    );
    let contract = if expedition.contract_accepted {
        site.contract_target
            .as_deref()
            .and_then(|id| ctx.data.salvage_objects.get(id))
            .map(|target| {
                copy.objective_detail
                    .replace("{target}", &target.display_name)
                    .replace("{brief}", &site.contract_brief)
                    .replace("{reward}", &site.contract_reward.to_string())
            })
            .unwrap_or_default()
    } else {
        copy.private_haul.clone()
    };
    visual_theme::body(
        &contract,
        Rect::new(648.0, 196.0, 378.0, 220.0),
        24.0,
        visual_theme::text(),
    );
    visual_theme::body(
        &copy.route_explanation,
        Rect::new(648.0, 424.0, 378.0, 156.0),
        21.0,
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(866.0, 596.0, 166.0, 44.0),
        &copy.close,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleTransitDetails);
    }
}
