//! Touch-first service-plan selection for safe-port recovery decisions.

use super::*;
use crate::state::maintenance::ServicePlan;
use crate::state::workspace_energy::{
    FIELD_POWER_CELL_ALLOY_COST, FIELD_POWER_CELL_ELECTRONICS_COST, FIELD_POWER_CELL_PRICE,
    MAX_FIELD_POWER_CELLS,
};
use crate::ui::visual_theme;

pub fn draw_open_button(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    let rect = Rect::new(
        width - 262.0,
        crate::ui::port_panel::HEADER_HEIGHT + 58.0,
        112.0,
        30.0,
    );
    if button(ctx, rect, "SERVICE", true, ButtonTone::Secondary) {
        actions.push(UiAction::ToggleServicePanel);
    }
}

pub fn draw_port_services(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    let height = ctx.viewport_height.max(1.0);
    draw_rectangle(
        0.0,
        crate::ui::port_panel::HEADER_HEIGHT,
        width,
        (height - crate::ui::port_panel::HEADER_HEIGHT).max(0.0),
        visual_theme::with_alpha(visual_theme::space(), 0.48),
    );
    let panel_width = (width * 0.40)
        .clamp(430.0, 540.0)
        .min((width - 36.0).max(280.0));
    let frame = Rect::new(
        (width - panel_width - 18.0).max(18.0),
        104.0,
        panel_width,
        (height - 122.0).max(390.0),
    );
    panel(frame, visual_theme::with_alpha(visual_theme::panel(), 0.98));
    panel_title(frame, "SERVICE BAY // SAFE PORT");
    draw_text(
        "CHOOSE WHAT THE YARD FIXES BEFORE THE NEXT DEPARTURE",
        frame.x + 18.0,
        frame.y + 62.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &current_condition_label(ctx),
        frame.x + 18.0,
        frame.y + 76.0,
        10.0,
        visual_theme::amber(),
    );
    let close = Rect::new(frame.right() - 88.0, frame.y + 8.0, 70.0, 26.0);
    if button(ctx, close, "CLOSE", true, ButtonTone::Secondary) {
        actions.push(UiAction::ToggleServicePanel);
    }

    for (index, plan) in ServicePlan::ALL.into_iter().enumerate() {
        let card = Rect::new(
            frame.x + 18.0,
            frame.y + 92.0 + index as f32 * 136.0,
            frame.w - 36.0,
            124.0,
        );
        draw_service_card(ctx, card, plan, actions);
    }
    draw_field_power_supply(ctx, frame, actions);
}

fn draw_field_power_supply(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    let stock = ctx.session.field_power_cells;
    let buy_label = if stock >= MAX_FIELD_POWER_CELLS {
        "STOCK FULL".to_owned()
    } else if ctx.session.economy.credits < FIELD_POWER_CELL_PRICE {
        format!("LOW CR ¢{FIELD_POWER_CELL_PRICE}")
    } else {
        format!("BUY CELL ¢{FIELD_POWER_CELL_PRICE}")
    };
    let fabricate_label = if stock >= MAX_FIELD_POWER_CELLS {
        "STOCK FULL".to_owned()
    } else if !ctx.session.can_fabricate_field_power_cell() {
        format!(
            "NEED A{} E{}",
            FIELD_POWER_CELL_ALLOY_COST, FIELD_POWER_CELL_ELECTRONICS_COST
        )
    } else {
        format!(
            "MAKE CELL A{} E{}",
            FIELD_POWER_CELL_ALLOY_COST, FIELD_POWER_CELL_ELECTRONICS_COST
        )
    };
    draw_rectangle(
        frame.x,
        frame.bottom() - 68.0,
        frame.w,
        68.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        &format!(
            "FIELD POWER  //  CELLS {stock}/{MAX_FIELD_POWER_CELLS}  //  +4 EACH  //  SALVAGE A{} E{}",
            ctx.session.economy.alloy, ctx.session.economy.electronics
        ),
        frame.x + 18.0,
        frame.bottom() - 47.0,
        10.0,
        visual_theme::cyan(),
    );
    if button(
        ctx,
        Rect::new(frame.x + 18.0, frame.bottom() - 40.0, 172.0, 26.0),
        &fabricate_label,
        ctx.session.can_fabricate_field_power_cell(),
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::FabricateFieldPowerCell);
    }
    if button(
        ctx,
        Rect::new(frame.right() - 190.0, frame.bottom() - 40.0, 172.0, 26.0),
        &buy_label,
        ctx.session.can_buy_field_power_cell(),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BuyFieldPowerCell);
    }
}

fn draw_service_card(
    ctx: &UiContext<'_>,
    card: Rect,
    plan: ServicePlan,
    actions: &mut Vec<UiAction>,
) {
    let quote = ctx.session.service_quote(plan, ctx.data);
    let due = quote.is_due();
    let affordable = ctx.session.economy.credits >= quote.total_cost;
    let accent = match plan {
        ServicePlan::Full => visual_theme::safe(),
        ServicePlan::Hull => visual_theme::amber(),
        ServicePlan::Systems => visual_theme::cyan(),
    };
    panel(
        card,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.92),
    );
    draw_rectangle(card.x, card.y, 4.0, card.h, accent);
    draw_text(
        plan.label(),
        card.x + 16.0,
        card.y + 23.0,
        16.0,
        visual_theme::text(),
    );
    draw_text(
        &clipped(plan.description(), 58),
        card.x + 16.0,
        card.y + 44.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &service_scope_label(quote),
        card.x + 16.0,
        card.y + 65.0,
        10.0,
        accent,
    );
    draw_text(
        &clipped(&service_cost_label(quote), 58),
        card.x + 16.0,
        card.y + 84.0,
        9.0,
        visual_theme::text_dim(),
    );
    let button_label = if !due {
        "NOT DUE".to_owned()
    } else if !affordable {
        format!("LOW CR ¢{}", quote.total_cost)
    } else {
        format!("{}  ¢{}", plan.button_label(), quote.total_cost)
    };
    let button_rect = Rect::new(card.x + 16.0, card.bottom() - 38.0, 178.0, 28.0);
    if button(
        ctx,
        button_rect,
        &button_label,
        due && affordable,
        match plan {
            ServicePlan::Full => ButtonTone::Positive,
            ServicePlan::Hull => ButtonTone::Warning,
            ServicePlan::Systems => ButtonTone::Primary,
        },
    ) {
        actions.push(UiAction::Service(plan));
    }
    let (status, status_color) = service_status_label(quote, ctx.session.economy.credits);
    draw_text(
        &clipped(&status, 31),
        card.x + 210.0,
        card.bottom() - 19.0,
        9.0,
        status_color,
    );
}

fn current_condition_label(ctx: &UiContext<'_>) -> String {
    format!(
        "CURRENT  //  HULL {}/{}  //  MODULES {}  //  WEAR {}%  //  CREDITS ¢{}",
        ctx.session.hull,
        ctx.session.max_hull_with_modules(ctx.data),
        ctx.session.damaged_modules.len(),
        ctx.session.ship_wear(),
        ctx.session.economy.credits,
    )
}

fn service_scope_label(quote: crate::state::maintenance::ServiceQuote) -> String {
    match quote.plan {
        ServicePlan::Full => format!(
            "RESTORES HULL {}  //  MODULES {}  //  WEAR {}%",
            quote.missing_hull, quote.offline_modules, quote.ship_wear
        ),
        ServicePlan::Hull => {
            format!(
                "RESTORES HULL {}  //  LEAVES SYSTEMS & WEAR",
                quote.missing_hull
            )
        }
        ServicePlan::Systems => format!(
            "RESTORES MODULES {}  //  WEAR {}%  //  LEAVES HULL",
            quote.offline_modules, quote.ship_wear
        ),
    }
}

fn service_cost_label(quote: crate::state::maintenance::ServiceQuote) -> String {
    match quote.plan {
        ServicePlan::Full => format!(
            "COST HULL ¢{}  //  MODULES ¢{}  //  WEAR ¢{}",
            quote.hull_cost, quote.module_cost, quote.wear_cost
        ),
        ServicePlan::Hull => format!("COST HULL ¢{}", quote.hull_cost),
        ServicePlan::Systems => format!(
            "COST MODULES ¢{}  //  WEAR ¢{}",
            quote.module_cost, quote.wear_cost
        ),
    }
}

fn service_status_label(
    quote: crate::state::maintenance::ServiceQuote,
    credits: i64,
) -> (String, Color) {
    if !quote.is_due() {
        (
            "STATUS NOMINAL // NO SERVICE DUE".to_owned(),
            visual_theme::text_dim(),
        )
    } else if credits < quote.total_cost {
        (
            format!(
                "STATUS LOW FUNDS // NEED ¢{} MORE",
                quote.total_cost - credits
            ),
            visual_theme::warning(),
        )
    } else {
        (
            "STATUS READY // TOUCH TO AUTHORIZE".to_owned(),
            visual_theme::safe(),
        )
    }
}
