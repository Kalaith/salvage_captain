//! Compact port telemetry, bottom navigation and operational status strip.

use super::*;

pub fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    panel(
        Rect::new(0.0, 0.0, LOGICAL_WIDTH, HEADER_HEIGHT),
        visual_theme::panel(),
    );
    visual_theme::body(
        &copy.title_short,
        Rect::new(28.0, 10.0, 210.0, 27.0),
        23.0,
        visual_theme::text(),
    );
    draw_circle(32.0, 48.0, 3.0, visual_theme::safe());
    visual_theme::body(
        &copy.checkpoint,
        Rect::new(42.0, 38.0, 185.0, 23.0),
        17.0,
        visual_theme::safe(),
    );
    draw_resources(ctx);
    draw_line(840.0, 13.0, 840.0, 55.0, 1.0, visual_theme::structure());
    visual_theme::body(
        &market_ticker(ctx),
        Rect::new(856.0, 10.0, 242.0, 50.0),
        17.0,
        visual_theme::cyan(),
    );
    if button(
        ctx,
        Rect::new(1110.0, 12.0, 74.0, 44.0),
        &voyage_archive::archive_button_label(
            ctx.session.voyage_log.len(),
            ctx.voyage_archive_open,
        ),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleVoyageArchive);
    }
    if button(
        ctx,
        Rect::new(1198.0, 12.0, 54.0, 44.0),
        "",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
    for y in [25.0, 34.0, 43.0] {
        draw_line(1214.0, y, 1236.0, y, 2.0, visual_theme::text());
    }
}

fn draw_resources(ctx: &UiContext<'_>) {
    let copy = &ctx.data.port_ui;
    let resources = [
        (
            246.0,
            102.0,
            format!(
                "{} {}",
                copy.credits,
                grouped_credits(ctx.session.economy.credits)
            ),
            visual_theme::amber(),
        ),
        (
            370.0,
            116.0,
            format!(
                "{} {}/{}",
                copy.fuel,
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            506.0,
            106.0,
            format!(
                "{} {}/{}",
                ctx.data.selection_ui.hull,
                ctx.session.hull,
                ctx.session.max_hull_with_modules(ctx.data)
            ),
            if ctx.session.hull < ctx.session.max_hull_with_modules(ctx.data) {
                visual_theme::warning()
            } else {
                visual_theme::text()
            },
        ),
        (
            632.0,
            90.0,
            format!("{} {}", copy.alloy, ctx.session.economy.alloy),
            visual_theme::text_dim(),
        ),
        (
            742.0,
            86.0,
            format!("{} {}", copy.electronics, ctx.session.economy.electronics),
            visual_theme::text_dim(),
        ),
    ];
    for (x, width, label, color) in resources {
        draw_line(
            x - 14.0,
            13.0,
            x - 14.0,
            55.0,
            1.0,
            visual_theme::structure(),
        );
        visual_theme::body(&label, Rect::new(x, 23.0, width, 28.0), 21.0, color);
    }
}

fn grouped_credits(value: i64) -> String {
    let digits = value.to_string();
    let mut result = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) && digit != '-' {
            result.push(',');
        }
        result.push(digit);
    }
    result
}

pub(super) fn draw_dock(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    for (tab, label, hint, icon) in [
        (
            PortTab::Service,
            &copy.service,
            &copy.service_hint,
            DockIcon::Service,
        ),
        (
            PortTab::Equipment,
            &copy.equipment,
            &copy.equipment_hint,
            DockIcon::Equipment,
        ),
        (
            PortTab::Crew,
            &copy.crew,
            &copy.dock_crew_hint,
            DockIcon::Crew,
        ),
    ] {
        let rect = tab_rect(tab);
        if button(ctx, rect, "", true, ButtonTone::Secondary) {
            actions.push(UiAction::SelectPortTab(tab));
        }
        if ctx.port_tab == tab {
            draw_rectangle(
                rect.x,
                rect.bottom() - 3.0,
                rect.w,
                3.0,
                visual_theme::cyan(),
            );
        }
        draw_icon(
            vec2(rect.x + 26.0, rect.y + 30.0),
            icon,
            visual_theme::text_dim(),
        );
        visual_theme::body(
            label,
            Rect::new(rect.x + 50.0, rect.y + 10.0, rect.w - 56.0, 26.0),
            22.0,
            visual_theme::text(),
        );
        visual_theme::body(
            hint,
            Rect::new(rect.x + 50.0, rect.y + 35.0, rect.w - 56.0, 22.0),
            16.0,
            visual_theme::text_dim(),
        );
    }
    let depart = departure_rect();
    if button(ctx, depart, "", true, ButtonTone::Primary) {
        actions.push(UiAction::GoToSites);
    }
    draw_icon(
        vec2(depart.x + 35.0, depart.y + 36.0),
        DockIcon::Depart,
        visual_theme::safe(),
    );
    visual_theme::body(
        &copy.depart,
        Rect::new(depart.x + 68.0, depart.y + 12.0, depart.w - 80.0, 30.0),
        28.0,
        visual_theme::safe(),
    );
    visual_theme::body(
        &copy.depart_hint,
        Rect::new(depart.x + 68.0, depart.y + 43.0, depart.w - 80.0, 24.0),
        20.0,
        visual_theme::text_dim(),
    );
}

pub(super) fn draw_footer(ctx: &UiContext<'_>) {
    panel(
        Rect::new(0.0, 684.0, LOGICAL_WIDTH, 36.0),
        visual_theme::panel(),
    );
    let message = if ctx.message == crate::game::prompts::state_prompt(GameState::Port) {
        &ctx.data.port_ui.instruction
    } else {
        ctx.message
    };
    draw_circle_lines(22.0, 700.0, 4.0, 1.0, visual_theme::amber());
    visual_theme::body(
        message,
        Rect::new(38.0, 687.0, 932.0, 31.0),
        16.0,
        visual_theme::text_dim(),
    );
    visual_theme::body(
        &ctx.data.port_ui.registry,
        Rect::new(986.0, 693.0, 274.0, 22.0),
        16.0,
        visual_theme::text_dim(),
    );
}

pub(super) enum DockIcon {
    Service,
    Equipment,
    Crew,
    Depart,
    Cargo,
}

pub(super) fn draw_icon(center: Vec2, icon: DockIcon, color: Color) {
    let (x, y) = (center.x, center.y);
    match icon {
        DockIcon::Service => {
            draw_line(x - 8.0, y - 8.0, x + 11.0, y + 11.0, 6.0, color);
            draw_circle(x - 8.0, y - 8.0, 8.0, color);
            draw_triangle(
                vec2(x - 17.0, y - 16.0),
                vec2(x - 3.0, y - 16.0),
                vec2(x - 8.0, y - 5.0),
                visual_theme::panel(),
            );
            draw_circle(x + 10.0, y + 10.0, 2.0, visual_theme::panel());
        }
        DockIcon::Equipment => {
            for index in 0..8 {
                let angle = index as f32 * std::f32::consts::TAU / 8.0;
                let axis = vec2(angle.cos(), angle.sin());
                let start = center + axis * 8.0;
                let end = center + axis * 15.0;
                draw_line(start.x, start.y, end.x, end.y, 6.0, color);
            }
            draw_circle(x, y, 11.0, color);
            draw_circle(x, y, 5.0, visual_theme::panel());
        }
        DockIcon::Crew => {
            draw_circle(x - 4.0, y - 7.0, 6.0, color);
            draw_circle(x + 8.0, y - 5.0, 5.0, visual_theme::with_alpha(color, 0.65));
            draw_rectangle(x - 13.0, y + 3.0, 19.0, 10.0, color);
            draw_rectangle(
                x + 8.0,
                y + 3.0,
                8.0,
                10.0,
                visual_theme::with_alpha(color, 0.65),
            );
        }
        DockIcon::Depart => {
            draw_triangle(
                vec2(x - 14.0, y - 5.0),
                vec2(x + 14.0, y - 14.0),
                vec2(x + 5.0, y + 14.0),
                color,
            );
            draw_line(x - 12.0, y + 14.0, x + 8.0, y - 8.0, 4.0, color);
            draw_line(
                x - 4.0,
                y + 3.0,
                x + 8.0,
                y - 8.0,
                2.0,
                visual_theme::panel(),
            );
        }
        DockIcon::Cargo => {
            draw_rectangle_lines(x - 13.0, y - 9.0, 26.0, 23.0, 2.0, color);
            draw_rectangle_lines(x - 6.0, y - 14.0, 12.0, 5.0, 2.0, color);
            draw_line(x - 14.0, y - 2.0, x + 14.0, y - 2.0, 3.0, color);
            for offset in [-6.0, 6.0] {
                draw_line(x + offset, y - 5.0, x + offset, y + 11.0, 3.0, color);
            }
        }
    }
}
