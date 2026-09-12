//! Side-on wreck and industrial yard silhouettes with shallow depth.

use super::*;

pub(super) fn draw_wreck(rect: Rect, theme: &str) {
    let p = |x: f32, y: f32| vec2(rect.x + rect.w * x, rect.y + rect.h * y);
    let outline = [
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
    let shadow: Vec<_> = outline
        .iter()
        .map(|point| *point + vec2(6.0, 12.0))
        .collect();
    // The underside is visible below the outer hull.
    polygon(&shadow, visual_theme::structure_dark());
    polygon(&outline, visual_theme::structure());
    draw_line(
        p(0.08, 0.18).x,
        p(0.08, 0.18).y,
        p(0.28, 0.10).x,
        p(0.28, 0.10).y,
        3.0,
        visual_theme::structure_light(),
    );
    draw_line(
        p(0.28, 0.10).x,
        p(0.28, 0.10).y,
        p(0.88, 0.10).x,
        p(0.88, 0.10).y,
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

pub(super) fn draw_yard(rect: Rect) {
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;
    // Heavy docking arms frame the opening; the boat approaches from the left.
    draw_rectangle(
        x + w * 0.58,
        y + h * 0.06,
        w * 0.39,
        h * 0.88,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        x + w * 0.62,
        y + h * 0.14,
        w * 0.30,
        h * 0.73,
        visual_theme::space(),
    );
    for index in 0..6 {
        let column = x + w * (0.66 + index as f32 * 0.046);
        draw_line(
            column,
            y + h * 0.2,
            column,
            y + h * 0.78,
            3.0,
            visual_theme::structure(),
        );
    }
    draw_docking_arms(rect);
    let beam_x = x + w * 0.60;
    draw_rectangle(
        beam_x,
        y + h * 0.12,
        w * 0.05,
        h * 0.80,
        visual_theme::structure(),
    );
    draw_line(
        beam_x,
        y + h * 0.12,
        beam_x,
        y + h * 0.92,
        3.0,
        visual_theme::structure_light(),
    );
    for index in 0..5 {
        let light_x = x + w * (0.10 + index as f32 * 0.10);
        draw_line(
            light_x,
            y + h * 0.71,
            light_x + w * 0.045,
            y + h * 0.71,
            3.0,
            visual_theme::cyan_dim(),
        );
    }
    draw_rectangle(
        x + w * 0.68,
        y + h * 0.29,
        w * 0.15,
        h * 0.06,
        visual_theme::cyan_dim(),
    );
    draw_rectangle(
        x + w * 0.68,
        y + h * 0.43,
        w * 0.15,
        h * 0.02,
        visual_theme::structure_light(),
    );
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

fn draw_docking_arms(rect: Rect) {
    let x = rect.x;
    let y = rect.y;
    let w = rect.w;
    let h = rect.h;
    for top in [true, false] {
        let arm_y = if top { y + h * 0.14 } else { y + h * 0.81 };
        let arm = [
            vec2(x, arm_y),
            vec2(x + w * 0.14, arm_y - h * 0.05),
            vec2(x + w, arm_y - h * 0.05),
            vec2(x + w, arm_y + h * 0.11),
            vec2(x + w * 0.07, arm_y + h * 0.11),
        ];
        polygon(&arm, visual_theme::structure());
        draw_line(
            x + w * 0.14,
            arm_y - h * 0.05,
            x + w,
            arm_y - h * 0.05,
            3.0,
            visual_theme::structure_light(),
        );
        draw_line(
            x + w * 0.07,
            arm_y + h * 0.11,
            x + w,
            arm_y + h * 0.11,
            6.0,
            visual_theme::space(),
        );
        for index in 0..7 {
            let light_x = x + w * (0.10 + index as f32 * 0.12);
            draw_rectangle(
                light_x,
                arm_y + h * 0.025,
                w * 0.05,
                5.0,
                visual_theme::amber(),
            );
        }
    }
}
