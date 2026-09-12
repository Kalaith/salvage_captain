//! Transit route, wake, and wreck-marker visuals.

use super::*;

pub(super) fn draw_transit_wake(ship: Rect, elapsed: f32, theme: &str, phase: TravelPhase) {
    let count = wake_segment_count(phase);
    if count == 0 {
        return;
    }
    let accent = visual_theme::site_accent(theme);
    for index in 0..count {
        let offset = index as f32 * 18.0 + (elapsed * 3.0 + index as f32).sin() * 3.0;
        let y = ship.y + ship.h * (0.38 + index as f32 * 0.08);
        draw_line(
            ship.x - 10.0 - offset,
            y,
            ship.x - 34.0 - offset,
            y + (index as f32 - 2.0) * 2.0,
            2.0,
            visual_theme::with_alpha(accent, 0.65 - index as f32 * 0.08),
        );
    }
}

pub(super) fn draw_transit_route(progress: f32, elapsed: f32, theme: &str, phase: TravelPhase) {
    let start_x = 92.0;
    let end_x = 880.0;
    let route_y = 390.0;
    let accent = visual_theme::site_accent(theme);
    draw_line(
        start_x,
        route_y - 18.0,
        end_x,
        route_y - 18.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.32),
    );
    draw_line(
        start_x,
        route_y + 18.0,
        end_x,
        route_y + 18.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.32),
    );
    draw_line(
        start_x,
        route_y,
        end_x,
        route_y,
        2.0,
        visual_theme::cyan_dim(),
    );
    for index in 0..10 {
        let marker_progress = index as f32 / 9.0;
        let x = 112.0 + index as f32 * 78.0;
        let reached = marker_progress <= progress;
        draw_circle(
            x,
            route_y,
            if reached { 4.0 } else { 2.0 },
            if reached {
                accent
            } else {
                visual_theme::structure_light()
            },
        );
    }
    if phase != TravelPhase::Docked {
        let signal_progress = (elapsed * 0.32).fract();
        let signal_x = start_x + (end_x - start_x) * signal_progress;
        draw_circle(
            signal_x,
            route_y,
            5.0,
            visual_theme::with_alpha(visual_theme::text(), 0.85),
        );
        draw_circle_lines(
            signal_x,
            route_y,
            12.0 + (elapsed * 5.0).sin().abs() * 5.0,
            2.0,
            visual_theme::with_alpha(accent, 0.72),
        );
    }
    if matches!(phase, TravelPhase::FinalApproach | TravelPhase::Docked) {
        draw_text(
            "DOCKING CORRIDOR",
            716.0,
            route_y - 28.0,
            10.0,
            travel_phase_color(phase),
        );
    }
}

pub(super) fn draw_wreck_marker(
    x: f32,
    y: f32,
    theme: &str,
    progress: f32,
    elapsed: f32,
    phase: TravelPhase,
) {
    let accent = visual_theme::site_accent(theme);
    let center = vec2(x + 60.0, y + 38.0);
    if matches!(phase, TravelPhase::FinalApproach | TravelPhase::Docked) {
        let pulse = if phase == TravelPhase::Docked {
            42.0
        } else {
            30.0 + (elapsed * 3.0).sin().abs() * 10.0
        };
        draw_circle_lines(
            center.x,
            center.y,
            pulse,
            2.0,
            visual_theme::with_alpha(accent, 0.7),
        );
        draw_line(
            center.x - pulse - 12.0,
            center.y,
            center.x - pulse,
            center.y,
            2.0,
            accent,
        );
        draw_line(
            center.x + pulse,
            center.y,
            center.x + pulse + 12.0,
            center.y,
            2.0,
            accent,
        );
    }
    draw_rectangle(x, y, 120.0, 76.0, visual_theme::structure_dark());
    draw_rectangle_lines(x, y, 120.0, 76.0, 3.0, accent);
    draw_line(x + 18.0, y + 20.0, x + 95.0, y + 58.0, 3.0, accent);
    draw_line(x + 82.0, y + 16.0, x + 18.0, y + 62.0, 3.0, accent);
    draw_profile_signature(x, y, theme, elapsed);
    draw_text(
        if progress >= 1.0 {
            "DOCKED"
        } else if phase == TravelPhase::FinalApproach {
            "APPROACH"
        } else {
            "TARGET"
        },
        x + 30.0,
        y + 100.0,
        13.0,
        visual_theme::text_dim(),
    );
}

pub(super) fn draw_profile_signature(x: f32, y: f32, theme: &str, elapsed: f32) {
    match theme {
        "military" => {
            draw_line(
                x + 20.0,
                y + 18.0,
                x + 48.0,
                y + 58.0,
                4.0,
                visual_theme::warning(),
            );
            draw_line(
                x + 100.0,
                y + 18.0,
                x + 72.0,
                y + 58.0,
                4.0,
                visual_theme::warning(),
            );
            draw_circle(x + 60.0, y + 38.0, 5.0, visual_theme::warning());
        }
        "research" => {
            draw_circle_lines(
                x + 60.0,
                y + 38.0,
                19.0 + (elapsed * 2.0).sin().abs() * 5.0,
                2.0,
                visual_theme::cyan(),
            );
            draw_circle(x + 60.0, y + 38.0, 4.0, visual_theme::cyan());
        }
        _ => {
            for index in 0..3 {
                draw_rectangle(
                    x + 18.0 + index as f32 * 27.0,
                    y + 51.0,
                    18.0,
                    10.0,
                    visual_theme::with_alpha(visual_theme::amber(), 0.75),
                );
            }
        }
    }
}
