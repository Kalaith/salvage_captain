//! Cargo capacity and the optional physical packing map.

use super::*;

pub(super) fn draw_cargo_hold(
    ctx: &UiContext<'_>,
    world: Rect,
    row: Rect,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.port_ui;
    panel(row, visual_theme::panel());
    text_at(
        &format!("{} L{}", copy.hold, ctx.session.cargo_bay_level()),
        Rect::new(row.x + 12.0, row.y + 4.0, 240.0, 26.0),
        visual_theme::text(),
    );
    text_at(
        &format!(
            "{} / {}",
            ctx.session.internal_cargo_count(ctx.data, None),
            ctx.session.internal_cargo_capacity()
        ),
        Rect::new(row.x + 12.0, row.y + 30.0, 100.0, 26.0),
        visual_theme::amber(),
    );
    let occupied = ctx.session.ship_layout.occupied_cells();
    let capacity = ctx.session.ship_layout.width * ctx.session.ship_layout.height;
    text_at(
        &format!("{occupied} / {capacity}"),
        Rect::new(row.x + 260.0, row.y + 4.0, 150.0, 26.0),
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(row.x + 260.0, row.y + 36.0, 150.0, 8.0),
        occupied as f32 / capacity as f32,
        visual_theme::amber(),
        "",
    );
    if button(
        ctx,
        Rect::new(row.right() - 324.0, row.y + 8.0, 172.0, 42.0),
        &ctx.session.cargo_bay_upgrade_label(),
        ctx.session
            .cargo_bay_upgrade_cost()
            .is_some_and(|cost| ctx.session.economy.credits >= cost),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::UpgradeCargoBay);
    }
    if button(
        ctx,
        Rect::new(row.right() - 140.0, row.y + 8.0, 128.0, 42.0),
        if ctx.port_hold_expanded {
            &copy.hide_grid
        } else {
            &copy.grid
        },
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePortHold);
    }
    if ctx.port_hold_expanded {
        let popup = Rect::new(world.x + 28.0, row.y - 184.0, 480.0, 166.0);
        panel(popup, visual_theme::panel());
        text_at(
            &copy.cargo_map,
            Rect::new(popup.x + 16.0, popup.y + 8.0, popup.w - 32.0, 26.0),
            visual_theme::cyan(),
        );
        crate::ui::draw_ship_grid(
            ctx,
            Rect::new(popup.x + 16.0, popup.y + 38.0, 142.0, 90.0),
            false,
            actions,
        );
        text_at(
            &copy.packing_hint,
            Rect::new(popup.x + 176.0, popup.y + 48.0, popup.w - 192.0, 90.0),
            visual_theme::text_dim(),
        );
    }
}
