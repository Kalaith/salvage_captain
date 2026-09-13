//! Thin port status strip; materials live in Service and market copy in its hover hint.

use super::*;

pub(super) const LOG_RECT: Rect = Rect::new(1112.0, 4.0, 88.0, 44.0);
pub(super) const MENU_RECT: Rect = Rect::new(1208.0, 4.0, 48.0, 44.0);
pub(super) const TITLE_RECT: Rect = Rect::new(24.0, 4.0, 204.0, 44.0);

pub fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        HEADER_HEIGHT,
        visual_theme::panel(),
    );
    draw_line(
        0.0,
        HEADER_HEIGHT,
        LOGICAL_WIDTH,
        HEADER_HEIGHT,
        1.0,
        visual_theme::cyan_dim(),
    );
    draw_circle(27.0, 26.0, 2.5, visual_theme::safe());
    visual_theme::body(
        &ctx.data.port_ui.title_short,
        Rect::new(38.0, 16.0, 190.0, 24.0),
        20.0,
        visual_theme::text(),
    );
    let hull_max = ctx.session.max_hull_with_modules(ctx.data);
    for (x, width, label, color) in [
        (
            248.0,
            130.0,
            format!(
                "{} {}",
                ctx.data.port_ui.credits,
                crate::ui::header::grouped_credits(ctx.session.economy.credits)
            ),
            visual_theme::amber(),
        ),
        (
            398.0,
            132.0,
            format!(
                "FUEL {}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            550.0,
            132.0,
            format!("HULL {}/{}", ctx.session.hull, hull_max),
            if ctx.session.hull < hull_max {
                visual_theme::warning()
            } else {
                visual_theme::text()
            },
        ),
    ] {
        visual_theme::body(&label, Rect::new(x, 17.0, width, 23.0), 19.0, color);
    }
    let log_label =
        voyage_archive::archive_button_label(ctx.session.voyage_log.len(), ctx.voyage_archive_open);
    if chrome::edge_button(ctx, LOG_RECT, ctx.voyage_archive_open, false) {
        actions.push(UiAction::ToggleVoyageArchive);
    }
    visual_theme::body(
        &log_label,
        Rect::new(LOG_RECT.x + 12.0, 17.0, 76.0, 23.0),
        18.0,
        visual_theme::text(),
    );
    if chrome::edge_button(ctx, MENU_RECT, false, false) {
        actions.push(UiAction::TogglePause);
    }
    for y in [20.0, 26.0, 32.0] {
        draw_line(1223.0, y, 1241.0, y, 1.5, visual_theme::text());
    }
}
