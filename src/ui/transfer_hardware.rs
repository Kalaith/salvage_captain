//! Transfer hardware attached to the workboat for the active salvage mode.

use super::visual_theme;
use crate::state::workspace::TransferMode;
use macroquad::prelude::*;

pub fn transfer_point(rect: Rect, mode: TransferMode) -> Vec2 {
    match mode {
        TransferMode::InternalCargo => vec2(rect.x + rect.w * 0.94, rect.y + rect.h * 0.46),
        TransferMode::ExternalClamp => vec2(rect.x + rect.w * 0.5, rect.bottom() + 25.0),
        TransferMode::Tow => vec2(rect.x - 4.0, rect.y + rect.h * 0.46),
    }
}

pub fn draw_transfer_hardware(rect: Rect, mode: TransferMode, elapsed: f32) {
    let point = transfer_point(rect, mode);
    match mode {
        TransferMode::InternalCargo => {
            draw_rectangle_lines(
                point.x - 18.0,
                point.y - 12.0,
                36.0,
                24.0,
                2.0,
                visual_theme::cyan(),
            );
            draw_line(
                point.x - 27.0,
                point.y,
                point.x - 18.0,
                point.y,
                3.0,
                visual_theme::cyan(),
            );
        }
        TransferMode::ExternalClamp => {
            let hull_bottom = rect.y + rect.h * 0.72;
            draw_line(
                point.x,
                hull_bottom,
                point.x,
                point.y - 10.0,
                3.0,
                visual_theme::amber(),
            );
            draw_circle_lines(point.x, point.y, 9.0, 2.0, visual_theme::amber());
            draw_line(
                point.x - 16.0,
                point.y - 8.0,
                point.x - 5.0,
                point.y + 3.0,
                3.0,
                visual_theme::amber(),
            );
            draw_line(
                point.x + 16.0,
                point.y - 8.0,
                point.x + 5.0,
                point.y + 3.0,
                3.0,
                visual_theme::amber(),
            );
        }
        TransferMode::Tow => {
            let pulse = 10.0 + (elapsed * 3.0).sin().abs() * 3.0;
            draw_circle_lines(point.x, point.y, pulse, 2.0, visual_theme::warning());
            draw_line(
                point.x - 10.0,
                point.y - 8.0,
                point.x - 32.0,
                point.y - 8.0,
                3.0,
                visual_theme::warning(),
            );
            draw_line(
                point.x - 10.0,
                point.y + 8.0,
                point.x - 32.0,
                point.y + 8.0,
                3.0,
                visual_theme::warning(),
            );
        }
    }
}
