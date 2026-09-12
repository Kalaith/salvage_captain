//! Restrained scan pulse and target brackets over the wreck art.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::state::WorkspaceScanProfile;
use macroquad::prelude::*;

pub fn draw_scan_overlay(
    layout: SalvageLayout,
    elapsed: f32,
    pulse_progress: f32,
    scanned: bool,
    revealed_targets: &[&String],
    profile: WorkspaceScanProfile,
) {
    if pulse_progress > 0.0 {
        let progress = pulse_progress.clamp(0.0, 1.0);
        let x = layout.wreck.x - 40.0 + progress * (layout.wreck.w + 80.0);
        for pulse in 0..profile.pulse_count() {
            let offset = pulse as f32 * 8.0;
            draw_line(
                x + offset,
                layout.wreck.y - 16.0,
                x - 20.0 + offset,
                layout.wreck.bottom() + 20.0,
                if pulse == 0 { 4.0 } else { 1.0 },
                visual_theme::with_alpha(
                    visual_theme::cyan(),
                    if pulse == 0 {
                        1.0 - progress * 0.5
                    } else {
                        0.5
                    },
                ),
            );
        }
    }
    if !scanned {
        return;
    }
    for target_id in revealed_targets {
        let Some(rect) = layout.target_rect(target_id) else {
            continue;
        };
        let alpha = 0.5 + (elapsed * 2.0).sin().abs() * 0.35;
        draw_brackets(rect, visual_theme::with_alpha(visual_theme::cyan(), alpha));
    }
}

fn draw_brackets(rect: Rect, color: Color) {
    let size = 12.0;
    let width = 2.0;
    draw_line(rect.x, rect.y, rect.x + size, rect.y, width, color);
    draw_line(rect.x, rect.y, rect.x, rect.y + size, width, color);
    draw_line(
        rect.right(),
        rect.y,
        rect.right() - size,
        rect.y,
        width,
        color,
    );
    draw_line(
        rect.right(),
        rect.y,
        rect.right(),
        rect.y + size,
        width,
        color,
    );
    draw_line(
        rect.x,
        rect.bottom(),
        rect.x + size,
        rect.bottom(),
        width,
        color,
    );
    draw_line(
        rect.x,
        rect.bottom(),
        rect.x,
        rect.bottom() - size,
        width,
        color,
    );
    draw_line(
        rect.right(),
        rect.bottom(),
        rect.right() - size,
        rect.bottom(),
        width,
        color,
    );
    draw_line(
        rect.right(),
        rect.bottom(),
        rect.right(),
        rect.bottom() - size,
        width,
        color,
    );
}
