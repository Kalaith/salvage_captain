//! Shared telemetry, operation, and footer chrome for every game screen.

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
        port_panel::draw_header(ctx, actions);
        return;
    }
    let (title, status, nav_label, nav_action) = match screen {
        GameState::SiteSelection => (
            "SITES // SC-07",
            "ROUTE PLANNING",
            "PORT",
            UiAction::GoToPort,
        ),
        GameState::Travel => (
            "TRANSIT // SC-07",
            "ROUTE ACTIVE",
            "PORT",
            UiAction::GoToPort,
        ),
        GameState::ReturnTravel => ("RETURN // SC-07", "DOCKING RUN", "PORT", UiAction::GoToPort),
        GameState::SalvageWorkspace => (
            "WORKSPACE // SC-07",
            "SALVAGE ACTIVE",
            "LOG",
            UiAction::ToggleWorkspaceLog,
        ),
        GameState::CargoInventory => (
            "INVENTORY // SC-07",
            "SECURED CARGO",
            "",
            UiAction::ReturnToWorkspace,
        ),
        GameState::Results => (
            "DEBRIEF // SC-07",
            "RETURN RESOLVED",
            "PORT",
            UiAction::GoToPort,
        ),
        _ => (
            "SALVAGE // SC-07",
            "SYSTEM READY",
            "PORT",
            UiAction::GoToPort,
        ),
    };
    draw_standard_header(
        ctx,
        actions,
        title,
        status,
        HeaderNavigation {
            label: nav_label,
            action: nav_action,
            enabled: true,
            pause_enabled: true,
        },
    );
}

pub(crate) struct HeaderNavigation<'a> {
    pub label: &'a str,
    pub action: UiAction,
    pub enabled: bool,
    pub pause_enabled: bool,
}

pub(crate) fn draw_menu_header() {
    panel(
        Rect::new(0.0, 0.0, LOGICAL_WIDTH, 68.0),
        visual_theme::panel(),
    );
    visual_theme::body(
        "SALVAGE CAPTAIN",
        Rect::new(28.0, 10.0, 360.0, 27.0),
        23.0,
        visual_theme::text(),
    );
    visual_theme::body(
        "COMMAND DECK",
        Rect::new(42.0, 38.0, 240.0, 23.0),
        17.0,
        visual_theme::safe(),
    );
}

/// The standard 68 px telemetry bar; the port owns its compact variant.
pub(crate) fn draw_standard_header(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    title: &str,
    status: &str,
    nav: HeaderNavigation<'_>,
) {
    panel(
        Rect::new(0.0, 0.0, LOGICAL_WIDTH, 68.0),
        visual_theme::panel(),
    );
    visual_theme::body(
        title,
        Rect::new(28.0, 10.0, 210.0, 27.0),
        23.0,
        visual_theme::text(),
    );
    draw_circle(32.0, 48.0, 3.0, visual_theme::safe());
    visual_theme::body(
        status,
        Rect::new(42.0, 38.0, 185.0, 23.0),
        17.0,
        visual_theme::safe(),
    );
    draw_resources(ctx);
    if !nav.label.is_empty()
        && button(
            ctx,
            Rect::new(1110.0, 12.0, 74.0, 44.0),
            nav.label,
            nav.enabled,
            ButtonTone::Secondary,
        )
    {
        actions.push(nav.action.clone());
    }
    if nav.pause_enabled {
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
}

fn draw_resources(ctx: &UiContext<'_>) {
    let resources = [
        (
            246.0,
            102.0,
            format!("CR {}", grouped_credits(ctx.session.economy.credits)),
            visual_theme::amber(),
        ),
        (
            370.0,
            116.0,
            format!(
                "FUEL {}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            506.0,
            106.0,
            format!(
                "HULL {}/{}",
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
            120.0,
            format!(
                "HOLD {}/{}",
                ctx.session.internal_cargo_count(ctx.data, None),
                ctx.session.internal_cargo_capacity()
            ),
            visual_theme::text(),
        ),
        (
            742.0,
            120.0,
            format!(
                "CLAMP {}/{}",
                ctx.session.external_cargo_count(ctx.data, None),
                ctx.session.external_capacity(ctx.data)
            ),
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

pub(crate) fn grouped_credits(value: i64) -> String {
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
    visual_theme::body(
        &text,
        Rect::new(rect.x + 18.0, rect.y + 5.0, rect.w - 24.0, rect.h - 4.0),
        16.0,
        visual_theme::text(),
    );
}

fn draw_salvage_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel(
        Rect::new(0.0, 0.0, LOGICAL_WIDTH, 68.0),
        visual_theme::panel(),
    );
    visual_theme::body(
        "WORKSPACE // SC-07",
        Rect::new(28.0, 10.0, 210.0, 27.0),
        23.0,
        visual_theme::text(),
    );
    draw_circle(32.0, 48.0, 3.0, visual_theme::safe());
    visual_theme::body(
        "SALVAGE ACTIVE",
        Rect::new(42.0, 38.0, 185.0, 23.0),
        17.0,
        visual_theme::safe(),
    );
    draw_salvage_resources(ctx);
    draw_line(840.0, 13.0, 840.0, 55.0, 1.0, visual_theme::structure());
    if button(
        ctx,
        Rect::new(1110.0, 12.0, 74.0, 44.0),
        "LOG",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleWorkspaceLog);
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

fn draw_salvage_resources(ctx: &UiContext<'_>) {
    let (power, power_capacity) = ctx.session.workspace_energy().unwrap_or((0, 0));
    let cargo = ctx.session.internal_cargo_count(ctx.data, None);
    let cargo_capacity = ctx.session.internal_cargo_capacity();
    let clamps = ctx.session.external_cargo_count(ctx.data, None);
    let clamp_capacity = ctx.session.external_capacity(ctx.data);
    let resources = [
        (
            246.0,
            130.0,
            format!(
                "FUEL {}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            390.0,
            130.0,
            format!(
                "HULL {}/{}",
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
            534.0,
            160.0,
            format!("POWER {power}/{power_capacity}"),
            if power <= 1 {
                visual_theme::warning()
            } else {
                visual_theme::text()
            },
        ),
        (
            706.0,
            120.0,
            format!("HOLD {cargo}/{cargo_capacity}"),
            visual_theme::text(),
        ),
        (
            838.0,
            120.0,
            format!("CLAMP {clamps}/{clamp_capacity}"),
            visual_theme::text_dim(),
        ),
    ];
    for (x, width, label, color) in resources {
        draw_line(
            x - 12.0,
            13.0,
            x - 12.0,
            55.0,
            1.0,
            visual_theme::structure(),
        );
        visual_theme::body(&label, Rect::new(x, 23.0, width, 28.0), 17.0, color);
    }
}
