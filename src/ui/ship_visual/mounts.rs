//! Shared hardpoint geometry and installed, damaged, and preview presentation.

use super::{equipment, hull_rect, ink::ShipInk, visual_theme as theme};
use crate::data::{GameData, ModuleData};
use crate::state::GameSession;
use macroquad::prelude::*;

pub fn module_mount_rect(rect: Rect, module: &ModuleData) -> Rect {
    let bounds = equipment_rect(hull_rect(rect, 0.), &module.mount);
    // Port hit targets include the complete machinery and remain touch sized.
    Rect::new(
        bounds.center().x - bounds.w.max(64.) * 0.5,
        bounds.center().y - bounds.h.max(64.) * 0.5,
        bounds.w.max(64.),
        bounds.h.max(64.),
    )
}

pub fn tractor_emitter_rect(rect: Rect) -> Rect {
    let hull = hull_rect(rect, 0.);
    Rect::new(
        hull.x + hull.w * 0.895,
        hull.y + hull.h * 0.06,
        hull.w * 0.095,
        hull.h * 0.55,
    )
}

fn equipment_rect(hull: Rect, mount: &str) -> Rect {
    let (x, y, w, h) = match mount {
        "aft_core" => (122., 183., 96., 88.),
        "port_tank" => (380., 83., 140., 55.),
        "bridge_nav" => (709., 46., 76., 70.),
        "bow_scanner" => (841., 123., 85., 72.),
        "starboard_reactor" => (585., 229., 105., 78.),
        "starboard_shield" => (734., 229., 100., 72.),
        "port_drone" => (314., 229., 120., 68.),
        "port_battery" => (451., 229., 90., 68.),
        "hull_patch" => (526., 137., 130., 56.),
        _ => (500., 180., 90., 65.),
    };
    Rect::new(
        hull.x + (x - w * 0.5) * hull.w / 1000.,
        hull.y + (y - h * 0.5) * hull.h / 360.,
        w * hull.w / 1000.,
        h * hull.h / 360.,
    )
}

pub(super) fn draw(
    hull: Rect,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    selected_module: Option<&str>,
) {
    // Recessed blank plates show where additional equipment can be bolted on.
    for (_, module) in data.modules.iter() {
        let bounds = equipment_rect(hull, &module.mount);
        let ink = ShipInk {
            rect: bounds,
            size: vec2(100., 80.),
            opacity: 1.,
        };
        ink.panel((7., 54., 86., 20.), theme::structure_dark());
        ink.line((10., 74.), (90., 74.), 2., theme::structure_light());
        ink.bolts((12., 58., 76., 12.), theme::structure_light());
    }
    for placement in session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
    {
        let Some(module) = data.modules.get(&placement.id) else {
            continue;
        };
        let bounds = equipment_rect(hull, &module.mount);
        let damaged = session.damaged_modules.contains(&placement.id);
        let ink = ShipInk {
            rect: bounds,
            size: vec2(100., 80.),
            opacity: 1.,
        };
        equipment::draw(&ink, &module.visual_kind, elapsed, damaged);
        if damaged {
            draw_damage(&ink);
        }
        if selected_module == Some(placement.id.as_str()) {
            draw_brackets(bounds, theme::amber());
        }
    }
    if let Some(module) = selected_module.and_then(|id| data.modules.get(id)) {
        let installed = session
            .ship_layout
            .placements
            .iter()
            .any(|p| p.permanent && p.id == module.id);
        if !installed {
            let bounds = equipment_rect(hull, &module.mount);
            let ink = ShipInk {
                rect: bounds,
                size: vec2(100., 80.),
                opacity: 0.48 + (elapsed * 3.).sin().abs() * 0.16,
            };
            equipment::draw(&ink, &module.visual_kind, elapsed, false);
            draw_brackets(bounds, theme::cyan());
        }
    }
}

fn draw_damage(ink: &ShipInk) {
    ink.disc((53., 39.), 18., theme::with_alpha(BLACK, 0.65));
    ink.line((37., 22.), (52., 41.), 3., theme::space());
    ink.line((52., 41.), (44., 49.), 3., theme::space());
    ink.line((44., 49.), (64., 61.), 3., theme::space());
    ink.polygon(&[(72., 4.), (89., 30.), (55., 30.)], theme::warning());
    ink.line((72., 12.), (72., 20.), 3., theme::space());
    ink.disc((72., 25.), 1.7, theme::space());
    draw_brackets(ink.rect, theme::warning());
}

pub(super) fn draw_brackets(bounds: Rect, color: Color) {
    let pad = 5.;
    let length = (bounds.w.min(bounds.h) * 0.22).clamp(5., 14.);
    for (x, dx) in [(bounds.x - pad, length), (bounds.right() + pad, -length)] {
        for (y, dy) in [(bounds.y - pad, length), (bounds.bottom() + pad, -length)] {
            draw_line(x, y, x + dx, y, 1.5, color);
            draw_line(x, y, x, y + dy, 1.5, color);
        }
    }
}
