//! Shared contract, preparation controls and a single departure action.

use super::*;

pub(super) fn draw(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, site: &crate::data::SiteData) {
    draw_contract(ctx, actions, site);
    let copy = &ctx.data.selection_ui;
    text(
        &copy.preparation,
        476.0,
        434.0,
        210.0,
        34.0,
        27.0,
        visual_theme::text(),
    );
    for (y, label, action) in [
        (
            486.0,
            format!("{}: {}", copy.plan, ctx.voyage_plan.label()),
            UiAction::CycleVoyagePlan,
        ),
        (
            542.0,
            format!("{}: {}", copy.crew, ctx.session.crew_role().short_label()),
            UiAction::CycleCrew,
        ),
    ] {
        if button(
            ctx,
            Rect::new(476.0, y, 440.0, 44.0),
            &label,
            true,
            ButtonTone::Secondary,
        ) {
            actions.push(action);
        }
    }
    let quote = ctx.session.reconnaissance_quote(&site.id, ctx.data);
    let label = quote.as_ref().map_or_else(
        || copy.intel_max.clone(),
        |quote| format!("{} · ¢{}", copy.intel, quote.cost),
    );
    if button(
        ctx,
        Rect::new(476.0, 598.0, 440.0, 44.0),
        &label,
        ctx.session.can_buy_reconnaissance(&site.id, ctx.data),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BuyReconnaissance(site.id.clone()));
    }
    text(
        if ctx.session.crew_fatigue() > 0 {
            &copy.tired
        } else {
            ctx.voyage_plan.description()
        },
        476.0,
        654.0,
        440.0,
        30.0,
        20.0,
        if ctx.session.crew_fatigue() > 0 {
            visual_theme::warning()
        } else {
            visual_theme::text_dim()
        },
    );
}

fn draw_contract(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, site: &crate::data::SiteData) {
    let copy = &ctx.data.selection_ui;
    text(
        if ctx.wreck_selection.private_haul {
            &copy.private_haul
        } else {
            &copy.contract
        },
        44.0,
        434.0,
        410.0,
        36.0,
        28.0,
        visual_theme::text(),
    );
    let progress = ctx.session.site_progress.get(&site.id);
    let completed = progress.is_some_and(|p| p.contract_completed);
    let failed = progress.is_some_and(|p| p.contract_failed);
    if ctx.wreck_selection.private_haul {
        text(
            &copy.private_hint,
            44.0,
            486.0,
            398.0,
            110.0,
            24.0,
            visual_theme::text_dim(),
        );
    } else if completed || failed || site.contract_target.is_none() {
        let status = if completed {
            &copy.completed
        } else if failed {
            &copy.failed
        } else {
            &copy.no_contract
        };
        text(status, 44.0, 486.0, 400.0, 45.0, 24.0, visual_theme::text());
        text(
            &copy.no_contract,
            44.0,
            542.0,
            398.0,
            70.0,
            22.0,
            visual_theme::text_dim(),
        );
    } else {
        let target = site
            .contract_target
            .as_deref()
            .and_then(|id| ctx.data.salvage_objects.get(id));
        if let Some(target) = target {
            text(
                &target.display_name,
                44.0,
                480.0,
                400.0,
                38.0,
                25.0,
                visual_theme::text(),
            );
        }
        text(
            &site.contract_brief,
            44.0,
            522.0,
            398.0,
            50.0,
            20.0,
            visual_theme::text_dim(),
        );
        text(
            &ctx.data
                .discovery
                .copy
                .requirements
                .replace("{equipment}", &ctx.session.wreck_equipment(site, ctx.data)),
            44.0,
            574.0,
            400.0,
            26.0,
            17.0,
            visual_theme::text_dim(),
        );
        text(
            &format!(
                "{} ¢{}",
                copy.contract_reward,
                site.contract_reward
                    + ctx.session.contract_reward_bonus(site.contract_reward)
                    + ctx.session.next_contract_streak_bonus()
            ),
            44.0,
            600.0,
            400.0,
            30.0,
            24.0,
            visual_theme::text(),
        );
    }
    draw_private_toggle(ctx, actions);
}

fn draw_private_toggle(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.selection_ui;
    let label = format!(
        "{}: {}",
        copy.private_haul,
        if ctx.wreck_selection.private_haul {
            &copy.on
        } else {
            &copy.off
        }
    );
    if button(
        ctx,
        Rect::new(44.0, 640.0, 400.0, 44.0),
        &label,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::WreckSelection(SelectionAction::TogglePrivate));
    }
}

pub(super) fn draw_departure(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
) {
    let copy = &ctx.data.selection_ui;
    let can_depart = ctx
        .session
        .can_depart_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let insured = ctx.wreck_selection.insured;
    let can_pay = !insured
        || ctx
            .session
            .can_depart_insured_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let status = if ctx.session.wreck_depleted(&site.id, ctx.data) {
        &ctx.data.discovery.copy.depleted_departure
    } else if !can_depart {
        &copy.fuel_short
    } else if !can_pay {
        &copy.credits_short
    } else {
        &copy.ready
    };
    text(
        status,
        948.0,
        434.0,
        288.0,
        40.0,
        25.0,
        if can_depart && can_pay {
            visual_theme::text()
        } else {
            visual_theme::warning()
        },
    );
    draw_insurance(ctx, actions, site);
    draw_fuel(ctx, site);
    if button(
        ctx,
        Rect::new(948.0, 640.0, 288.0, 44.0),
        &copy.depart,
        can_depart && can_pay,
        ButtonTone::Primary,
    ) {
        actions.push(ctx.wreck_selection.departure_to(&site.id));
    }
}

fn draw_fuel(ctx: &UiContext<'_>, site: &crate::data::SiteData) {
    let copy = &ctx.data.selection_ui;
    let trip = ctx
        .session
        .effective_fuel_cost_with_plan(&site.id, ctx.data, ctx.voyage_plan)
        .unwrap_or(site.fuel_cost);
    let fuel = copy
        .fuel_hint
        .replace("{trip}", &trip.to_string())
        .replace("{required}", &fuel_required(ctx, site).to_string());
    text(
        &fuel,
        948.0,
        582.0,
        288.0,
        56.0,
        20.0,
        visual_theme::text_dim(),
    );
}

fn draw_insurance(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, site: &crate::data::SiteData) {
    let copy = &ctx.data.selection_ui;
    let quote = ctx
        .session
        .insurance_quote_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let label = if ctx.wreck_selection.insured {
        format!(
            "{} · ¢{}",
            copy.cover_on,
            quote.as_ref().map_or(0, |q| q.premium)
        )
    } else {
        quote.as_ref().map_or_else(
            || copy.cover_off.clone(),
            |q| format!("{} +¢{}", copy.cover, q.premium),
        )
    };
    if button(
        ctx,
        Rect::new(948.0, 486.0, 288.0, 44.0),
        &label,
        !ctx.wreck_selection.private_haul && quote.is_some(),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::WreckSelection(SelectionAction::ToggleInsurance));
    }
    let hint = if ctx.wreck_selection.private_haul {
        copy.coverage_unavailable.clone()
    } else {
        copy.cover_hint.replace(
            "{percent}",
            &ctx.data.config.insurance.coverage_percent.to_string(),
        )
    };
    text(
        &hint,
        948.0,
        540.0,
        288.0,
        52.0,
        20.0,
        visual_theme::text_dim(),
    );
}
