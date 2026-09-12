//! Local design-space strokes for the procedural ship artwork.

use macroquad::prelude::*;

pub(super) struct ShipInk {
    pub rect: Rect,
    pub size: Vec2,
    pub opacity: f32,
}

impl ShipInk {
    pub fn point(&self, p: (f32, f32)) -> Vec2 {
        vec2(
            self.rect.x + p.0 * self.rect.w / self.size.x,
            self.rect.y + p.1 * self.rect.h / self.size.y,
        )
    }

    fn color(&self, color: Color) -> Color {
        Color::new(color.r, color.g, color.b, color.a * self.opacity)
    }

    pub fn scale(&self) -> f32 {
        (self.rect.w / self.size.x).min(self.rect.h / self.size.y)
    }

    pub fn line(&self, from: (f32, f32), to: (f32, f32), width: f32, color: Color) {
        let a = self.point(from);
        let b = self.point(to);
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            (width * self.scale()).max(0.65),
            self.color(color),
        );
    }

    pub fn panel(&self, bounds: (f32, f32, f32, f32), color: Color) {
        let p = self.point((bounds.0, bounds.1));
        draw_rectangle(
            p.x,
            p.y,
            bounds.2 * self.rect.w / self.size.x,
            bounds.3 * self.rect.h / self.size.y,
            self.color(color),
        );
    }

    pub fn polygon(&self, points: &[(f32, f32)], color: Color) {
        // Ship panels are convex, so a triangle fan preserves their bevels.
        for index in 1..points.len().saturating_sub(1) {
            draw_triangle(
                self.point(points[0]),
                self.point(points[index]),
                self.point(points[index + 1]),
                self.color(color),
            );
        }
    }

    pub fn disc(&self, center: (f32, f32), radius: f32, color: Color) {
        let p = self.point(center);
        draw_circle(p.x, p.y, radius * self.scale(), self.color(color));
    }

    pub fn ring(&self, center: (f32, f32), radius: f32, color: Color) {
        let p = self.point(center);
        draw_circle_lines(
            p.x,
            p.y,
            radius * self.scale(),
            self.scale().max(0.7),
            self.color(color),
        );
    }

    pub fn bolts(&self, bounds: (f32, f32, f32, f32), color: Color) {
        let (x, y, w, h) = bounds;
        for p in [(x, y), (x + w, y), (x, y + h), (x + w, y + h)] {
            self.disc(p, 1.8, color);
        }
    }
}
