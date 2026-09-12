//! Industrial workboat rendering shared by port, travel, and salvage.

use super::visual_theme;
use crate::data::GameData;
use crate::state::workspace::TransferMode;
use crate::state::{CargoStatus, GameSession};
use macroquad::prelude::*;

mod cargo;
mod equipment;
mod hull;
mod ink;
mod mounts;
pub use mounts::{module_mount_rect, tractor_emitter_rect};

const TRACTOR_NOZZLE: (f32, f32) = (976., 176.);

pub fn tractor_nozzle(rect: Rect) -> Vec2 {
    let hull = hull_rect(rect, 0.);
    vec2(
        hull.x + hull.w * TRACTOR_NOZZLE.0 / 1000.,
        hull.y + hull.h * TRACTOR_NOZZLE.1 / 360.,
    )
}

pub fn draw_ship(rect: Rect, session: &GameSession, data: &GameData, elapsed: f32, selected: bool) {
    draw_vessel(ShipView {
        rect,
        session,
        data,
        elapsed,
        selected,
        selected_module: None,
        show_labels: true,
        docked: false,
    });
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
        docked: true,
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
        docked: false,
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
    docked: bool,
}

fn hull_rect(rect: Rect, elapsed: f32) -> Rect {
    Rect::new(rect.x, rect.y + (elapsed * 2.).sin(), rect.w, rect.h * 0.72)
}

fn draw_vessel(view: ShipView<'_>) {
    let hull = hull_rect(view.rect, if view.docked { 0. } else { view.elapsed });
    let ink = ink::ShipInk {
        rect: hull,
        size: vec2(1000., 360.),
        opacity: 1.,
    };
    if view.docked {
        draw_ellipse(
            hull.center().x,
            hull.bottom() + 9.,
            hull.w * 0.4,
            hull.h * 0.045,
            0.,
            visual_theme::with_alpha(BLACK, 0.35),
        );
    }
    hull::draw(&ink, view.elapsed, view.docked);
    mounts::draw(
        hull,
        view.session,
        view.data,
        view.elapsed,
        view.selected_module,
    );
    cargo::draw_external_cargo(
        hull,
        view.session,
        view.data,
        view.elapsed,
        view.show_labels,
    );
    if view.selected {
        mounts::draw_brackets(hull, visual_theme::cyan());
    }
    if view.selected_module == Some("engine_core") {
        mounts::draw_brackets(tractor_emitter_rect(view.rect), visual_theme::amber());
    }
    if view.show_labels {
        draw_text(
            "SALVAGE WORKBOAT  //  SC-07",
            hull.x + 4.,
            hull.bottom() + 52.,
            12.,
            visual_theme::text_dim(),
        );
    }
}
