//! Small, patched work-vessel renderer shared by Port, Travel, and salvage.

use super::visual_theme;
use crate::data::GameData;
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::math::{bob, pulse_range};

pub fn emitter_point(rect: Rect) -> Vec2 {
    vec2(rect.x + rect.w * 0.94, rect.y + rect.h * 0.45)
}

pub fn draw_ship(rect: Rect, session: &GameSession, data: &GameData, elapsed: f32, selected: bool) {
    let drift = bob(1.1, 3.0);
    let hull = Rect::new(rect.x, rect.y + drift, rect.w, rect.h * 0.68);
    let hull_color = if selected {
        visual_theme::structure_light()
    } else {
        visual_theme::structure()
    };
    draw_rectangle(hull.x, hull.y, hull.w, hull.h, hull_color);
    draw_rectangle(
        hull.x + hull.w * 0.12,
        hull.y - hull.h * 0.18,
        hull.w * 0.34,
        hull.h * 0.24,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        hull.x + hull.w * 0.54,
        hull.y - hull.h * 0.08,
        hull.w * 0.2,
        hull.h * 0.16,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        hull.x + hull.w * 0.15,
        hull.y + hull.h * 0.18,
        hull.w * 0.18,
        hull.h * 0.28,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        hull.x + hull.w * 0.74,
        hull.y + hull.h * 0.13,
        hull.w * 0.16,
        hull.h * 0.3,
        visual_theme::structure_dark(),
    );
    draw_line(
        hull.x + hull.w * 0.02,
        hull.y + hull.h,
        hull.x + hull.w * 0.88,
        hull.y + hull.h,
        3.0,
        visual_theme::structure_light(),
    );
    draw_line(
        hull.x + hull.w * 0.1,
        hull.y + hull.h * 0.24,
        hull.x + hull.w * 0.83,
        hull.y + hull.h * 0.24,
        2.0,
        visual_theme::structure_light(),
    );

    for index in 0..4 {
        let x = hull.x + hull.w * (0.12 + index as f32 * 0.18);
        draw_line(
            x,
            hull.y + hull.h * 0.2,
            x - 12.0,
            hull.y + hull.h * 0.9,
            1.0,
            visual_theme::structure_dark(),
        );
    }

    let work_light = pulse_range(2.4, 0.65, 1.0);
    for index in 0..3 {
        let x = hull.x + hull.w * (0.18 + index as f32 * 0.28);
        draw_circle(
            x,
            hull.y + hull.h * 0.38,
            4.0,
            Color::new(
                visual_theme::amber().r,
                visual_theme::amber().g,
                visual_theme::amber().b,
                work_light,
            ),
        );
    }

    draw_rectangle(
        hull.x + hull.w * 0.88,
        hull.y + hull.h * 0.28,
        hull.w * 0.1,
        hull.h * 0.35,
        visual_theme::cyan_dim(),
    );
    draw_line(
        hull.x + hull.w * 0.94,
        hull.y + hull.h * 0.3,
        hull.x + hull.w * 0.94,
        hull.y - hull.h * 0.2,
        3.0,
        visual_theme::cyan(),
    );
    draw_circle(
        hull.x + hull.w * 0.94,
        hull.y - hull.h * 0.2,
        5.0,
        visual_theme::cyan(),
    );

    draw_rectangle(
        hull.x + hull.w * 0.35,
        hull.y + hull.h * 0.78,
        hull.w * 0.12,
        hull.h * 0.28,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        hull.x + hull.w * 0.58,
        hull.y + hull.h * 0.78,
        hull.w * 0.12,
        hull.h * 0.28,
        visual_theme::structure_dark(),
    );
    draw_circle(
        hull.x + hull.w * 0.41,
        hull.y + hull.h * 1.02,
        hull.h * 0.09,
        Color::new(
            visual_theme::amber().r,
            visual_theme::amber().g,
            visual_theme::amber().b,
            work_light,
        ),
    );
    draw_circle(
        hull.x + hull.w * 0.64,
        hull.y + hull.h * 1.02,
        hull.h * 0.09,
        Color::new(
            visual_theme::amber().r,
            visual_theme::amber().g,
            visual_theme::amber().b,
            work_light,
        ),
    );

    draw_line(
        hull.x + hull.w * 0.07,
        hull.y + hull.h * 0.65,
        hull.x - 18.0,
        hull.y + hull.h * 1.03,
        2.0,
        visual_theme::amber(),
    );
    draw_circle(
        hull.x - 20.0,
        hull.y + hull.h * 1.05,
        7.0,
        visual_theme::structure_light(),
    );

    draw_module_mounts(hull, session, data);
    draw_text(
        "WORK VESSEL  //  SC-07",
        hull.x,
        hull.bottom() + 24.0,
        12.0,
        visual_theme::text_dim(),
    );
    let _ = elapsed;
}

fn draw_module_mounts(hull: Rect, session: &GameSession, data: &GameData) {
    for (index, placement) in session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
        .enumerate()
    {
        let Some(module) = data.modules.get(&placement.id) else {
            continue;
        };
        let x = hull.x + hull.w * (0.22 + index as f32 * 0.12).min(0.72);
        let y = hull.y - 8.0 - (index % 2) as f32 * 10.0;
        match module.visual_kind.as_str() {
            "scanner" => {
                draw_line(x, y, x + 18.0, y - 20.0, 2.0, visual_theme::cyan());
                draw_circle(x + 18.0, y - 20.0, 4.0, visual_theme::cyan());
            }
            "reactor" => {
                draw_circle(x, y, 9.0, visual_theme::warning());
                draw_circle(x, y, 4.0, visual_theme::amber());
            }
            "shield" => draw_rectangle(x - 7.0, y - 6.0, 14.0, 12.0, visual_theme::safe()),
            "antenna" => draw_line(x, y, x, y - 26.0, 2.0, visual_theme::text_dim()),
            "tank" | "battery" => {
                draw_rectangle(
                    x - 12.0,
                    y - 5.0,
                    24.0,
                    10.0,
                    visual_theme::structure_light(),
                );
            }
            _ => draw_circle(x, y, 6.0, visual_theme::structure_light()),
        }
    }
    let clamp_capacity = session.external_capacity(data).max(0) as usize;
    let external = session.external_cargo_count(data, None).max(0) as usize;
    let visible_clamps = clamp_capacity.min(6);
    let clamp_step = if visible_clamps <= 1 {
        0.0
    } else {
        (hull.w - 44.0) / (visible_clamps - 1) as f32
    };
    for index in 0..visible_clamps {
        let x = hull.x + 12.0 + index as f32 * clamp_step;
        draw_rectangle_lines(
            x,
            hull.bottom() + 2.0,
            32.0,
            14.0,
            2.0,
            visual_theme::structure_light(),
        );
        if index < external {
            draw_rectangle(
                x + 2.0,
                hull.bottom() + 4.0,
                28.0,
                10.0,
                visual_theme::amber(),
            );
        }
        draw_line(
            x + 16.0,
            hull.bottom(),
            x + 16.0,
            hull.bottom() + 2.0,
            2.0,
            visual_theme::structure_light(),
        );
    }
    if clamp_capacity > 0 {
        draw_text(
            format!("CLAMPS {}/{}", external, clamp_capacity),
            hull.x + 12.0,
            hull.bottom() + 34.0,
            10.0,
            visual_theme::text_dim(),
        );
    }
}
