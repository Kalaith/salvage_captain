//! Shared resource, operation, and footer chrome for every game screen.

use super::*;

pub(super) fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let screen = active_screen(ctx);
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
    if matches!(screen, GameState::Travel | GameState::SalvageWorkspace) {
        operation_header::draw_operation_badges(ctx);
        let action_rect = Rect::new(1000.0, 20.0, 108.0, 46.0);
        let action_label = if screen == GameState::Travel {
            if ctx.travel_elapsed >= travel::TRAVEL_DURATION_SECONDS {
                "CONTINUE".to_owned()
            } else {
                "ARRIVE".to_owned()
            }
        } else {
            operation_header::log_button_label(
                ctx.session
                    .workspace_log()
                    .map_or(0, |entries| entries.len()),
            )
        };
        let action_enabled = matches!(screen, GameState::Travel | GameState::SalvageWorkspace);
        if button(
            ctx,
            action_rect,
            &action_label,
            action_enabled,
            ButtonTone::Positive,
        ) {
            actions.push(if screen == GameState::Travel {
                UiAction::ContinueTravel
            } else {
                UiAction::ToggleWorkspaceLog
            });
        }
    } else {
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
    let width = ctx.viewport_width;
    let height = port_panel::HEADER_HEIGHT;
    draw_rectangle(
        0.0,
        0.0,
        width,
        height,
        visual_theme::with_alpha(visual_theme::panel(), 0.97),
    );
    draw_rectangle(0.0, 0.0, 6.0, height, visual_theme::amber());
    draw_line(
        22.0,
        height - 1.0,
        width - 22.0,
        height - 1.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.85),
    );
    draw_text(
        screen_title(ctx.state, ctx.resume_state),
        26.0,
        35.0,
        16.0,
        visual_theme::text(),
    );
    draw_text(
        &clipped(
            &format!(
                "SC-07  //  SHIPYARD ONLINE  //  CREW {}  //  RANK {}",
                ctx.session.crew_role().short_label(),
                ctx.session.career.rank_code()
            ),
            42,
        ),
        26.0,
        49.0,
        9.0,
        visual_theme::text_dim(),
    );

    let pause_width = 74.0;
    let pause_x = (width - pause_width - 18.0).max(230.0);
    let log_width = 74.0;
    let log_x = pause_x - log_width - 10.0;
    let show_log = width >= 760.0;
    let resources_left = 238.0;
    let resources_right = if show_log {
        log_x - 16.0
    } else {
        pause_x - 16.0
    };
    let cell_width = ((resources_right - resources_left) / 5.0).max(74.0);
    let resources = [
        (
            "CREDITS",
            format!("¢{}", ctx.session.economy.credits),
            visual_theme::safe(),
        ),
        (
            "FUEL",
            format!(
                "{}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            "HULL",
            format!(
                "{}/{}",
                ctx.session.hull,
                ctx.session.max_hull_with_modules(ctx.data)
            ),
            if ctx.session.hull <= 3 {
                visual_theme::warning()
            } else {
                visual_theme::amber()
            },
        ),
        (
            "ALLOY",
            ctx.session.economy.alloy.to_string(),
            visual_theme::text_dim(),
        ),
        (
            "ELEC",
            ctx.session.economy.electronics.to_string(),
            visual_theme::text_dim(),
        ),
    ];
    for (index, (label, value, color)) in resources.into_iter().enumerate() {
        draw_resource_value_at(
            resources_left + index as f32 * cell_width,
            label,
            &value,
            color,
            16.0,
            16,
        );
    }
    if show_log
        && button(
            ctx,
            Rect::new(log_x, 13.0, log_width, 32.0),
            &voyage_archive::archive_button_label(
                ctx.session.voyage_log.len(),
                ctx.voyage_archive_open,
            ),
            true,
            ButtonTone::Secondary,
        )
    {
        actions.push(UiAction::ToggleVoyageArchive);
    }
    if button(
        ctx,
        Rect::new(pause_x, 13.0, pause_width, 32.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
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
    let footer_message = if active_screen(ctx) == GameState::Travel && ctx.travel_elapsed >= 4.0 {
        "Arrival locked. Tap CONTINUE to enter the wreck workspace."
    } else {
        ctx.message
    };
    let text = clipped(footer_message, 92);
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
