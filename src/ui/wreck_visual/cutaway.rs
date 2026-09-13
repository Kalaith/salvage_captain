//! Side-on hull thickness, recessed rooms, and foreground wreck fragments.

use super::visual_theme as theme;
use macroquad::prelude::*;

pub(super) fn draw_shell(wreck: Rect, accent: Color) {
    let outline = shell_outline(wreck);
    let center = wreck.center();
    draw_shell_shadow(center, &outline);
    draw_shell_body(center, &outline);
    let interior = Rect::new(
        wreck.x + 100.0,
        wreck.y + 38.0,
        wreck.w - 160.0,
        wreck.h - 80.0,
    );
    draw_shell_rooms(wreck, interior, accent);
    draw_shell_edges(wreck);
    draw_torn_plating(wreck);
    draw_shell_rivets(wreck);
}

fn shell_outline(wreck: Rect) -> [Vec2; 10] {
    let x = wreck.x;
    let y = wreck.y;
    let right = wreck.right();
    let bottom = wreck.bottom();
    [
        vec2(x + 60.0, y + 30.0),
        vec2(x + 132.0, y),
        vec2(right - 92.0, y),
        vec2(right, y + 58.0),
        vec2(right - 14.0, bottom - 36.0),
        vec2(right - 78.0, bottom),
        vec2(x + 86.0, bottom),
        vec2(x + 30.0, bottom - 44.0),
        vec2(x + 70.0, bottom - 96.0),
        vec2(x + 10.0, y + 146.0),
    ]
}

fn draw_shell_shadow(center: Vec2, outline: &[Vec2; 10]) {
    for index in 0..outline.len() {
        let next = (index + 1) % outline.len();
        draw_triangle(
            center + vec2(8.0, 14.0),
            outline[index] + vec2(8.0, 14.0),
            outline[next] + vec2(8.0, 14.0),
            theme::with_alpha(BLACK, 0.65),
        );
    }
}

fn draw_shell_body(center: Vec2, outline: &[Vec2; 10]) {
    for index in 0..outline.len() {
        draw_triangle(
            center,
            outline[index],
            outline[(index + 1) % outline.len()],
            theme::structure(),
        );
    }
}

fn draw_shell_rooms(wreck: Rect, interior: Rect, accent: Color) {
    draw_rectangle(
        interior.x,
        interior.y,
        interior.w,
        interior.h,
        theme::space(),
    );
    for index in 0..3 {
        let room = Rect::new(
            interior.x + index as f32 * 254.0 + 8.0,
            interior.y + 14.0,
            232.0,
            interior.h - 28.0,
        );
        draw_room(room, interior, wreck.y, accent);
    }
}

fn draw_room(room: Rect, interior: Rect, top: f32, accent: Color) {
    draw_rectangle(
        room.x,
        room.y,
        room.w,
        room.h,
        Color::new(0.065, 0.10, 0.13, 1.0),
    );
    draw_rectangle(room.x, room.y, room.w, 16.0, theme::with_alpha(BLACK, 0.55));
    draw_rectangle(room.x, room.y, 12.0, room.h, theme::with_alpha(BLACK, 0.4));
    for seam in 1..5 {
        let seam_x = room.x + seam as f32 * 44.0;
        draw_line(
            seam_x,
            room.y + 22.0,
            seam_x,
            room.bottom(),
            1.0,
            theme::with_alpha(theme::structure_light(), 0.12),
        );
    }
    let rib_x = room.right() + 2.0;
    draw_rectangle(
        rib_x + 7.0,
        interior.y,
        14.0,
        interior.h,
        theme::with_alpha(BLACK, 0.65),
    );
    draw_rectangle(
        rib_x,
        interior.y - 8.0,
        12.0,
        interior.h + 16.0,
        theme::structure(),
    );
    draw_line(
        rib_x,
        interior.y - 8.0,
        rib_x,
        interior.bottom() + 8.0,
        2.0,
        theme::structure_light(),
    );
    draw_rectangle(
        room.x + 54.0,
        top + 23.0,
        72.0,
        5.0,
        theme::with_alpha(accent, 0.65),
    );
}

fn draw_shell_edges(wreck: Rect) {
    let x = wreck.x;
    let y = wreck.y;
    let right = wreck.right();
    let bottom = wreck.bottom();
    draw_line(
        x + 132.0,
        y + 2.0,
        right - 92.0,
        y + 2.0,
        3.0,
        theme::structure_light(),
    );
    draw_line(
        right - 92.0,
        y + 2.0,
        right,
        y + 58.0,
        3.0,
        theme::structure_light(),
    );
    draw_rectangle(
        x + 90.0,
        bottom - 34.0,
        wreck.w - 168.0,
        22.0,
        theme::structure_dark(),
    );
    draw_line(
        x + 90.0,
        bottom - 34.0,
        right - 78.0,
        bottom - 34.0,
        3.0,
        theme::structure_light(),
    );
    draw_line(x + 86.0, bottom, right - 78.0, bottom, 8.0, theme::space());
}

fn draw_torn_plating(wreck: Rect) {
    let x = wreck.x;
    let y = wreck.y;
    let bottom = wreck.bottom();
    // Torn plating exposes space at the ship-facing edge.
    draw_triangle(
        vec2(x, y + 104.0),
        vec2(x + 130.0, y + 164.0),
        vec2(x + 28.0, bottom - 36.0),
        theme::space(),
    );
    draw_line(
        x + 44.0,
        y + 126.0,
        x + 130.0,
        y + 164.0,
        6.0,
        theme::structure_light(),
    );
}

fn draw_shell_rivets(wreck: Rect) {
    for index in 0..12 {
        let rivet_x = wreck.x + 152.0 + index as f32 * 55.0;
        draw_circle(rivet_x, wreck.y + 14.0, 2.0, theme::structure_dark());
        draw_circle(
            rivet_x,
            wreck.bottom() - 20.0,
            2.0,
            theme::structure_light(),
        );
    }
}

pub(super) fn draw_foreground(wreck: Rect, accent: Color) {
    let bottom = wreck.bottom();
    for index in 0..3 {
        let x = wreck.x + 138.0 + index as f32 * 260.0;
        draw_triangle(
            vec2(x, bottom - 5.0),
            vec2(x + 140.0, bottom - 5.0),
            vec2(x + 118.0, bottom - 56.0),
            theme::structure_dark(),
        );
        draw_line(
            x,
            bottom - 5.0,
            x + 118.0,
            bottom - 56.0,
            3.0,
            theme::structure_light(),
        );
        draw_line(
            x + 58.0,
            bottom - 11.0,
            x + 92.0,
            bottom - 11.0,
            3.0,
            theme::with_alpha(accent, 0.6),
        );
    }
    // A detached piece sits in front of the hull without hiding a target.
    draw_triangle(
        vec2(wreck.x - 38.0, bottom - 2.0),
        vec2(wreck.x + 22.0, bottom + 13.0),
        vec2(wreck.x + 4.0, bottom - 30.0),
        theme::structure(),
    );
    draw_line(
        wreck.x - 38.0,
        bottom - 2.0,
        wreck.x + 4.0,
        bottom - 30.0,
        2.0,
        theme::structure_light(),
    );
}
