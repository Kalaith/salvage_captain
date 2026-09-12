//! Shared side-on wreck illustrations for destination and briefing scenes.

use super::*;

pub fn draw_wreck(rect: Rect, theme: &str) {
    let p = |x: f32, y: f32| vec2(rect.x + rect.w * x, rect.y + rect.h * y);
    let mut outline = [
        p(0.0, 0.34),
        p(0.08, 0.18),
        p(0.28, 0.10),
        p(0.88, 0.10),
        p(1.0, 0.33),
        p(0.93, 0.82),
        p(0.23, 0.88),
        p(0.03, 0.70),
        p(0.12, 0.49),
    ];
    match theme {
        "military" => {
            outline[3] = p(0.78, 0.16);
            outline[4] = p(1.0, 0.52);
            outline[5] = p(0.80, 0.76);
            outline[6] = p(0.20, 0.80);
        }
        "research" => {
            outline[1] = p(0.08, 0.08);
            outline[2] = p(0.36, 0.04);
            outline[3] = p(0.78, 0.22);
            outline[4] = p(0.93, 0.40);
        }
        _ => {}
    }
    let shadow: Vec<_> = outline
        .iter()
        .map(|point| *point + vec2(6.0, 12.0))
        .collect();
    // The underside is visible below the outer hull.
    polygon(&shadow, visual_theme::structure_dark());
    polygon(&outline, visual_theme::structure());
    draw_line(
        outline[1].x,
        outline[1].y,
        outline[2].x,
        outline[2].y,
        3.0,
        visual_theme::structure_light(),
    );
    draw_line(
        outline[2].x,
        outline[2].y,
        outline[3].x,
        outline[3].y,
        3.0,
        visual_theme::structure_light(),
    );
    draw_wreck_bays(rect, theme);
    let tear = [p(0.0, 0.40), p(0.22, 0.49), p(0.05, 0.61), p(0.0, 0.72)];
    polygon(&tear, visual_theme::space());
    let edge_start = p(0.02, 0.38);
    let edge_end = p(0.22, 0.49);
    draw_line(
        edge_start.x,
        edge_start.y,
        edge_end.x,
        edge_end.y,
        4.0,
        visual_theme::structure_light(),
    );
    // Plating overlaps the lower edges of the recessed bays.
    for index in 0..3 {
        let x = 0.18 + index as f32 * 0.24;
        polygon(
            &[p(x, 0.80), p(x + 0.16, 0.67), p(x + 0.22, 0.80)],
            visual_theme::structure_dark(),
        );
    }
}

fn polygon(points: &[Vec2], color: Color) {
    for index in 1..points.len() - 1 {
        draw_triangle(points[0], points[index], points[index + 1], color);
    }
}

fn draw_wreck_bays(rect: Rect, theme: &str) {
    let accent = visual_theme::site_accent(theme);
    let p = |x: f32, y: f32| vec2(rect.x + rect.w * x, rect.y + rect.h * y);
    for index in 0..4 {
        let x = 0.15 + index as f32 * 0.19;
        let bay = Rect::new(p(x, 0.26).x, p(x, 0.26).y, rect.w * 0.164, rect.h * 0.46);
        draw_rectangle(bay.x + 5.0, bay.y + 8.0, bay.w, bay.h, BLACK);
        draw_rectangle(bay.x, bay.y, bay.w, bay.h, visual_theme::space());
        draw_rectangle(
            bay.x + 8.0,
            bay.y + 18.0,
            bay.w - 16.0,
            bay.h - 30.0,
            visual_theme::structure_dark(),
        );
        draw_line(
            bay.x,
            bay.bottom(),
            bay.right(),
            bay.bottom(),
            3.0,
            visual_theme::structure_light(),
        );
        draw_rectangle(
            bay.x + bay.w * 0.18,
            bay.y - rect.h * 0.05,
            bay.w * 0.56,
            3.0,
            visual_theme::with_alpha(accent, 0.7),
        );
        match theme {
            "military" => {
                draw_line(
                    bay.x,
                    bay.y,
                    bay.right(),
                    bay.bottom(),
                    6.0,
                    visual_theme::structure(),
                );
                draw_circle(
                    bay.center().x,
                    bay.center().y,
                    rect.h * 0.055,
                    visual_theme::with_alpha(accent, 0.5),
                );
            }
            "research" => {
                draw_circle_lines(
                    bay.center().x,
                    bay.center().y,
                    bay.w * 0.28,
                    3.0,
                    visual_theme::cyan_dim(),
                );
                draw_circle(bay.center().x, bay.center().y, 4.0, visual_theme::cyan());
            }
            _ => {
                for crate_index in 0..2 {
                    let crate_x = bay.x + 14.0 + crate_index as f32 * bay.w * 0.38;
                    draw_rectangle(
                        crate_x,
                        bay.bottom() - rect.h * 0.17,
                        bay.w * 0.28,
                        rect.h * 0.12,
                        visual_theme::with_alpha(accent, 0.35),
                    );
                }
            }
        }
    }
}
