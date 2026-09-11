//! Compact survey-drone visuals for a scanned wreck section.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::data::GameData;
use crate::state::GameSession;
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

pub fn draw_deployed_drones(
    layout: SalvageLayout,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    selected_target: Option<&str>,
) {
    if !session.workspace_drones_deployed() {
        return;
    }
    let count = visible_drone_count(session.module_stats(data).drone_support);
    if count == 0 {
        return;
    }
    let anchor = selected_target
        .and_then(|target_id| layout.target_rect(target_id))
        .map_or(
            vec2(
                layout.wreck.x + layout.wreck.w * 0.52,
                layout.wreck.y + layout.wreck.h * 0.42,
            ),
            |rect| rect.center(),
        );
    for index in 0..count {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let drift = (elapsed * 1.8 + index as f32 * 2.1).sin() * 12.0;
        let position = vec2(
            anchor.x + side * (70.0 + drift),
            anchor.y - 52.0 + (elapsed * 1.4 + index as f32).cos() * 8.0,
        );
        draw_line(
            anchor.x,
            anchor.y,
            position.x,
            position.y,
            1.0,
            visual_theme::with_alpha(visual_theme::cyan(), 0.42),
        );
        draw_drone(position, elapsed, index);
    }
    draw_text(
        format!("DRONE MESH  //  {} ACTIVE", count),
        layout.wreck.x + 22.0,
        layout.wreck.y + 31.0,
        10.0,
        visual_theme::cyan(),
    );
}

fn visible_drone_count(drone_support: i32) -> usize {
    drone_support.clamp(0, 2) as usize
}

fn draw_drone(position: Vec2, elapsed: f32, index: usize) {
    let body = Rect::new(position.x - 16.0, position.y - 9.0, 32.0, 18.0);
    draw_rectangle(
        body.x,
        body.y,
        body.w,
        body.h,
        visual_theme::structure_dark(),
    );
    draw_rectangle_lines(body.x, body.y, body.w, body.h, 1.5, visual_theme::cyan());
    let blink = 0.62 + (elapsed * 4.0 + index as f32).sin().abs() * 0.38;
    draw_circle(
        position.x,
        position.y,
        4.0,
        visual_theme::with_alpha(visual_theme::amber(), blink),
    );
    for side in [-1.0, 1.0] {
        draw_line(
            position.x + side * 11.0,
            position.y - 6.0,
            position.x + side * 21.0,
            position.y - 13.0,
            2.0,
            visual_theme::structure_light(),
        );
        draw_circle(
            position.x + side * 22.0,
            position.y - 14.0,
            3.0,
            visual_theme::cyan_dim(),
        );
    }
}
