//! Side-on wreck and industrial yard silhouettes with shallow depth.

use super::*;

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
