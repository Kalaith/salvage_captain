//! The orbital view beyond the hangar opening.

use super::*;

pub(super) fn draw_orbit() {
    for index in 0..40 {
        let brightness = index as f32 / 40.0;
        draw_rectangle(
            650.0,
            80.0 + index as f32 * 12.0,
            630.0,
            12.0,
            Color::new(
                0.023 + brightness * 0.012,
                0.048 + brightness * 0.034,
                0.079 + brightness * 0.045,
                1.0,
            ),
        );
    }
    for index in 0..180 {
        let x = 653.0 + ((index * 197) % 600) as f32;
        let y = 84.0 + ((index * 79) % 453) as f32;
        draw_circle(
            x,
            y,
            if index % 11 == 0 { 1.2 } else { 0.55 },
            visual_theme::with_alpha(visual_theme::text(), 0.15 + (index % 6) as f32 * 0.07),
        );
    }
    draw_planet();
    for (x, y, scale) in [
        (1170.0, 302.0, 1.0),
        (997.0, 411.0, 0.72),
        (1228.0, 488.0, 0.9),
    ] {
        draw_station(vec2(x, y), scale);
    }
}

fn draw_planet() {
    let center = vec2(651.0, 480.0);
    let radius = 351.0;
    for index in (1..9).rev() {
        draw_circle(
            center.x,
            center.y,
            radius + index as f32 * 1.4,
            visual_theme::with_alpha(visual_theme::cyan(), 0.012),
        );
    }
    const RINGS: usize = 24;
    const SEGMENTS: usize = 64;
    // Submit bounded annuli so the default WebGL batch never truncates the globe.
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(2 * (SEGMENTS + 1)),
        indices: Vec::with_capacity(SEGMENTS * 6),
        texture: None,
    };
    for ring in 0..RINGS {
        mesh.vertices.clear();
        mesh.indices.clear();
        for row in 0..=1 {
            let distance = (ring + row) as f32 / RINGS as f32;
            for segment in 0..=SEGMENTS {
                let angle = segment as f32 / SEGMENTS as f32 * std::f32::consts::TAU;
                let nx = distance * angle.cos();
                let ny = distance * angle.sin();
                let nz = (1.0 - distance * distance).max(0.0).sqrt();
                let sun = (-nx * 0.25 - ny * 0.65 + nz * 0.5).max(0.0);
                let cloud = ((nx * 21.0 + (ny * 17.0).sin() * 2.4).sin()
                    + (ny * 36.0 + (nx * 12.0).cos() * 3.0).sin())
                    * 0.5;
                let land = ((nx * 13.0 + ny * 5.0).sin() * (ny * 11.0 - nx * 7.0).cos()).max(0.0);
                let ice = cloud.max(0.0) * 0.24;
                let color = Color::new(
                    0.025 + sun * (0.075 + land * 0.10 + ice),
                    0.062 + sun * (0.19 + land * 0.10 + ice),
                    0.10 + sun * (0.28 + land * 0.07 + ice),
                    1.0,
                );
                mesh.vertices.push(Vertex::new(
                    center.x + nx * radius,
                    center.y + ny * radius,
                    0.0,
                    0.0,
                    0.0,
                    color,
                ));
                if row == 0 && segment < SEGMENTS {
                    let a = segment as u16;
                    let b = a + (SEGMENTS + 1) as u16;
                    mesh.indices
                        .extend_from_slice(&[a, b, a + 1, a + 1, b, b + 1]);
                }
            }
        }
        draw_mesh(&mesh);
    }
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        1.5,
        visual_theme::with_alpha(visual_theme::cyan(), 0.25),
    );
}

fn draw_station(origin: Vec2, scale: f32) {
    let (x, y) = (origin.x, origin.y);
    let dark = Color::new(0.035, 0.06, 0.08, 1.0);
    let edge = Color::new(0.15, 0.22, 0.26, 1.0);
    draw_rectangle(x - 126.0 * scale, y, 240.0 * scale, 38.0 * scale, dark);
    draw_rectangle(
        x - 142.0 * scale,
        y + 7.0 * scale,
        270.0 * scale,
        12.0 * scale,
        dark,
    );
    for index in 0..3 {
        let tx = x - 89.0 * scale + index as f32 * 92.0 * scale;
        let height = (93.0 - index as f32 * 16.0) * scale;
        draw_rectangle(tx, y - height, 15.0 * scale, height, dark);
        draw_line(
            tx + 4.0 * scale,
            y - height + 6.0,
            tx + 4.0 * scale,
            y,
            2.0,
            edge,
        );
        draw_line(
            tx + 7.0 * scale,
            y - height - 23.0 * scale,
            tx + 7.0 * scale,
            y - height,
            1.0,
            edge,
        );
        draw_rectangle(
            tx - 6.0 * scale,
            y - 21.0 * scale,
            27.0 * scale,
            8.0 * scale,
            dark,
        );
        draw_rectangle(
            tx + 6.0 * scale,
            y - height + 12.0,
            2.0,
            9.0 * scale,
            visual_theme::with_alpha(visual_theme::amber(), 0.5),
        );
    }
    draw_line(x - 126.0 * scale, y, x + 114.0 * scale, y, 2.0, edge);
    for index in 0..24 {
        let wx = x - 117.0 * scale + index as f32 * 9.0 * scale;
        draw_rectangle(
            wx,
            y + 8.0 * scale,
            3.0 * scale,
            2.0,
            visual_theme::with_alpha(
                if index % 4 == 0 {
                    visual_theme::amber()
                } else {
                    visual_theme::cyan()
                },
                0.4,
            ),
        );
    }
    for index in 0..7 {
        let bx = x - 115.0 * scale + index as f32 * 34.0 * scale;
        draw_rectangle_lines(bx, y + 22.0 * scale, 26.0 * scale, 11.0 * scale, 1.0, edge);
    }
}
