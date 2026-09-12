//! Cargo dock telemetry and an on-demand storage drawer.

use super::*;

pub(super) fn draw_cargo_hold(ctx: &UiContext<'_>, row: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    if button(ctx, row, "", true, ButtonTone::Secondary) {
        actions.push(UiAction::TogglePortHold);
    }
    chrome::draw_icon(
        vec2(row.x + 28.0, row.y + 29.0),
        chrome::DockIcon::Cargo,
        visual_theme::text_dim(),
    );
    let occupied = ctx.session.internal_cargo_count(ctx.data, None);
    let capacity = ctx.session.internal_cargo_capacity();
    visual_theme::body(
        &format!("{}  {occupied} / {capacity}", copy.hold),
        Rect::new(row.x + 56.0, row.y + 9.0, row.w - 64.0, 26.0),
        21.0,
        visual_theme::text(),
    );
    for index in 0..12 {
        let segment = Rect::new(row.x + 56.0 + index as f32 * 16.0, row.y + 39.0, 13.0, 10.0);
        let filled = (index as f32) < occupied as f32 / capacity.max(1) as f32 * 12.0;
        draw_rectangle(
            segment.x,
            segment.y,
            segment.w,
            segment.h,
            if filled {
                visual_theme::safe()
            } else {
                visual_theme::structure_dark()
            },
        );
        draw_rectangle_lines(
            segment.x,
            segment.y,
            segment.w,
            segment.h,
            1.0,
            visual_theme::structure_light(),
        );
    }
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
