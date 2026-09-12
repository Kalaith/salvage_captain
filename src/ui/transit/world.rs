//! Parallax space layers and a stationary workboat approaching its destination.

use super::*;
mod destination;

pub(super) fn draw_world(
    ctx: &UiContext<'_>,
    leg: FlightLeg,
    frame: layout::TransitLayout,
    progress: f32,
    theme: &str,
) {
    draw_rectangle(
        0.0,
        84.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 84.0,
        visual_theme::space(),
    );
    draw_distant_space(progress, ctx.reduced_motion);
    if leg == FlightLeg::Homebound {
        destination::draw_yard(frame.destination);
    } else {
        destination::draw_wreck(frame.destination, theme);
    }
    let elapsed = if ctx.reduced_motion {
        0.0
    } else if leg == FlightLeg::Homebound {
        ctx.return_elapsed
    } else {
        ctx.travel_elapsed
    };
    if !ctx.reduced_motion && progress < 1.0 {
        draw_wake(frame.ship, progress);
    }
    ship_visual::draw_flight_ship(frame.ship, ctx.session, ctx.data, elapsed);
    draw_foreground(progress, ctx.reduced_motion);
}

fn draw_distant_space(progress: f32, reduced_motion: bool) {
    // Broad, dim masses put depth behind the machinery without competing with it.
    draw_circle(934.0, 315.0, 192.0, Color::new(0.023, 0.062, 0.084, 1.0));
    draw_circle(955.0, 305.0, 174.0, Color::new(0.031, 0.083, 0.108, 1.0));
    draw_circle(998.0, 291.0, 173.0, visual_theme::space());
    for layer in 0..3 {
        let offset = layout::scenery_offset(progress, layer, reduced_motion);
        for index in 0..36 {
            let x = ((index * 127 + layer * 59) as f32 - offset).rem_euclid(1380.0) - 50.0;
            let y = 94.0 + ((index * 83 + layer * 113) % 460) as f32;
            let alpha = 0.17 + layer as f32 * 0.12;
            let color = visual_theme::with_alpha(visual_theme::text_dim(), alpha);
            let trail = if reduced_motion || progress >= 1.0 {
                0.0
            } else {
                layer as f32 * 9.0 * (1.0 - progress)
            };
            draw_line(x, y, x + trail + 1.0, y, 1.0 + layer as f32 * 0.35, color);
        }
    }
    let drift = layout::scenery_offset(progress, 0, reduced_motion);
    for index in 0..5 {
        let x = (510.0 + index as f32 * 189.0 - drift).rem_euclid(1440.0);
        let y = 230.0 + (index % 3) as f32 * 87.0;
        draw_triangle(
            vec2(x, y),
            vec2(x + 46.0, y - 17.0),
            vec2(x + 82.0, y + 8.0),
            Color::new(0.055, 0.085, 0.106, 1.0),
        );
    }
}

fn draw_wake(ship: Rect, progress: f32) {
    let length = 130.0 * (1.0 - progress).max(0.10);
    for index in 0..3 {
        let y = ship.y + ship.h * 0.72 * (0.34 + index as f32 * 0.10);
        let x = ship.x + 8.0;
        draw_triangle(
            vec2(x, y - 5.0),
            vec2(x - length, y),
            vec2(x, y + 5.0),
            visual_theme::with_alpha(visual_theme::cyan(), 0.12),
        );
        draw_line(
            x,
            y,
            x - length * 0.66,
            y,
            2.0,
            visual_theme::with_alpha(visual_theme::cyan(), 0.40),
        );
    }
}

fn draw_foreground(progress: f32, reduced_motion: bool) {
    let offset = layout::scenery_offset(progress, 2, reduced_motion);
    for index in 0..4 {
        let x = (index as f32 * 420.0 + 70.0 - offset).rem_euclid(1660.0) - 180.0;
        let y = 542.0 - (index % 2) as f32 * 24.0;
        draw_triangle(
            vec2(x, y),
            vec2(x + 76.0, y - 16.0),
            vec2(x + 102.0, y + 14.0),
            visual_theme::structure_dark(),
        );
        draw_line(
            x,
            y,
            x + 76.0,
            y - 16.0,
            2.0,
            visual_theme::with_alpha(visual_theme::structure_light(), 0.42),
        );
    }
}
