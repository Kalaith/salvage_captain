//! Visual language for persistent wreck condition and section wear.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::state::workspace::WorkspaceConditionStatus;
use crate::state::GameSession;
use macroquad::prelude::*;

pub fn draw_condition_overlay(
    layout: SalvageLayout,
    condition: WorkspaceConditionStatus,
    theme: &str,
    elapsed: f32,
    section_targets: &[String],
    session: &GameSession,
) {
    draw_condition_wear(layout, condition, theme, elapsed, section_targets, session);
}

fn draw_condition_wear(
    layout: SalvageLayout,
    condition: WorkspaceConditionStatus,
    theme: &str,
    elapsed: f32,
    section_targets: &[String],
    session: &GameSession,
) {
    let wreck = layout.wreck;
    let stress = condition.structural_stress();
    let tone = condition_tone(condition);
    let crack_count = ((stress.saturating_sub(12) / 6).clamp(0, 4)) as usize;
    for index in 0..crack_count {
        let x = wreck.x + 56.0 + index as f32 * 112.0;
        let y = wreck.y + wreck.h * (0.34 + index as f32 * 0.08);
        let drift = (elapsed * 0.8 + index as f32).sin() * 3.0;
        draw_line(
            x,
            y,
            x + 18.0 + drift,
            y + 28.0,
            2.0 + stress as f32 * 0.02,
            visual_theme::with_alpha(tone, 0.7),
        );
        draw_line(
            x + 18.0 + drift,
            y + 28.0,
            x + 8.0,
            y + 50.0,
            1.5,
            visual_theme::with_alpha(tone, 0.62),
        );
    }
    if stress >= 24 {
        draw_line(
            wreck.x + 34.0,
            wreck.bottom() - 48.0,
            wreck.x + 118.0,
            wreck.bottom() - 62.0,
            3.0,
            visual_theme::with_alpha(tone, 0.74),
        );
        draw_line(
            wreck.right() - 176.0,
            wreck.y + 78.0,
            wreck.right() - 108.0,
            wreck.y + 126.0,
            2.0,
            visual_theme::with_alpha(tone, 0.68),
        );
    }
    for target_id in section_targets {
        if !session.target_is_removed(target_id) {
            continue;
        }
        let Some(rect) = layout.target_rect(target_id) else {
            continue;
        };
        let sway = (elapsed * 1.2 + rect.x).sin() * 3.0;
        draw_line(
            rect.x + rect.w * 0.28,
            rect.bottom(),
            rect.x + rect.w * 0.22 + sway,
            rect.bottom() + 14.0,
            2.0,
            visual_theme::with_alpha(tone, 0.75),
        );
        draw_line(
            rect.right() - rect.w * 0.2,
            rect.bottom(),
            rect.right() - rect.w * 0.16 - sway,
            rect.bottom() + 10.0,
            2.0,
            visual_theme::with_alpha(tone, 0.66),
        );
    }
    if theme == "military" && condition.section_condition <= 40 {
        draw_rectangle_lines(
            wreck.x + 22.0,
            wreck.y + 22.0,
            wreck.w - 44.0,
            wreck.h - 44.0,
            2.0,
            visual_theme::with_alpha(visual_theme::warning(), 0.72),
        );
    }
}

fn condition_tone(condition: WorkspaceConditionStatus) -> Color {
    match condition.label() {
        "CRITICAL" => visual_theme::warning(),
        "STRESSED" => visual_theme::amber(),
        "UNMAPPED" => visual_theme::cyan(),
        _ => visual_theme::safe(),
    }
}
