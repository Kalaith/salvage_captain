//! Industrial salvage vessel renderer shared by Port, Travel, and salvage.

use super::visual_theme;
use crate::data::{GameData, ModuleData};
use crate::state::workspace::TransferMode;
use crate::state::{CargoStatus, GameSession};
use macroquad::prelude::*;
use macroquad_toolkit::math::{bob, pulse_range};
mod cargo;
pub use cargo::{module_mount_rect, tractor_emitter_rect};

pub fn draw_ship(rect: Rect, session: &GameSession, data: &GameData, elapsed: f32, selected: bool) {
    draw_ship_with_selection(rect, session, data, elapsed, selected, None);
}

pub fn draw_ship_with_selection(
    rect: Rect,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    selected: bool,
    selected_module: Option<&str>,
) {
    let drift = bob(1.0, 2.0);
    let hull = Rect::new(rect.x, rect.y + drift, rect.w, rect.h * 0.72);
    let metal = if selected {
        visual_theme::structure_light()
    } else {
        visual_theme::structure()
    };
    let dark_metal = visual_theme::structure_dark();
    let seam = visual_theme::with_alpha(visual_theme::structure_light(), 0.62);
    let work_light = pulse_range(2.4, 0.58, 0.96);

    draw_rectangle(
        hull.x + hull.w * 0.08,
        hull.y + hull.h * 0.79,
        hull.w * 0.72,
        hull.h * 0.12,
        visual_theme::with_alpha(Color::new(0.0, 0.0, 0.0, 1.0), 0.5),
    );
    draw_circle(
        hull.x + hull.w * 0.6,
        hull.y + hull.h * 0.46,
        hull.w * 0.33,
        visual_theme::with_alpha(visual_theme::amber(), 0.035),
    );

    // Chunky primary hull: a deep belly, a tapered bow, and an armored aft block.
    draw_rectangle(
        hull.x + hull.w * 0.12,
        hull.y + hull.h * 0.34,
        hull.w * 0.69,
        hull.h * 0.36,
        metal,
    );
    draw_triangle(
        vec2(hull.x + hull.w * 0.76, hull.y + hull.h * 0.34),
        vec2(hull.x + hull.w * 0.96, hull.y + hull.h * 0.52),
        vec2(hull.x + hull.w * 0.76, hull.y + hull.h * 0.70),
        metal,
    );
    draw_rectangle(
        hull.x + hull.w * 0.08,
        hull.y + hull.h * 0.27,
        hull.w * 0.2,
        hull.h * 0.48,
        dark_metal,
    );
    draw_rectangle(
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.36,
        hull.w * 0.08,
        hull.h * 0.28,
        seam,
    );
    draw_line(
        hull.x + hull.w * 0.12,
        hull.y + hull.h * 0.70,
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.70,
        3.0,
        seam,
    );
    draw_line(
        hull.x + hull.w * 0.16,
        hull.y + hull.h * 0.36,
        hull.x + hull.w * 0.75,
        hull.y + hull.h * 0.36,
        2.0,
        seam,
    );
    draw_line(
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.35,
        hull.x + hull.w * 0.96,
        hull.y + hull.h * 0.52,
        2.0,
        seam,
    );
    draw_line(
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.69,
        hull.x + hull.w * 0.96,
        hull.y + hull.h * 0.52,
        2.0,
        seam,
    );

    // Cargo bay and pressure ribs.
    draw_rectangle(
        hull.x + hull.w * 0.28,
        hull.y + hull.h * 0.39,
        hull.w * 0.31,
        hull.h * 0.24,
        dark_metal,
    );
    for index in 0..4 {
        let x = hull.x + hull.w * (0.3 + index as f32 * 0.075);
        draw_line(
            x,
            hull.y + hull.h * 0.4,
            x,
            hull.y + hull.h * 0.62,
            2.0,
            seam,
        );
    }
    draw_rectangle(
        hull.x + hull.w * 0.34,
        hull.y + hull.h * 0.45,
        hull.w * 0.18,
        hull.h * 0.08,
        visual_theme::cyan_dim(),
    );
    draw_line(
        hull.x + hull.w * 0.35,
        hull.y + hull.h * 0.49,
        hull.x + hull.w * 0.51,
        hull.y + hull.h * 0.49,
        2.0,
        visual_theme::cyan(),
    );

    // Command section and cyan cockpit glass.
    draw_rectangle(
        hull.x + hull.w * 0.63,
        hull.y + hull.h * 0.16,
        hull.w * 0.2,
        hull.h * 0.22,
        dark_metal,
    );
    draw_triangle(
        vec2(hull.x + hull.w * 0.63, hull.y + hull.h * 0.16),
        vec2(hull.x + hull.w * 0.83, hull.y + hull.h * 0.16),
        vec2(hull.x + hull.w * 0.78, hull.y + hull.h * 0.32),
        dark_metal,
    );
    draw_rectangle(
        hull.x + hull.w * 0.68,
        hull.y + hull.h * 0.2,
        hull.w * 0.12,
        hull.h * 0.08,
        visual_theme::cyan_dim(),
    );
    draw_line(
        hull.x + hull.w * 0.69,
        hull.y + hull.h * 0.24,
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.24,
        2.0,
        visual_theme::cyan(),
    );
    draw_line(
        hull.x + hull.w * 0.64,
        hull.y + hull.h * 0.16,
        hull.x + hull.w * 0.6,
        hull.y + hull.h * 0.34,
        2.0,
        seam,
    );

    // Fuel tanks, crane spine, and warm maintenance lamps.
    for (x, width) in [(0.34, 0.13), (0.49, 0.11)] {
        draw_rectangle(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.2,
            hull.w * width,
            hull.h * 0.12,
            dark_metal,
        );
        draw_rectangle_lines(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.2,
            hull.w * width,
            hull.h * 0.12,
            2.0,
            seam,
        );
        draw_line(
            hull.x + hull.w * (x + 0.02),
            hull.y + hull.h * 0.24,
            hull.x + hull.w * (x + width - 0.02),
            hull.y + hull.h * 0.24,
            2.0,
            visual_theme::amber(),
        );
    }
    draw_line(
        hull.x + hull.w * 0.2,
        hull.y + hull.h * 0.1,
        hull.x + hull.w * 0.74,
        hull.y + hull.h * 0.1,
        3.0,
        dark_metal,
    );
    draw_line(
        hull.x + hull.w * 0.2,
        hull.y + hull.h * 0.1,
        hull.x + hull.w * 0.2,
        hull.y + hull.h * 0.35,
        2.0,
        seam,
    );
    for index in 0..4 {
        draw_circle(
            hull.x + hull.w * (0.25 + index as f32 * 0.17),
            hull.y + hull.h * 0.58,
            4.0,
            Color::new(
                visual_theme::amber().r,
                visual_theme::amber().g,
                visual_theme::amber().b,
                work_light,
            ),
        );
    }

    // Aft engine block with three visible exhaust nozzles.
    for index in 0..3 {
        let y = hull.y + hull.h * (0.31 + index as f32 * 0.14);
        draw_rectangle(
            hull.x + hull.w * 0.01,
            y,
            hull.w * 0.1,
            hull.h * 0.09,
            dark_metal,
        );
        draw_circle(
            hull.x - hull.w * 0.005,
            y + hull.h * 0.045,
            hull.h * 0.045,
            visual_theme::amber(),
        );
        draw_circle(
            hull.x - hull.w * 0.023,
            y + hull.h * 0.045,
            hull.h * 0.022,
            Color::new(1.0, 0.8, 0.3, work_light),
        );
    }
    draw_line(
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.25,
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.76,
        3.0,
        seam,
    );

    // Landing struts and support skids make the vessel read as machinery.
    for x in [0.27, 0.63] {
        draw_rectangle(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.68,
            hull.w * 0.06,
            hull.h * 0.27,
            dark_metal,
        );
        draw_line(
            hull.x + hull.w * (x + 0.03),
            hull.y + hull.h * 0.84,
            hull.x + hull.w * (x - 0.03),
            hull.y + hull.h * 0.98,
            3.0,
            seam,
        );
        draw_line(
            hull.x + hull.w * (x - 0.08),
            hull.y + hull.h * 0.98,
            hull.x + hull.w * (x + 0.08),
            hull.y + hull.h * 0.98,
            4.0,
            seam,
        );
    }

    // Tractor emitter on the bow: the most legible cyan technology signal.
    draw_rectangle(
        hull.x + hull.w * 0.87,
        hull.y + hull.h * 0.39,
        hull.w * 0.09,
        hull.h * 0.25,
        dark_metal,
    );
    draw_line(
        hull.x + hull.w * 0.92,
        hull.y + hull.h * 0.45,
        hull.x + hull.w * 0.92,
        hull.y + hull.h * 0.12,
        3.0,
        visual_theme::cyan(),
    );
    draw_circle(
        hull.x + hull.w * 0.92,
        hull.y + hull.h * 0.12,
        7.0,
        visual_theme::cyan(),
    );
    draw_circle(
        hull.x + hull.w * 0.92,
        hull.y + hull.h * 0.12,
        13.0,
        visual_theme::with_alpha(visual_theme::cyan(), 0.18),
    );
    draw_line(
        hull.x + hull.w * 0.88,
        hull.y + hull.h * 0.46,
        hull.x + hull.w * 1.02,
        hull.y + hull.h * 0.46,
        2.0,
        visual_theme::cyan(),
    );
    if selected_module == Some("engine_core") {
        let emitter = tractor_emitter_rect(rect);
        draw_rectangle_lines(
            emitter.x,
            emitter.y,
            emitter.w,
            emitter.h,
            2.0,
            visual_theme::amber(),
        );
    }

    cargo::draw_module_mounts(hull, session, data, elapsed, selected_module);
    cargo::draw_external_cargo(hull, session, data, elapsed);
    if selected {
        draw_rectangle_lines(
            hull.x - 5.0,
            hull.y - 5.0,
            hull.w + 10.0,
            hull.h + hull.h * 0.32,
            2.0,
            visual_theme::cyan_dim(),
        );
    }
    draw_text(
        "SALVAGE WORKBOAT  //  SC-07",
        hull.x + 4.0,
        hull.bottom() + 30.0,
        12.0,
        visual_theme::text_dim(),
    );
}
