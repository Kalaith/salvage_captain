//! Hazard-specific motion and warning marks on an active salvage mount.

use super::visual_theme;
use crate::engine::WorkspaceHazard;
use macroquad::prelude::*;

pub fn draw_target_hazard(rect: Rect, hazard_value: &str, elapsed: f32, progress: f32) {
    let Some(hazard) = WorkspaceHazard::from_value(hazard_value) else {
        return;
    };
    let center = rect.center();
    match hazard {
        WorkspaceHazard::ReactorInstability => draw_reactor_instability(rect, center, elapsed),
        WorkspaceHazard::ElectricalArcs => draw_electrical_arcs(rect, center, elapsed),
        WorkspaceHazard::AutomatedDefenses => draw_automated_defenses(center, elapsed),
        WorkspaceHazard::UnexplodedAmmunition => draw_unexploded_ammunition(rect, elapsed),
        WorkspaceHazard::MagneticInterference => {
            draw_magnetic_interference(rect, center, elapsed, progress)
        }
        WorkspaceHazard::StructuralCollapse => draw_structural_collapse(rect, center, elapsed),
    }
}

fn draw_reactor_instability(rect: Rect, center: Vec2, elapsed: f32) {
    let pulse = rect.h * (0.36 + (elapsed * 5.0).sin().abs() * 0.12);
    draw_circle_lines(center.x, center.y, pulse, 3.0, visual_theme::warning());
    draw_line(
        center.x - pulse,
        center.y,
        center.x - pulse - 12.0,
        center.y + 8.0,
        2.0,
        visual_theme::warning(),
    );
}

fn draw_electrical_arcs(rect: Rect, center: Vec2, elapsed: f32) {
    for index in 0..3 {
        let x = rect.x + rect.w * (0.2 + index as f32 * 0.3);
        let offset = (elapsed * 9.0 + index as f32).sin() * 8.0;
        draw_line(
            x,
            rect.y + 10.0,
            x - 7.0,
            center.y + offset,
            2.0,
            visual_theme::cyan(),
        );
        draw_line(
            x - 7.0,
            center.y + offset,
            x + 6.0,
            rect.bottom() - 10.0,
            2.0,
            visual_theme::cyan(),
        );
    }
}

fn draw_automated_defenses(center: Vec2, elapsed: f32) {
    let radius = 16.0 + (elapsed * 4.0).sin().abs() * 8.0;
    draw_circle_lines(center.x, center.y, radius, 2.0, visual_theme::warning());
    draw_line(
        center.x - radius - 8.0,
        center.y,
        center.x + radius + 8.0,
        center.y,
        2.0,
        visual_theme::warning(),
    );
    draw_line(
        center.x,
        center.y - radius - 8.0,
        center.x,
        center.y + radius + 8.0,
        2.0,
        visual_theme::warning(),
    );
}

fn draw_unexploded_ammunition(rect: Rect, elapsed: f32) {
    let blink = (elapsed * 5.0).sin().abs();
    draw_circle(
        rect.right() - 16.0,
        rect.y + 16.0,
        5.0 + blink * 3.0,
        visual_theme::warning(),
    );
    draw_text(
        "!",
        rect.right() - 18.0,
        rect.y + 20.0,
        10.0,
        visual_theme::panel(),
    );
}

fn draw_magnetic_interference(rect: Rect, center: Vec2, elapsed: f32, progress: f32) {
    let rotation = elapsed * 2.2 + progress * 4.0;
    for index in 0..2 {
        let radius = rect.h * (0.28 + index as f32 * 0.12);
        draw_circle_lines(center.x, center.y, radius, 2.0, visual_theme::cyan_dim());
        draw_line(
            center.x,
            center.y,
            center.x + rotation.cos() * radius,
            center.y + rotation.sin() * radius,
            2.0,
            visual_theme::cyan(),
        );
    }
}

fn draw_structural_collapse(rect: Rect, center: Vec2, elapsed: f32) {
    for index in 0..3 {
        let y = rect.y + 18.0 + index as f32 * rect.h * 0.28;
        let jitter = (elapsed * 7.0 + index as f32).sin() * 7.0;
        draw_line(
            rect.x + 14.0,
            y,
            center.x + jitter,
            y + 10.0,
            3.0,
            visual_theme::warning(),
        );
        draw_line(
            center.x + jitter,
            y + 10.0,
            rect.right() - 12.0,
            y - 6.0,
            2.0,
            visual_theme::warning(),
        );
    }
}

pub fn draw_stabilization_lock(rect: Rect, elapsed: f32) {
    let center = rect.center();
    let pulse = 0.8 + (elapsed * 3.5).sin().abs() * 0.2;
    let lock_color = visual_theme::with_alpha(visual_theme::safe(), pulse);
    draw_rectangle_lines(
        rect.x - 6.0,
        rect.y - 6.0,
        rect.w + 12.0,
        rect.h + 12.0,
        2.0,
        lock_color,
    );
    draw_circle_lines(center.x, center.y, rect.h * 0.28, 2.0, lock_color);
    draw_line(
        center.x - 14.0,
        center.y,
        center.x + 14.0,
        center.y,
        2.0,
        lock_color,
    );
    draw_line(
        center.x,
        center.y - 14.0,
        center.x,
        center.y + 14.0,
        2.0,
        lock_color,
    );
}
