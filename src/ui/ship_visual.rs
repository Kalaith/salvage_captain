//! Industrial salvage vessel renderer shared by Port, Travel, and salvage.

use super::visual_theme;
use crate::data::{GameData, ModuleData};
use crate::state::workspace::TransferMode;
use crate::state::{CargoStatus, GameSession};
use macroquad::prelude::*;
mod cargo;
pub use cargo::{module_mount_rect, tractor_emitter_rect};

struct ShipVisual {
    hull: Rect,
    metal: Color,
    dark_metal: Color,
    seam: Color,
    work_light: f32,
}

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
    draw_vessel(ShipView {
        rect,
        session,
        data,
        elapsed,
        selected,
        selected_module,
        show_labels: true,
    });
}

pub fn draw_flight_ship(rect: Rect, session: &GameSession, data: &GameData, elapsed: f32) {
    draw_vessel(ShipView {
        rect,
        session,
        data,
        elapsed,
        selected: false,
        selected_module: None,
        show_labels: false,
    });
}

struct ShipView<'a> {
    rect: Rect,
    session: &'a GameSession,
    data: &'a GameData,
    elapsed: f32,
    selected: bool,
    selected_module: Option<&'a str>,
    show_labels: bool,
}

fn draw_vessel(view: ShipView<'_>) {
    let ShipView {
        rect,
        session,
        data,
        elapsed,
        selected,
        selected_module,
        show_labels,
    } = view;
    let visual = ShipVisual {
        hull: Rect::new(
            rect.x,
            rect.y + (elapsed * 2.0).sin(),
            rect.w,
            rect.h * 0.72,
        ),
        metal: if selected {
            visual_theme::structure_light()
        } else {
            visual_theme::structure()
        },
        dark_metal: visual_theme::structure_dark(),
        seam: visual_theme::with_alpha(visual_theme::structure_light(), 0.62),
        work_light: 0.77 + (elapsed * 2.4).sin() * 0.19,
    };
    draw_ship_shadow(&visual);
    draw_ship_body(&visual);
    draw_ship_cargo_bay(&visual);
    draw_ship_command_section(&visual);
    draw_ship_fuel_spine(&visual);
    draw_ship_engines(&visual);
    draw_ship_landing_struts(&visual);
    draw_tractor_emitter(&visual, rect, selected_module);
    cargo::draw_module_mounts(visual.hull, session, data, elapsed, selected_module);
    cargo::draw_external_cargo(visual.hull, session, data, elapsed, show_labels);
    draw_ship_selection_frame(&visual, selected);
    if !show_labels {
        return;
    }
    draw_text(
        "SALVAGE WORKBOAT  //  SC-07",
        visual.hull.x + 4.0,
        visual.hull.bottom() + 30.0,
        12.0,
        visual_theme::text_dim(),
    );
}

fn draw_ship_shadow(visual: &ShipVisual) {
    draw_rectangle(
        visual.hull.x + visual.hull.w * 0.08,
        visual.hull.y + visual.hull.h * 0.79,
        visual.hull.w * 0.72,
        visual.hull.h * 0.12,
        visual_theme::with_alpha(Color::new(0.0, 0.0, 0.0, 1.0), 0.5),
    );
    draw_circle(
        visual.hull.x + visual.hull.w * 0.6,
        visual.hull.y + visual.hull.h * 0.46,
        visual.hull.w * 0.33,
        visual_theme::with_alpha(visual_theme::amber(), 0.035),
    );
}

fn draw_ship_body(visual: &ShipVisual) {
    let hull = visual.hull;
    draw_rectangle(
        hull.x + hull.w * 0.12,
        hull.y + hull.h * 0.34,
        hull.w * 0.69,
        hull.h * 0.36,
        visual.metal,
    );
    draw_triangle(
        vec2(hull.x + hull.w * 0.76, hull.y + hull.h * 0.34),
        vec2(hull.x + hull.w * 0.96, hull.y + hull.h * 0.52),
        vec2(hull.x + hull.w * 0.76, hull.y + hull.h * 0.70),
        visual.metal,
    );
    draw_rectangle(
        hull.x + hull.w * 0.08,
        hull.y + hull.h * 0.27,
        hull.w * 0.2,
        hull.h * 0.48,
        visual.dark_metal,
    );
    draw_rectangle(
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.36,
        hull.w * 0.08,
        hull.h * 0.28,
        visual.seam,
    );
    draw_line(
        hull.x + hull.w * 0.12,
        hull.y + hull.h * 0.70,
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.70,
        3.0,
        visual.seam,
    );
    draw_line(
        hull.x + hull.w * 0.16,
        hull.y + hull.h * 0.36,
        hull.x + hull.w * 0.75,
        hull.y + hull.h * 0.36,
        2.0,
        visual.seam,
    );
    draw_line(
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.35,
        hull.x + hull.w * 0.96,
        hull.y + hull.h * 0.52,
        2.0,
        visual.seam,
    );
    draw_line(
        hull.x + hull.w * 0.78,
        hull.y + hull.h * 0.69,
        hull.x + hull.w * 0.96,
        hull.y + hull.h * 0.52,
        2.0,
        visual.seam,
    );
}

fn draw_ship_cargo_bay(visual: &ShipVisual) {
    let hull = visual.hull;
    draw_rectangle(
        hull.x + hull.w * 0.28,
        hull.y + hull.h * 0.39,
        hull.w * 0.31,
        hull.h * 0.24,
        visual.dark_metal,
    );
    for index in 0..4 {
        let x = hull.x + hull.w * (0.3 + index as f32 * 0.075);
        draw_line(
            x,
            hull.y + hull.h * 0.4,
            x,
            hull.y + hull.h * 0.62,
            2.0,
            visual.seam,
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
}

fn draw_ship_command_section(visual: &ShipVisual) {
    let hull = visual.hull;
    draw_rectangle(
        hull.x + hull.w * 0.63,
        hull.y + hull.h * 0.16,
        hull.w * 0.2,
        hull.h * 0.22,
        visual.dark_metal,
    );
    draw_triangle(
        vec2(hull.x + hull.w * 0.63, hull.y + hull.h * 0.16),
        vec2(hull.x + hull.w * 0.83, hull.y + hull.h * 0.16),
        vec2(hull.x + hull.w * 0.78, hull.y + hull.h * 0.32),
        visual.dark_metal,
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
        visual.seam,
    );
}

fn draw_ship_fuel_spine(visual: &ShipVisual) {
    let hull = visual.hull;
    for (x, width) in [(0.34, 0.13), (0.49, 0.11)] {
        draw_rectangle(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.2,
            hull.w * width,
            hull.h * 0.12,
            visual.dark_metal,
        );
        draw_rectangle_lines(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.2,
            hull.w * width,
            hull.h * 0.12,
            2.0,
            visual.seam,
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
        visual.dark_metal,
    );
    draw_line(
        hull.x + hull.w * 0.2,
        hull.y + hull.h * 0.1,
        hull.x + hull.w * 0.2,
        hull.y + hull.h * 0.35,
        2.0,
        visual.seam,
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
                visual.work_light,
            ),
        );
    }
}

fn draw_ship_engines(visual: &ShipVisual) {
    let hull = visual.hull;
    for index in 0..3 {
        let y = hull.y + hull.h * (0.31 + index as f32 * 0.14);
        draw_rectangle(
            hull.x + hull.w * 0.01,
            y,
            hull.w * 0.1,
            hull.h * 0.09,
            visual.dark_metal,
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
            Color::new(1.0, 0.8, 0.3, visual.work_light),
        );
    }
    draw_line(
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.25,
        hull.x + hull.w * 0.03,
        hull.y + hull.h * 0.76,
        3.0,
        visual.seam,
    );
}

fn draw_ship_landing_struts(visual: &ShipVisual) {
    let hull = visual.hull;
    for x in [0.27, 0.63] {
        draw_rectangle(
            hull.x + hull.w * x,
            hull.y + hull.h * 0.68,
            hull.w * 0.06,
            hull.h * 0.27,
            visual.dark_metal,
        );
        draw_line(
            hull.x + hull.w * (x + 0.03),
            hull.y + hull.h * 0.84,
            hull.x + hull.w * (x - 0.03),
            hull.y + hull.h * 0.98,
            3.0,
            visual.seam,
        );
        draw_line(
            hull.x + hull.w * (x - 0.08),
            hull.y + hull.h * 0.98,
            hull.x + hull.w * (x + 0.08),
            hull.y + hull.h * 0.98,
            4.0,
            visual.seam,
        );
    }
}

fn draw_tractor_emitter(visual: &ShipVisual, rect: Rect, selected_module: Option<&str>) {
    let hull = visual.hull;
    draw_rectangle(
        hull.x + hull.w * 0.87,
        hull.y + hull.h * 0.39,
        hull.w * 0.09,
        hull.h * 0.25,
        visual.dark_metal,
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
}

fn draw_ship_selection_frame(visual: &ShipVisual, selected: bool) {
    if selected {
        draw_rectangle_lines(
            visual.hull.x - 5.0,
            visual.hull.y - 5.0,
            visual.hull.w + 10.0,
            visual.hull.h + visual.hull.h * 0.32,
            2.0,
            visual_theme::cyan_dim(),
        );
    }
}
