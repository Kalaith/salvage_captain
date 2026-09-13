//! Compact edge controls with larger release targets and contextual hints.

use super::*;

/// Keep the toolkit's button semantics while rendering only the inset face.
pub(super) fn edge_button(ctx: &UiContext<'_>, hit: Rect, selected: bool, primary: bool) -> bool {
    let face = Rect::new(hit.x, hit.y + 4.0, hit.w, hit.h - 8.0);
    let hovered = ctx.interaction_enabled && ctx.pointer.hovering_over(hit);
    let mut style = visual_theme::button_style(
        if primary {
            ButtonTone::Primary
        } else {
            ButtonTone::Secondary
        },
        true,
    );
    if !primary {
        style.normal = visual_theme::with_alpha(visual_theme::panel(), 0.78);
        style.border = if selected {
            visual_theme::cyan_dim()
        } else {
            Color::new(0.0, 0.0, 0.0, 0.0)
        };
    }
    if hovered || selected {
        style.normal = style.hovered;
    }
    let activated = button_rect_enabled_styled_ex_at(
        face,
        "",
        ctx.interaction_enabled,
        &style,
        TextStyle::new(18.0, style.text_color),
        ButtonTrigger::Release,
        ctx.pointer.position,
    );
    ctx.interaction_enabled && (activated || ctx.pointer.released_on(hit))
}

pub(super) fn draw_dock(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    for (tab, label, icon) in [
        (PortTab::Service, &copy.service, DockIcon::Service),
        (PortTab::Equipment, &copy.equipment, DockIcon::Equipment),
        (PortTab::Crew, &copy.crew, DockIcon::Crew),
    ] {
        let rect = tab_rect(tab);
        if edge_button(ctx, rect, ctx.port_tab == tab, false) {
            actions.push(UiAction::SelectPortTab(tab));
        }
        draw_icon(
            vec2(rect.x + 21.0, rect.y + 22.0),
            icon,
            visual_theme::text_dim(),
        );
        visual_theme::body(
            label,
            Rect::new(rect.x + 40.0, rect.y + 13.0, rect.w - 44.0, 23.0),
            18.0,
            visual_theme::text(),
        );
    }
    let depart = departure_rect();
    if edge_button(ctx, depart, false, true) {
        actions.push(UiAction::GoToSites);
    }
    draw_icon(
        vec2(depart.x + 22.0, depart.y + 22.0),
        DockIcon::Depart,
        visual_theme::safe(),
    );
    visual_theme::body(
        &copy.depart,
        Rect::new(depart.x + 43.0, depart.y + 12.0, depart.w - 50.0, 24.0),
        20.0,
        visual_theme::safe(),
    );
}

pub(super) fn draw_context_hint(ctx: &UiContext<'_>) {
    let copy = &ctx.data.port_ui;
    let service_hint = format!("{}\n{}", copy.service_hint, market_ticker(ctx));
    let hover = [
        (tab_rect(PortTab::Service), service_hint.as_str()),
        (tab_rect(PortTab::Equipment), copy.equipment_hint.as_str()),
        (tab_rect(PortTab::Crew), copy.dock_crew_hint.as_str()),
        (departure_rect(), copy.depart_hint.as_str()),
        (cargo_rect(), copy.hold.as_str()),
        (status::TITLE_RECT, copy.checkpoint.as_str()),
        (status::MENU_RECT, copy.pause.as_str()),
    ]
    .into_iter()
    .find(|(hit, _)| ctx.interaction_enabled && ctx.pointer.hovering_over(*hit));
    if let Some((hit, hint)) = hover {
        let anchor = vec2(
            hit.x.min(LOGICAL_WIDTH - 340.0),
            if hit.y < HEADER_HEIGHT {
                HEADER_HEIGHT - 8.0
            } else {
                hit.y - hint.lines().count() as f32 * 19.0 - 36.0
            },
        );
        macroquad_toolkit::ui::draw_tooltip_styled(
            hint,
            anchor,
            &macroquad_toolkit::ui::TooltipStyle {
                background: visual_theme::panel(),
                border: visual_theme::cyan_dim(),
                text: visual_theme::text_dim(),
                padding: 8.0,
                max_width: 320.0,
                font_size: 16.0,
                line_gap: 3.0,
            },
            None,
        );
    } else if (ctx.port_tab != PortTab::Hangar || ctx.port_hold_expanded)
        && !ctx.message.is_empty()
        && ctx.message != crate::game::prompts::state_prompt(GameState::Port)
    {
        // Transaction feedback is relevant while managing the ship, never a permanent footer.
        visual_theme::body(
            ctx.message,
            Rect::new(28.0, 616.0, 1224.0, 36.0),
            16.0,
            visual_theme::text(),
        );
    }
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
            draw_line(x - 5.2, y - 5.2, x + 7.15, y + 7.15, 3.9, color);
            draw_circle(x - 5.2, y - 5.2, 5.2, color);
            draw_triangle(
                vec2(x - 11.05, y - 10.4),
                vec2(x - 1.95, y - 10.4),
                vec2(x - 5.2, y - 3.25),
                visual_theme::panel(),
            );
            draw_circle(x + 6.5, y + 6.5, 1.3, visual_theme::panel());
        }
        DockIcon::Equipment => {
            for index in 0..8 {
                let angle = index as f32 * std::f32::consts::TAU / 8.0;
                let axis = vec2(angle.cos(), angle.sin());
                let start = center + axis * 5.2;
                let end = center + axis * 9.75;
                draw_line(start.x, start.y, end.x, end.y, 3.9, color);
            }
            draw_circle(x, y, 7.15, color);
            draw_circle(x, y, 3.25, visual_theme::panel());
        }
        DockIcon::Crew => {
            draw_circle(x - 2.6, y - 4.55, 3.9, color);
            draw_circle(
                x + 5.2,
                y - 3.25,
                3.25,
                visual_theme::with_alpha(color, 0.65),
            );
            draw_rectangle(x - 8.45, y + 1.95, 12.35, 6.5, color);
            draw_rectangle(
                x + 5.2,
                y + 1.95,
                5.2,
                6.5,
                visual_theme::with_alpha(color, 0.65),
            );
        }
        DockIcon::Depart => {
            draw_triangle(
                vec2(x - 9.1, y - 3.25),
                vec2(x + 9.1, y - 9.1),
                vec2(x + 3.25, y + 9.1),
                color,
            );
            draw_line(x - 7.8, y + 9.1, x + 5.2, y - 5.2, 2.6, color);
            draw_line(
                x - 2.6,
                y + 1.95,
                x + 5.2,
                y - 5.2,
                1.3,
                visual_theme::panel(),
            );
        }
        DockIcon::Cargo => {
            draw_rectangle_lines(x - 8.45, y - 5.85, 16.9, 14.95, 1.3, color);
            draw_rectangle_lines(x - 3.9, y - 9.1, 7.8, 3.25, 1.3, color);
            draw_line(x - 9.1, y - 1.3, x + 9.1, y - 1.3, 1.95, color);
            for offset in [-3.9, 3.9] {
                draw_line(x + offset, y - 3.25, x + offset, y + 7.15, 1.95, color);
            }
        }
    }
}
