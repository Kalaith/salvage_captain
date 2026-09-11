//! Industrial hangar geometry behind the port controls.

use super::*;

pub(super) fn draw_hangar_world(world: Rect, cargo_row: Rect) {
    draw_rectangle(
        world.x,
        world.y,
        world.w,
        world.h,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.38),
    );
    draw_rectangle(
        world.x,
        world.y,
        world.w,
        7.0,
        visual_theme::with_alpha(visual_theme::structure_dark(), 0.9),
    );
    draw_text(
        "HANGAR DECK  //  SC-07",
        world.x + 26.0,
        world.y + 34.0,
        16.0,
        visual_theme::text(),
    );
    draw_text(
        "PATCHED WORKBOAT  //  BERTH 04",
        world.right() - 236.0,
        world.y + 33.0,
        10.0,
        visual_theme::text_dim(),
    );

    let structure = visual_theme::with_alpha(visual_theme::structure_light(), 0.16);
    let deep = visual_theme::with_alpha(visual_theme::structure_dark(), 0.86);
    let floor = (cargo_row.y - 14.0).max(world.y + 230.0);
    draw_line(
        world.x + 22.0,
        world.y + 68.0,
        world.right() - 22.0,
        world.y + 68.0,
        2.0,
        structure,
    );
    draw_line(
        world.x + 46.0,
        world.y + 74.0,
        world.right() - 46.0,
        world.y + 74.0,
        1.0,
        structure,
    );
    for index in 0..6 {
        let fraction = index as f32 / 6.0;
        let x = world.x + 54.0 + (world.w - 108.0) * fraction;
        draw_line(x, world.y + 22.0, x + 30.0, floor - 18.0, 2.0, deep);
        draw_line(
            x + 14.0,
            world.y + 24.0,
            x + 44.0,
            floor - 18.0,
            1.0,
            structure,
        );
    }
    for index in 0..5 {
        let x = world.x + 70.0 + (world.w - 140.0) * index as f32 / 4.0;
        let cable_length = 24.0 + (index % 2) as f32 * 18.0;
        draw_line(
            x,
            world.y + 82.0,
            x,
            world.y + 82.0 + cable_length,
            2.0,
            structure,
        );
        draw_circle(x, world.y + 82.0 + cable_length, 3.0, visual_theme::amber());
    }
    draw_rectangle(
        world.x + 70.0,
        world.y + 60.0,
        (world.w * 0.16).min(150.0),
        5.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.72),
    );
    draw_rectangle(
        world.right() - (world.w * 0.2).min(180.0) - 70.0,
        world.y + 60.0,
        (world.w * 0.2).min(180.0),
        5.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.72),
    );
    draw_line(
        world.x + 22.0,
        floor,
        world.right() - 22.0,
        floor,
        2.0,
        structure,
    );
    draw_line(
        world.x + 22.0,
        floor + 30.0,
        world.right() - 22.0,
        floor + 30.0,
        1.0,
        deep,
    );
    for index in 0..12 {
        let x = world.x + 42.0 + (world.w - 84.0) * index as f32 / 11.0;
        draw_line(x, floor + 22.0, x + 30.0, floor + 22.0, 3.0, deep);
        draw_line(
            x + 39.0,
            floor + 22.0,
            x + 51.0,
            floor + 22.0,
            3.0,
            structure,
        );
    }
    draw_service_cart(world.x + 44.0, floor - 54.0);
    draw_line(
        world.x + 120.0,
        floor - 2.0,
        world.x + 120.0,
        floor - 28.0,
        2.0,
        visual_theme::amber(),
    );
    draw_line(
        world.right() - 106.0,
        floor - 2.0,
        world.right() - 106.0,
        floor - 28.0,
        2.0,
        visual_theme::amber(),
    );
}

fn draw_service_cart(x: f32, y: f32) {
    draw_rectangle(
        x,
        y,
        70.0,
        28.0,
        visual_theme::with_alpha(visual_theme::structure_dark(), 0.92),
    );
    draw_rectangle_lines(x, y, 70.0, 28.0, 1.0, visual_theme::structure_light());
    draw_rectangle(x + 8.0, y + 7.0, 22.0, 5.0, visual_theme::amber());
    draw_rectangle(x + 38.0, y + 7.0, 20.0, 5.0, visual_theme::cyan_dim());
    draw_circle(x + 12.0, y + 31.0, 4.0, visual_theme::structure_light());
    draw_circle(x + 58.0, y + 31.0, 4.0, visual_theme::structure_light());
}
