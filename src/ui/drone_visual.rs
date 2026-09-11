//! Compact survey-drone visuals for a scanned wreck section.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::data::GameData;
use crate::state::{DroneDirective, GameSession};
use macroquad::prelude::*;


pub fn draw_deployed_drones(
    layout: SalvageLayout,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    selected_target: Option<&str>,
    extraction_target: Option<&str>,
) {
    if !session.workspace_drones_deployed() {
        return;
    }
    let count = visible_drone_count(session.module_stats(data).drone_support);
    if count == 0 {
        return;
    }
    let anchor = extraction_target
        .or(selected_target)
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
        let tether_color = if extraction_target.is_some() {
            visual_theme::amber()
        } else {
            visual_theme::cyan()
        };
        draw_line(
            anchor.x,
            anchor.y,
            position.x,
            position.y,
            1.0,
            visual_theme::with_alpha(tether_color, 0.48),
        );
        let signal = anchor + (position - anchor) * drone_signal_progress(elapsed, index);
        draw_circle(
            signal.x,
            signal.y,
            2.5,
            visual_theme::with_alpha(tether_color, 0.9),
        );
        draw_drone(position, elapsed, index);
    }
    if extraction_target.is_some() {
        let pulse = 24.0 + (elapsed * 4.0).sin().abs() * 8.0;
        draw_circle_lines(anchor.x, anchor.y, pulse, 2.0, visual_theme::amber());
    }
    draw_text(
        format!(
            "{}  //  {}",
            drone_operation_label(
                extraction_target.is_some(),
                session.workspace_drone_directive(),
            ),
            count
        ),
        layout.wreck.x + 22.0,
        layout.wreck.y + 31.0,
        10.0,
        visual_theme::cyan(),
    );
}

fn drone_operation_label(extraction_active: bool, directive: DroneDirective) -> &'static str {
    match (extraction_active, directive) {
        (true, DroneDirective::PullSupport) => "DRONE MESH  //  PULL ASSIST",
        (true, DroneDirective::Survey) => "DRONE MESH  //  SURVEY COVER",
        (false, DroneDirective::PullSupport) => "DRONE MESH  //  PULL READY",
        (false, DroneDirective::Survey) => "DRONE MESH  //  SURVEY NET",
        (_, DroneDirective::Standby) => "DRONE MESH  //  STANDBY",
    }
}

fn visible_drone_count(drone_support: i32) -> usize {
    drone_support.clamp(0, 2) as usize
}

fn drone_signal_progress(elapsed: f32, index: usize) -> f32 {
    (elapsed * 0.65 + index as f32 * 0.5).fract()
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
