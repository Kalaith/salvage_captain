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
        WorkspaceHazard::ReactorInstability => {
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
        WorkspaceHazard::ElectricalArcs => {
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
        WorkspaceHazard::AutomatedDefenses => {
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
        WorkspaceHazard::UnexplodedAmmunition => {
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
        WorkspaceHazard::MagneticInterference => {
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
        WorkspaceHazard::StructuralCollapse => {
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
    }
}
