//! Service preparation within the port rail.

use super::*;
use crate::state::maintenance::ServicePlan;
use crate::state::workspace_energy::{
    FIELD_POWER_CELL_ALLOY_COST, FIELD_POWER_CELL_ELECTRONICS_COST, FIELD_POWER_CELL_PRICE,
    MAX_FIELD_POWER_CELLS,
};
use crate::ui::port_panel::text_at;

pub fn draw_port_services(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    let condition = copy
        .condition
        .replace("{hull}", &ctx.session.hull.to_string())
        .replace(
            "{max}",
            &ctx.session.max_hull_with_modules(ctx.data).to_string(),
        )
        .replace("{modules}", &ctx.session.damaged_modules.len().to_string())
        .replace("{wear}", &ctx.session.ship_wear().to_string());
    text_at(
        &condition,
        Rect::new(frame.x, frame.y, frame.w, 28.0),
        visual_theme::amber(),
    );
    let fuel = ctx.session.refuel_quote(ctx.data);
    text_at(
        &format!(
            "{} {}/{}",
            copy.fuel,
            ctx.session.economy.fuel,
            ctx.session.max_fuel(ctx.data)
        ),
        Rect::new(frame.x, 208.0, 246.0, 26.0),
        visual_theme::text(),
    );
    let label = if fuel.amount > 0 {
        format!("{} {} CR", copy.refuel, fuel.cost)
    } else if ctx.session.economy.fuel >= ctx.session.max_fuel(ctx.data) {
        copy.full.clone()
    } else {
        copy.low_funds.clone()
    };
    if button(
        ctx,
        Rect::new(frame.right() - 172.0, 204.0, 172.0, 44.0),
        &label,
        fuel.amount > 0,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Refuel);
    }
    visual_theme::body(
        &copy
            .fuel_hint
            .replace("{amount}", &fuel.amount.to_string())
            .replace("{cost}", &fuel.cost.to_string()),
        Rect::new(frame.x, 234.0, 242.0, 22.0),
        17.0,
        visual_theme::text_dim(),
    );
    for (index, plan) in ServicePlan::ALL.into_iter().enumerate() {
        draw_service(
            ctx,
            Rect::new(frame.x, 260.0 + index as f32 * 64.0, frame.w, 58.0),
            plan,
            actions,
        );
    }
    draw_power(ctx, Rect::new(frame.x, 460.0, frame.w, 84.0), actions);
    crate::ui::port_panel::refinery::draw_refinery_console(
        ctx,
        Rect::new(frame.x, 556.0, frame.w, 80.0),
        actions,
    );
}

fn draw_service(ctx: &UiContext<'_>, card: Rect, plan: ServicePlan, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    let quote = ctx.session.service_quote(plan, ctx.data);
    panel(card, visual_theme::panel_soft());
    text_at(
        plan.label(),
        Rect::new(card.x + 10.0, card.y + 5.0, 244.0, 26.0),
        visual_theme::text(),
    );
    let scope = match plan {
        ServicePlan::Full => copy
            .repair_scope
            .replace("{hull}", &quote.missing_hull.to_string())
            .replace("{modules}", &quote.offline_modules.to_string())
            .replace("{wear}", &quote.ship_wear.to_string()),
        ServicePlan::Hull => format!("{} +{}", ctx.data.selection_ui.hull, quote.missing_hull),
        ServicePlan::Systems => copy
            .systems_scope
            .replace("{modules}", &quote.offline_modules.to_string())
            .replace("{wear}", &quote.ship_wear.to_string()),
    };
    let affordable = ctx.session.economy.credits >= quote.total_cost;
    let detail = if !affordable {
        copy.need_credits.replace(
            "{credits}",
            &(quote.total_cost - ctx.session.economy.credits).to_string(),
        )
    } else {
        scope
    };
    visual_theme::body(
        &detail,
        Rect::new(card.x + 10.0, card.y + 32.0, 244.0, 24.0),
        17.0,
        visual_theme::text_dim(),
    );
    let label = if !quote.is_due() {
        copy.not_due.clone()
    } else {
        format!("{} CR", quote.total_cost)
    };
    if button(
        ctx,
        Rect::new(card.right() - 156.0, card.y + 7.0, 148.0, 44.0),
        &label,
        quote.is_due() && affordable,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Service(plan));
    }
}

fn draw_power(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    let stock = ctx.session.field_power_cells;
    text_at(
        &copy
            .cell_stock
            .replace("{count}", &stock.to_string())
            .replace("{max}", &MAX_FIELD_POWER_CELLS.to_string())
            .replace("{alloy}", &ctx.session.economy.alloy.to_string())
            .replace(
                "{electronics}",
                &ctx.session.economy.electronics.to_string(),
            ),
        Rect::new(frame.x, frame.y, frame.w, 28.0),
        visual_theme::cyan(),
    );
    let make_label = if stock >= MAX_FIELD_POWER_CELLS {
        copy.full_stock.clone()
    } else if !ctx.session.can_fabricate_field_power_cell() {
        copy.need_materials
            .replace("{alloy}", &FIELD_POWER_CELL_ALLOY_COST.to_string())
            .replace(
                "{electronics}",
                &FIELD_POWER_CELL_ELECTRONICS_COST.to_string(),
            )
    } else {
        copy.make_cell.clone()
    };
    let buy_label = if stock >= MAX_FIELD_POWER_CELLS {
        copy.full_stock.clone()
    } else {
        format!("{} {} CR", copy.buy_cell, FIELD_POWER_CELL_PRICE)
    };
    for (index, (label, enabled, action)) in [
        (
            make_label,
            ctx.session.can_fabricate_field_power_cell(),
            UiAction::FabricateFieldPowerCell,
        ),
        (
            buy_label,
            ctx.session.can_buy_field_power_cell(),
            UiAction::BuyFieldPowerCell,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            ctx,
            Rect::new(frame.x + index as f32 * 216.0, frame.y + 32.0, 208.0, 44.0),
            &label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(action);
        }
    }
}
