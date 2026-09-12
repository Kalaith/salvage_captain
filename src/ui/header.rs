//! Shared resource, operation, and footer chrome for every game screen.

use super::*;

pub(super) fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let screen = active_screen(ctx);
    if matches!(screen, GameState::Travel | GameState::ReturnTravel) {
        transit::draw_header(ctx, actions);
        return;
    }
    if screen == GameState::SiteSelection {
        site_cards::draw_header(ctx, actions);
        return;
    }
    if screen == GameState::SalvageWorkspace {
        draw_salvage_header(ctx, actions);
        return;
    }
    if screen == GameState::Port {
        draw_port_header(ctx, actions);
        return;
    }
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        84.0,
        visual_theme::with_alpha(visual_theme::panel(), 0.94),
    );
    draw_rectangle(0.0, 0.0, 7.0, 84.0, visual_theme::amber());
    draw_line(
        24.0,
        83.0,
        LOGICAL_WIDTH - 24.0,
        83.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.8),
    );
    draw_text(
        screen_title(ctx.state, ctx.resume_state),
        32.0,
        48.0,
        18.0,
        visual_theme::text(),
    );
    draw_resource_value(
        330.0,
        "CREDITS",
        &format!("¢{}", ctx.session.economy.credits),
        visual_theme::safe(),
    );
    draw_resource_value(
        458.0,
        "FUEL",
        &format!(
            "{}/{}",
            ctx.session.economy.fuel,
            ctx.session.max_fuel(ctx.data)
        ),
        visual_theme::cyan(),
    );
    draw_resource_value(
        584.0,
        "HULL",
        &format!(
            "{}/{}",
            ctx.session.hull,
            ctx.session.max_hull_with_modules(ctx.data)
        ),
        if ctx.session.hull <= 3 {
            visual_theme::warning()
        } else {
            visual_theme::amber()
        },
    );
    draw_resource_value(
        712.0,
        "ALLOY",
        &ctx.session.economy.alloy.to_string(),
        visual_theme::text_dim(),
    );
    draw_resource_value(
        818.0,
        "ELEC",
        &ctx.session.economy.electronics.to_string(),
        visual_theme::text_dim(),
    );
    let port_enabled = matches!(screen, GameState::Port | GameState::SiteSelection);
    if button(
        ctx,
        Rect::new(1000.0, 20.0, 108.0, 46.0),
        "PORT",
        port_enabled,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::GoToPort);
    }
    if button(
        ctx,
        Rect::new(1120.0, 20.0, 136.0, 46.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
}

fn draw_port_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    panel(
        Rect::new(0.0, 0.0, LOGICAL_WIDTH, port_panel::HEADER_HEIGHT),
        visual_theme::panel(),
    );
    visual_theme::body(
        &copy.title_short,
        Rect::new(28.0, 16.0, 288.0, 30.0),
        24.0,
        visual_theme::text(),
    );
    for (index, value) in [
        format!("{} {}", ctx.session.economy.credits, copy.credits),
        format!("{} {}", copy.fuel, ctx.session.economy.fuel),
        format!("{} {}", ctx.data.selection_ui.hull, ctx.session.hull),
        format!("{} {}", copy.alloy, ctx.session.economy.alloy),
        format!("{} {}", copy.electronics, ctx.session.economy.electronics),
    ]
    .iter()
    .enumerate()
    {
        visual_theme::body(
            value,
            Rect::new(332.0 + index as f32 * 132.0, 19.0, 126.0, 28.0),
            21.0,
            visual_theme::text(),
        );
    }
    let log_label =
        voyage_archive::archive_button_label(ctx.session.voyage_log.len(), ctx.voyage_archive_open);
    for (rect, label, action) in [
        (
            Rect::new(1000.0, 6.0, 108.0, 44.0),
            &log_label,
            UiAction::ToggleVoyageArchive,
        ),
        (
            Rect::new(1120.0, 6.0, 136.0, 44.0),
            &copy.pause,
            UiAction::TogglePause,
        ),
    ] {
        if button(ctx, rect, label, true, ButtonTone::Secondary) {
            actions.push(action);
        }
    }
}

fn draw_resource_value(x: f32, label: &str, value: &str, color: Color) {
    draw_resource_value_at(x, label, value, color, 20.0, 18);
}

fn draw_resource_value_at(
    x: f32,
    label: &str,
    value: &str,
    color: Color,
    top: f32,
    value_size: u16,
) {
    draw_text(label, x, top + 9.0, 9.0, visual_theme::text_dim());
    draw_text(value, x, top + 32.0, value_size as f32, color);
    draw_line(
        x - 20.0,
        top,
        x - 20.0,
        top + 38.0,
        1.0,
        visual_theme::with_alpha(visual_theme::structure_light(), 0.3),
    );
}

fn active_screen(ctx: &UiContext<'_>) -> GameState {
    if ctx.state == GameState::Pause {
        ctx.resume_state
    } else {
        ctx.state
    }
}

pub(super) fn draw_footer(ctx: &UiContext<'_>) {
    if ctx.message.is_empty() {
        return;
    }
    let text = clipped(ctx.message, 92);
    let measured = measure_text(&text, None, 16, 1.0).width;
    let rect = Rect::new(
        ((LOGICAL_WIDTH - measured - 36.0) * 0.5).max(24.0),
        LOGICAL_HEIGHT - 42.0,
        measured + 36.0,
        28.0,
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        visual_theme::with_alpha(visual_theme::panel(), 0.92),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        visual_theme::cyan_dim(),
    );
    draw_text(&text, rect.x + 18.0, rect.y + 19.0, 16.0, dark::TEXT);
}

fn draw_salvage_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(0.0, 0.0, LOGICAL_WIDTH, 84.0, visual_theme::panel());
    draw_text("SALVAGE", 28.0, 48.0, 22.0, visual_theme::text());
    let resources = [
        format!("FUEL {}", ctx.session.economy.fuel),
        format!("HULL {}", ctx.session.hull),
        format!(
            "CARGO {}/{}",
            ctx.session.internal_cargo_count(ctx.data, None),
            ctx.session.internal_cargo_capacity()
        ),
        ctx.session
            .workspace_energy()
            .map_or_else(String::new, |(remaining, capacity)| {
                format!("POWER {remaining}/{capacity}")
            }),
    ];
    for (index, value) in resources.iter().enumerate() {
        visual_theme::body(
            value,
            Rect::new(240.0 + index as f32 * 180.0, 26.0, 164.0, 32.0),
            24.0,
            if (index == 1 && ctx.session.hull <= 3)
                || (index == 3
                    && ctx
                        .session
                        .workspace_energy()
                        .is_some_and(|(remaining, _)| remaining <= 2))
            {
                visual_theme::warning()
            } else {
                visual_theme::text()
            },
        );
    }
    if button(
        ctx,
        Rect::new(1000.0, 20.0, 108.0, 46.0),
        "LOG",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleWorkspaceLog);
    }
    if button(
        ctx,
        Rect::new(1120.0, 20.0, 136.0, 46.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
}
