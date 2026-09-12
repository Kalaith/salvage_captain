//! Port cargo-bay readout and physical packing map control.

use super::*;
use crate::ui::visual_theme;

pub(super) fn draw_cargo_hold(
    ctx: &UiContext<'_>,
    world: Rect,
    row: Rect,
    actions: &mut Vec<UiAction>,
) {
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        visual_theme::with_alpha(visual_theme::panel(), 0.86),
    );
    draw_rectangle(row.x, row.y, 4.0, row.h, visual_theme::amber());
    draw_line(
        row.x,
        row.y - 10.0,
        row.right(),
        row.y - 10.0,
        1.0,
        visual_theme::with_alpha(visual_theme::structure_light(), 0.5),
    );
    draw_text(
        format!(
            "CARGO HOLD L{}  //  BERTHS {}/{}",
            ctx.session.cargo_bay_level(),
            ctx.session.internal_cargo_count(ctx.data, None),
            ctx.session.internal_cargo_capacity()
        ),
        row.x + 16.0,
        row.y + 18.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!(
            "{} / {} CELLS",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        row.x + 16.0,
        row.y + 39.0,
        17.0,
        visual_theme::amber(),
    );

    let button_width = 112.0_f32.min((row.w - 28.0).max(80.0));
    let button_gap = 8.0;
    let view_rect = Rect::new(
        row.right() - button_width - 12.0,
        row.y + 9.0,
        button_width,
        30.0,
    );
    let upgrade_rect = Rect::new(
        view_rect.x - button_width - button_gap,
        row.y + 9.0,
        button_width,
        30.0,
    );
    let can_upgrade = ctx
        .session
        .cargo_bay_upgrade_cost()
        .is_some_and(|cost| ctx.session.economy.credits >= cost);
    let meter_x = row.x + 176.0;
    let meter_width = (upgrade_rect.x - meter_x - 18.0).max(110.0);
    draw_text(
        "PHYSICAL PACKING CAPACITY",
        meter_x,
        row.y + 16.0,
        9.0,
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(meter_x, row.y + 25.0, meter_width, 10.0),
        ctx.session.ship_layout.occupied_cells() as f32
            / (ctx.session.ship_layout.width * ctx.session.ship_layout.height) as f32,
        visual_theme::amber(),
        &format!(
            "{} / {}",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
    );
    if button(
        ctx,
        upgrade_rect,
        &ctx.session.cargo_bay_upgrade_label(),
        can_upgrade,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::UpgradeCargoBay);
    }
    if button(
        ctx,
        view_rect,
        if ctx.port_hold_expanded {
            "HIDE GRID"
        } else {
            "VIEW GRID"
        },
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePortHold);
    }

    if ctx.port_hold_expanded {
        let popup_width = 300.0_f32.min((world.w - 28.0).max(230.0));
        let popup_height = 168.0;
        let popup = Rect::new(
            row.x + 18.0,
            (row.y - popup_height - 14.0).max(world.y + 18.0),
            popup_width,
            popup_height,
        );
        panel(popup, visual_theme::with_alpha(visual_theme::panel(), 0.98));
        draw_text(
            "CARGO MAP  //  5 × 5",
            popup.x + 16.0,
            popup.y + 24.0,
            12.0,
            visual_theme::cyan(),
        );
        crate::ui::draw_ship_grid(
            ctx,
            Rect::new(popup.x + 16.0, popup.y + 34.0, 142.0, 100.0),
            false,
            actions,
        );
        draw_text(
            "Recovered hardware must fit before return.",
            popup.x + 176.0,
            popup.y + 74.0,
            11.0,
            visual_theme::text_dim(),
        );
    }
}
