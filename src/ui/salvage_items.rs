//! Optional cargo inventory: hold layout, capacity, and return controls.

use super::*;
mod manifest;

pub fn draw_inventory(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_hold(ctx, actions);
    manifest::draw_manifest(ctx, actions);
}

fn draw_hold(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.salvage_ui.inventory_copy;
    let hold = Rect::new(24.0, 84.0, 450.0, 588.0);
    visual_theme::surface(hold);
    visual_theme::body(
        &copy.hold_title,
        Rect::new(44.0, 102.0, 410.0, 32.0),
        26.0,
        visual_theme::text(),
    );
    let capacity = copy
        .capacity
        .replace(
            "{used}",
            &ctx.session.internal_cargo_count(ctx.data, None).to_string(),
        )
        .replace(
            "{capacity}",
            &ctx.session.internal_cargo_capacity().to_string(),
        )
        .replace(
            "{external}",
            &ctx.session.external_cargo_count(ctx.data, None).to_string(),
        )
        .replace(
            "{clamps}",
            &ctx.session.external_capacity(ctx.data).to_string(),
        );
    visual_theme::body(
        &capacity,
        Rect::new(44.0, 144.0, 410.0, 28.0),
        20.0,
        visual_theme::cyan(),
    );
    draw_ship_grid(ctx, Rect::new(52.0, 186.0, 394.0, 320.0), true, actions);
    if ctx.dragged_item.is_some() {
        visual_theme::body(
            &copy.placing_hint,
            Rect::new(44.0, 518.0, 410.0, 46.0),
            19.0,
            visual_theme::text(),
        );
        if button(
            ctx,
            Rect::new(44.0, 580.0, 410.0, 48.0),
            &copy.cancel_move,
            true,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::CancelDrag);
        }
    } else {
        if ctx.session.inventory_cargo().next().is_some() {
            visual_theme::body(
                &copy.move_hint,
                Rect::new(44.0, 518.0, 410.0, 28.0),
                19.0,
                visual_theme::text_dim(),
            );
        }
        draw_return_controls(ctx, actions);
    }
}

fn draw_return_controls(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.salvage_ui.inventory_copy;
    let risk = ctx
        .session
        .expedition_risk_preview(ctx.data)
        .map_or(0, |risk| risk.danger_score);
    visual_theme::body(
        &copy.return_risk.replace("{risk}", &risk.to_string()),
        Rect::new(44.0, 558.0, 210.0, 28.0),
        20.0,
        danger_color(risk),
    );
    visual_theme::body(
        &copy.return_fuel.replace(
            "{fuel}",
            &ctx.data.config.safe_return_buffer.max(0).to_string(),
        ),
        Rect::new(266.0, 558.0, 188.0, 28.0),
        20.0,
        visual_theme::text_dim(),
    );
    let policy = ctx.session.return_policy().unwrap_or_default();
    if button(
        ctx,
        Rect::new(44.0, 602.0, 410.0, 48.0),
        &copy.policy.replace("{policy}", policy.label()),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleReturnPolicy);
    }
}

pub(super) fn draw_grid_label(
    ctx: &UiContext<'_>,
    item: &crate::engine::packing::PlacedItem,
    rect: Rect,
) {
    let label = if item.permanent {
        ctx.data.modules.get(&item.id).map_or_else(
            || short_label(&item.id),
            |module| module.display_name.clone(),
        )
    } else {
        let object_id = item.id.strip_prefix("cargo:").unwrap_or(&item.id);
        ctx.session
            .inventory_cargo()
            .position(|cargo| cargo.object_id == object_id)
            .map_or_else(String::new, |index| (index + 1).to_string())
    };
    visual_theme::body(
        &label,
        Rect::new(rect.x + 6.0, rect.y + 10.0, rect.w - 12.0, rect.h - 16.0),
        if item.permanent { 18.0 } else { 24.0 },
        visual_theme::text(),
    );
}
