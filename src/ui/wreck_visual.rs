//! Close camera framing for a wreck section and its physical salvage mounts.

use super::condition_visual;
use super::hazard_visual;
use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::data::{GameData, SiteData};
use crate::engine::WorkspaceHazard;
use crate::state::workspace::{TransferMode, WorkspaceConditionStatus};
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::math::blink;
mod hazards;
pub(super) use hazards::{draw_target_mount, TargetMountView};

pub struct WreckView<'a> {
    pub layout: SalvageLayout,
    pub site: &'a SiteData,
    pub session: &'a GameSession,
    pub data: &'a GameData,
    pub elapsed: f32,
    pub scanned: bool,
    pub selected_target: Option<&'a str>,
    pub extraction_target: Option<&'a str>,
    pub extraction_progress: f32,
    pub section_targets: &'a [String],
    pub section_hazards: &'a [String],
    pub condition: WorkspaceConditionStatus,
}

pub fn draw_wreck(view: WreckView<'_>) {
    let WreckView {
        layout,
        site,
        session,
        data,
        elapsed,
        scanned,
        selected_target,
        extraction_target,
        extraction_progress,
        section_targets,
        section_hazards,
        condition,
    } = view;
    let wreck = layout.wreck;
    let accent = visual_theme::site_accent(&site.visual_theme);
    draw_rectangle(
        wreck.x - 18.0,
        wreck.y - 20.0,
        wreck.w + 36.0,
        wreck.h + 40.0,
        visual_theme::with_alpha(visual_theme::structure_dark(), 0.55),
    );
    draw_rectangle(
        wreck.x,
        wreck.y,
        wreck.w,
        wreck.h,
        visual_theme::structure(),
    );
    draw_rectangle(
        wreck.x + 14.0,
        wreck.y + 16.0,
        wreck.w - 28.0,
        wreck.h - 32.0,
        visual_theme::structure_dark(),
    );
    draw_line(
        wreck.x + 20.0,
        wreck.y + wreck.h * 0.25,
        wreck.right() - 20.0,
        wreck.y + wreck.h * 0.25,
        3.0,
        visual_theme::structure_light(),
    );
    draw_line(
        wreck.x + 20.0,
        wreck.y + wreck.h * 0.72,
        wreck.right() - 24.0,
        wreck.y + wreck.h * 0.72,
        2.0,
        visual_theme::structure_light(),
    );
    draw_line(
        wreck.x + wreck.w * 0.62,
        wreck.y + 12.0,
        wreck.x + wreck.w * 0.62,
        wreck.bottom() - 12.0,
        2.0,
        visual_theme::structure_light(),
    );
    draw_line(
        wreck.x + wreck.w * 0.18,
        wreck.y + 12.0,
        wreck.x + wreck.w * 0.28,
        wreck.bottom() - 20.0,
        2.0,
        visual_theme::structure_light(),
    );

    for index in 0..6 {
        let x = wreck.x + 38.0 + index as f32 * 72.0;
        let blink_on = blink(elapsed + index as f32 * 0.23, 0.65);
        draw_circle(
            x,
            wreck.y + 34.0,
            4.0,
            if blink_on {
                accent
            } else {
                visual_theme::structure_dark()
            },
        );
    }
    condition_visual::draw_condition_overlay(
        layout,
        condition,
        &site.visual_theme,
        elapsed,
        section_targets,
        session,
    );
    draw_damage(wreck, &site.visual_theme);
    draw_pipes(wreck, elapsed, &site.visual_theme);
    draw_theme_details(wreck, &site.visual_theme, elapsed);
    hazards::draw_hazard_details(wreck, section_hazards, elapsed);
    for target_id in section_targets {
        draw_target_mount(TargetMountView {
            layout,
            target_id,
            session,
            data,
            contract_target: site.contract_target.as_deref(),
            scanned,
            selected_target,
            extraction_target,
            extraction_progress,
            elapsed,
        });
    }
    draw_text(
        format!(
            "{}  //  {}",
            site.wreck_class.to_uppercase(),
            site.display_name.to_uppercase()
        ),
        wreck.x,
        wreck.bottom() + 24.0,
        13.0,
        visual_theme::text_dim(),
    );
}

fn draw_damage(wreck: Rect, theme: &str) {
    let cut = if theme == "military" {
        visual_theme::warning()
    } else {
        visual_theme::structure_dark()
    };
    draw_line(
        wreck.x + 18.0,
        wreck.y + wreck.h * 0.45,
        wreck.x + 110.0,
        wreck.y + wreck.h * 0.36,
        8.0,
        cut,
    );
    draw_line(
        wreck.right() - 130.0,
        wreck.y + wreck.h * 0.58,
        wreck.right() - 18.0,
        wreck.y + wreck.h * 0.68,
        7.0,
        cut,
    );
    draw_rectangle(
        wreck.x + wreck.w * 0.42,
        wreck.y + wreck.h * 0.42,
        54.0,
        28.0,
        visual_theme::space(),
    );
    draw_line(
        wreck.x + wreck.w * 0.44,
        wreck.y + wreck.h * 0.41,
        wreck.x + wreck.w * 0.51,
        wreck.y + wreck.h * 0.54,
        2.0,
        visual_theme::warning(),
    );
}

fn draw_pipes(wreck: Rect, elapsed: f32, theme: &str) {
    let color = if theme == "research" {
        visual_theme::cyan_dim()
    } else {
        visual_theme::structure_light()
    };
    for index in 0..4 {
        let x = wreck.x + 72.0 + index as f32 * 94.0;
        let sag = (elapsed * 0.7 + index as f32).sin() * 3.0;
        draw_line(
            x,
            wreck.y + 46.0,
            x + 26.0,
            wreck.y + 88.0 + sag,
            3.0,
            color,
        );
        draw_line(
            x + 26.0,
            wreck.y + 88.0 + sag,
            x + 18.0,
            wreck.y + 132.0,
            2.0,
            color,
        );
    }
}

fn draw_theme_details(wreck: Rect, theme: &str, elapsed: f32) {
    match theme {
        "military" => draw_military_details(wreck, elapsed),
        "research" => draw_research_details(wreck, elapsed),
        _ => draw_merchant_details(wreck),
    }
}

fn draw_merchant_details(wreck: Rect) {
    let cargo = visual_theme::with_alpha(visual_theme::amber(), 0.42);
    for index in 0..4 {
        let x = wreck.x + 34.0 + index as f32 * 66.0;
        let y = wreck.y + wreck.h * 0.83 - (index % 2) as f32 * 18.0;
        draw_rectangle(
            x,
            y,
            46.0,
            24.0,
            visual_theme::with_alpha(visual_theme::structure_dark(), 0.86),
        );
        draw_rectangle_lines(x, y, 46.0, 24.0, 2.0, cargo);
        draw_line(x + 8.0, y + 8.0, x + 38.0, y + 8.0, 2.0, cargo);
        draw_circle(x + 40.0, y + 18.0, 2.0, visual_theme::amber());
    }
    draw_line(
        wreck.x + 24.0,
        wreck.y + wreck.h * 0.79,
        wreck.right() - 30.0,
        wreck.y + wreck.h * 0.79,
        2.0,
        cargo,
    );
}

fn draw_military_details(wreck: Rect, elapsed: f32) {
    let armor = visual_theme::with_alpha(visual_theme::structure_light(), 0.72);
    for index in 0..3 {
        let x = wreck.x + 46.0 + index as f32 * 150.0;
        draw_line(
            x,
            wreck.y + 22.0,
            x + 54.0,
            wreck.bottom() - 34.0,
            8.0,
            armor,
        );
        draw_line(
            x + 54.0,
            wreck.y + 22.0,
            x,
            wreck.bottom() - 34.0,
            3.0,
            visual_theme::warning(),
        );
    }
    for index in 0..3 {
        let x = wreck.x + 92.0 + index as f32 * 164.0;
        let y = wreck.y + wreck.h * 0.63;
        draw_rectangle(x, y, 42.0, 16.0, visual_theme::structure_dark());
        draw_rectangle_lines(x, y, 42.0, 16.0, 2.0, visual_theme::warning());
        draw_circle(
            x + 21.0,
            y + 8.0,
            4.0,
            if (elapsed * 3.5 + index as f32).sin() > 0.0 {
                visual_theme::warning()
            } else {
                visual_theme::structure_light()
            },
        );
    }
}

fn draw_research_details(wreck: Rect, elapsed: f32) {
    let instrument = visual_theme::with_alpha(visual_theme::cyan(), 0.68);
    for index in 0..4 {
        let x = wreck.x + 48.0 + index as f32 * 118.0;
        draw_rectangle(
            x,
            wreck.y + 78.0,
            62.0,
            34.0,
            visual_theme::with_alpha(visual_theme::structure_dark(), 0.84),
        );
        draw_rectangle_lines(x, wreck.y + 78.0, 62.0, 34.0, 2.0, instrument);
        draw_line(
            x + 10.0,
            wreck.y + 94.0,
            x + 52.0,
            wreck.y + 94.0,
            2.0,
            instrument,
        );
    }
    let ring = 42.0 + (elapsed * 1.8).sin().abs() * 8.0;
    draw_circle_lines(
        wreck.x + wreck.w * 0.78,
        wreck.y + wreck.h * 0.47,
        ring,
        2.0,
        visual_theme::with_alpha(visual_theme::cyan(), 0.52),
    );
    draw_circle(
        wreck.x + wreck.w * 0.78,
        wreck.y + wreck.h * 0.47,
        5.0,
        instrument,
    );
}
