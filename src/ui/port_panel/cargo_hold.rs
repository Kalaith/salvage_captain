//! Cargo capacity on the edge HUD and an on-demand storage drawer.

use super::*;

pub(super) fn draw_cargo_hold(ctx: &UiContext<'_>, row: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    if chrome::edge_button(ctx, row, ctx.port_hold_expanded, false) {
        actions.push(UiAction::TogglePortHold);
    }
    chrome::draw_icon(
        vec2(row.x + 20.0, row.y + 22.0),
        chrome::DockIcon::Cargo,
        visual_theme::text_dim(),
    );
    let occupied = ctx.session.internal_cargo_count(ctx.data, None);
    let capacity = ctx.session.internal_cargo_capacity();
    visual_theme::body(
        &format!("{occupied} / {capacity}"),
        Rect::new(row.x + 40.0, row.y + 13.0, 58.0, 23.0),
        18.0,
        visual_theme::text(),
    );
    let meter = Rect::new(row.x + 104.0, row.y + 20.0, 58.0, 4.0);
    draw_rectangle(
        meter.x,
        meter.y,
        meter.w,
        meter.h,
        visual_theme::structure(),
    );
    draw_rectangle(
        meter.x,
        meter.y,
        meter.w * (occupied as f32 / capacity.max(1) as f32).clamp(0.0, 1.0),
        meter.h,
        visual_theme::safe(),
    );
    if !ctx.port_hold_expanded {
        return;
    }
    let popup = Rect::new(28.0, 362.0, 510.0, 226.0);
    visual_theme::surface(popup);
    text_at(
        &format!("{} / L{}", copy.cargo_map, ctx.session.cargo_bay_level()),
        Rect::new(popup.x + 16.0, popup.y + 12.0, 330.0, 28.0),
        visual_theme::cyan(),
    );
    if button(
        ctx,
        Rect::new(popup.right() - 112.0, popup.y + 4.0, 100.0, 44.0),
        &copy.close,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePortHold);
    }
    crate::ui::draw_ship_grid(
        ctx,
        Rect::new(popup.x + 16.0, popup.y + 54.0, 160.0, 100.0),
        false,
        actions,
    );
    text_at(
        &copy.packing_hint,
        Rect::new(popup.x + 198.0, popup.y + 58.0, 292.0, 60.0),
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(popup.x + 198.0, popup.y + 132.0, 292.0, 48.0),
        &ctx.session.cargo_bay_upgrade_label(),
        ctx.session
            .cargo_bay_upgrade_cost()
            .is_some_and(|cost| ctx.session.economy.credits >= cost),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::UpgradeCargoBay);
    }
}
