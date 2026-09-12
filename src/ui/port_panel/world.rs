//! Layered dock architecture, work lights and foreground floor details.

use super::*;
mod vista;

pub(super) fn draw_hangar_world(world: Rect, copy: &crate::data::port_ui::PortUiCopy) {
    draw_rectangle(world.x, world.y, world.w, world.h, visual_theme::space());
    vista::draw_orbit();
    draw_bulkhead(copy);
    draw_floor();
    draw_roof();
    draw_workshop();
    // Foreground falloff anchors the transparent action dock without a solid rail.
    for index in 0..24 {
        let y = 564.0 + index as f32 * 5.0;
        draw_rectangle(
            0.0,
            y,
            1280.0,
            5.0,
            visual_theme::with_alpha(BLACK, index as f32 * 0.026),
        );
    }
}

fn draw_bulkhead(copy: &crate::data::port_ui::PortUiCopy) {
    draw_rectangle(
        0.0,
        68.0,
        650.0,
        480.0,
        Color::new(0.055, 0.072, 0.083, 1.0),
    );
    for row in 0..5 {
        for column in 0..7 {
            let x = column as f32 * 92.0 + 10.0;
            let y = row as f32 * 88.0 + 106.0;
            let tint = ((row * 3 + column * 7) % 5) as f32 * 0.004;
            draw_rectangle(
                x,
                y,
                88.0,
                83.0,
                Color::new(0.07 + tint, 0.085 + tint, 0.095 + tint, 1.0),
            );
            draw_line(
                x,
                y,
                x + 87.0,
                y,
                1.0,
                visual_theme::with_alpha(visual_theme::structure_light(), 0.15),
            );
            for dx in [5.0, 81.0] {
                for dy in [5.0, 77.0] {
                    draw_circle(
                        x + dx,
                        y + dy,
                        1.1,
                        visual_theme::with_alpha(visual_theme::structure_light(), 0.28),
                    );
                }
            }
            for scratch in 0..4 {
                let sx = x + ((column * 17 + row * 13 + scratch * 23) % 76) as f32;
                let sy = y + 10.0 + ((scratch * 19 + column * 7) % 64) as f32;
                draw_line(
                    sx,
                    sy,
                    sx + 5.0,
                    sy - 1.0,
                    1.0,
                    visual_theme::with_alpha(visual_theme::structure_light(), 0.10),
                );
            }
        }
    }
    for x in [20.0, 332.0, 616.0, 1230.0] {
        draw_rectangle(x, 100.0, 30.0, 447.0, Color::new(0.03, 0.044, 0.054, 1.0));
        draw_rectangle(x + 4.0, 100.0, 5.0, 447.0, visual_theme::structure_dark());
        draw_line(
            x + 27.0,
            110.0,
            x + 27.0,
            540.0,
            2.0,
            visual_theme::structure(),
        );
        for y in [170.0, 326.0, 478.0] {
            draw_rectangle(x - 4.0, y, 38.0, 12.0, visual_theme::structure_dark());
            light(Rect::new(x + 12.0, y + 26.0, 4.0, 22.0));
        }
    }
    let stencil = Color::new(0.30, 0.33, 0.35, 0.8);
    visual_theme::body(
        &copy.hangar_label,
        Rect::new(68.0, 156.0, 220.0, 30.0),
        27.0,
        stencil,
    );
    visual_theme::body(
        &copy.ship_id,
        Rect::new(64.0, 183.0, 248.0, 78.0),
        82.0,
        stencil,
    );
    visual_theme::body(
        &copy.hangar_motto,
        Rect::new(70.0, 266.0, 214.0, 54.0),
        18.0,
        stencil,
    );
    draw_rectangle(1249.0, 112.0, 31.0, 436.0, visual_theme::structure_dark());
    for y in (150..500).step_by(30) {
        draw_line(
            1256.0,
            y as f32,
            1277.0,
            y as f32 - 16.0,
            5.0,
            visual_theme::with_alpha(visual_theme::amber(), 0.24),
        );
    }
}

fn draw_roof() {
    draw_rectangle(
        0.0,
        68.0,
        1280.0,
        40.0,
        Color::new(0.025, 0.036, 0.046, 1.0),
    );
    for y in [73.0, 88.0, 103.0] {
        draw_line(
            0.0,
            y,
            1280.0,
            y + 14.0,
            5.0,
            visual_theme::structure_dark(),
        );
        draw_line(
            0.0,
            y - 2.0,
            1280.0,
            y + 12.0,
            1.0,
            visual_theme::structure(),
        );
    }
    for index in 0..12 {
        let x = index as f32 * 114.0;
        draw_line(
            x,
            70.0,
            x + 55.0,
            112.0,
            5.0,
            Color::new(0.03, 0.05, 0.065, 1.0),
        );
        draw_circle(x + 53.0, 108.0, 2.0, visual_theme::structure_light());
    }
    for x in [278.0, 494.0, 734.0, 968.0, 1174.0] {
        draw_rectangle(x - 7.0, 105.0, 64.0, 15.0, visual_theme::structure_dark());
        light(Rect::new(x, 110.0, 48.0, 4.0));
        draw_triangle(
            vec2(x, 117.0),
            vec2(x - 85.0, 330.0),
            vec2(x + 128.0, 330.0),
            visual_theme::with_alpha(visual_theme::amber(), 0.023),
        );
    }
    draw_crane();
}

fn draw_crane() {
    for index in 0..6 {
        let start = 250.0 + index as f32 * 74.0;
        let depth = 28.0 + (index % 3) as f32 * 20.0;
        for segment in 0..16 {
            let t = segment as f32 / 16.0;
            let next = (segment + 1) as f32 / 16.0;
            draw_line(
                start + t * 64.0,
                106.0 + (t * std::f32::consts::PI).sin() * depth,
                start + next * 64.0,
                106.0 + (next * std::f32::consts::PI).sin() * depth,
                3.0,
                BLACK,
            );
        }
    }
    draw_rectangle(512.0, 70.0, 42.0, 86.0, visual_theme::structure_dark());
    draw_rectangle_lines(512.0, 70.0, 42.0, 86.0, 2.0, visual_theme::structure());
    draw_rectangle(521.0, 78.0, 24.0, 63.0, Color::new(0.07, 0.08, 0.085, 1.0));
    for x in [518.0, 550.0] {
        draw_line(x, 153.0, x, 205.0, 3.0, visual_theme::structure());
    }
    draw_rectangle(516.0, 198.0, 40.0, 12.0, visual_theme::structure_dark());
    light(Rect::new(528.0, 210.0, 16.0, 5.0));
    draw_triangle(
        vec2(536.0, 216.0),
        vec2(479.0, 349.0),
        vec2(593.0, 349.0),
        visual_theme::with_alpha(visual_theme::amber(), 0.05),
    );
    for (x, direction) in [(0.0, 1.0), (1280.0, -1.0)] {
        draw_line(
            x,
            203.0,
            x + direction * 125.0,
            92.0,
            22.0,
            visual_theme::structure_dark(),
        );
        draw_line(
            x,
            196.0,
            x + direction * 125.0,
            85.0,
            2.0,
            visual_theme::structure(),
        );
    }
}

fn draw_floor() {
    draw_rectangle(
        0.0,
        548.0,
        1280.0,
        136.0,
        Color::new(0.045, 0.063, 0.078, 1.0),
    );
    for y in [550.0, 556.0, 568.0, 589.0, 621.0, 671.0] {
        draw_line(0.0, y, 1280.0, y, 2.0, Color::new(0.015, 0.027, 0.038, 1.0));
        draw_line(
            0.0,
            y + 2.0,
            1280.0,
            y + 2.0,
            1.0,
            visual_theme::with_alpha(visual_theme::structure_light(), 0.18),
        );
    }
    for index in -9..16 {
        let x = index as f32 * 100.0;
        draw_line(
            640.0 + (x - 640.0) * 0.45,
            548.0,
            x,
            684.0,
            1.0,
            visual_theme::with_alpha(BLACK, 0.7),
        );
    }
    for index in 0..100 {
        let x = ((index * 137) % 1270) as f32;
        let y = 552.0 + ((index * 29) % 124) as f32;
        let reflected = index % 4 == 0;
        draw_line(
            x,
            y,
            x + (index % 13 + 3) as f32,
            y,
            1.0,
            visual_theme::with_alpha(
                if reflected {
                    visual_theme::amber()
                } else {
                    visual_theme::structure_light()
                },
                if reflected { 0.24 } else { 0.09 },
            ),
        );
    }
    for index in 0..24 {
        let x = 80.0 + index as f32 * 49.0;
        draw_line(
            x,
            557.0,
            x + 21.0,
            557.0,
            2.0,
            visual_theme::with_alpha(visual_theme::amber(), 0.42),
        );
    }
}

fn draw_workshop() {
    for (x, y, w, h) in [
        (4.0, 524.0, 71.0, 56.0),
        (79.0, 564.0, 90.0, 43.0),
        (1141.0, 539.0, 78.0, 53.0),
        (1217.0, 503.0, 63.0, 87.0),
        (1086.0, 575.0, 57.0, 35.0),
    ] {
        draw_rectangle(x, y, w, h, Color::new(0.025, 0.039, 0.048, 1.0));
        draw_rectangle_lines(
            x + 3.0,
            y + 3.0,
            w - 6.0,
            h - 6.0,
            2.0,
            visual_theme::structure_dark(),
        );
        for dx in [w * 0.2, w * 0.8] {
            draw_rectangle(x + dx, y, 5.0, h, Color::new(0.085, 0.105, 0.115, 1.0));
        }
        draw_rectangle(
            x + w * 0.4,
            y + 9.0,
            w * 0.2,
            4.0,
            visual_theme::with_alpha(visual_theme::text_dim(), 0.35),
        );
    }
    for x in [187.0, 425.0, 770.0, 1044.0] {
        draw_rectangle(x, 529.0, 26.0, 19.0, visual_theme::structure_dark());
        draw_rectangle_lines(x, 529.0, 26.0, 19.0, 1.0, visual_theme::structure());
        light(Rect::new(x + 7.0, 532.0, 11.0, 3.0));
    }
    draw_deck_crew();
}

fn draw_deck_crew() {
    // Tiny deck crew and a cyan diagnostic terminal establish the vessel's scale.
    for x in [174.0, 800.0, 1078.0] {
        draw_circle(x, 512.0, 4.0, visual_theme::amber());
        draw_line(x, 517.0, x, 533.0, 6.0, visual_theme::structure());
        draw_line(
            x - 2.0,
            531.0,
            x - 4.0,
            548.0,
            3.0,
            visual_theme::structure_dark(),
        );
        draw_line(
            x + 2.0,
            531.0,
            x + 5.0,
            548.0,
            3.0,
            visual_theme::structure_dark(),
        );
    }
    draw_rectangle(141.0, 526.0, 18.0, 10.0, visual_theme::cyan_dim());
    draw_rectangle(144.0, 528.0, 12.0, 2.0, visual_theme::cyan());
    draw_line(150.0, 536.0, 150.0, 548.0, 3.0, visual_theme::structure());
}

fn light(rect: Rect) {
    for index in (1..5).rev() {
        let spread = index as f32 * 3.0;
        draw_rectangle(
            rect.x - spread,
            rect.y - spread,
            rect.w + spread * 2.0,
            rect.h + spread * 2.0,
            visual_theme::with_alpha(visual_theme::amber(), 0.018),
        );
    }
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::amber());
    draw_rectangle(
        rect.x + 1.0,
        rect.y,
        rect.w - 2.0,
        rect.h * 0.5,
        Color::new(1.0, 0.84, 0.54, 1.0),
    );
}
