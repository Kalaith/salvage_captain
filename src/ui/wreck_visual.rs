//! Close camera framing for a wreck section and its physical salvage mounts.

use super::condition_visual;
use super::scene_layout::SalvageLayout;
use super::visual_theme;
use crate::data::{GameData, SiteData};
use crate::state::workspace::WorkspaceConditionStatus;
use crate::state::GameSession;
use macroquad::prelude::*;
use macroquad_toolkit::math::blink;

pub fn draw_wreck(
    layout: SalvageLayout,
    site: &SiteData,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    scanned: bool,
    selected_target: Option<&str>,
    extraction_target: Option<&str>,
    extraction_progress: f32,
    section_targets: &[String],
    section_hazards: &[String],
    condition: WorkspaceConditionStatus,
) {
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
    draw_hazard_details(wreck, section_hazards, elapsed);
    for target_id in section_targets {
        draw_target_mount(
            layout,
            target_id,
            session,
            data,
            site.contract_target.as_deref(),
            scanned,
            selected_target,
            extraction_target,
            extraction_progress,
        );
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

fn draw_hazard_details(wreck: Rect, hazard_tags: &[String], elapsed: f32) {
    let spacing = if hazard_tags.len() > 1 {
        (wreck.w - 144.0) / (hazard_tags.len() - 1) as f32
    } else {
        0.0
    };
    for (index, hazard) in hazard_tags.iter().enumerate() {
        let x = wreck.x + 72.0 + index as f32 * spacing;
        let y = wreck.y + wreck.h * 0.2 + (index % 2) as f32 * wreck.h * 0.48;
        match hazard.as_str() {
            "electrical_arcs" => draw_electrical_hazard(x, y, elapsed),
            "unstable_fuel" => draw_fuel_hazard(x, y, elapsed),
            "decompression" => draw_decompression_hazard(x, y),
            "moving_debris" => draw_debris_hazard(x, y, elapsed),
            "reactor_instability" => draw_reactor_hazard(x, y, elapsed),
            "radiation" => draw_radiation_hazard(x, y, elapsed),
            "automated_defenses" => draw_defense_hazard(x, y),
            "magnetic_interference" => draw_magnetic_hazard(x, y, elapsed),
            _ => draw_generic_hazard(x, y),
        }
        draw_circle(x, y, 6.0, visual_theme::warning());
        draw_text("!", x - 3.0, y + 4.0, 10.0, WHITE);
    }
}

fn draw_electrical_hazard(x: f32, y: f32, elapsed: f32) {
    let flicker = if (elapsed * 9.0).sin() > -0.2 {
        visual_theme::cyan()
    } else {
        visual_theme::warning()
    };
    draw_line(x - 26.0, y - 34.0, x - 10.0, y - 12.0, 3.0, flicker);
    draw_line(x - 10.0, y - 12.0, x - 20.0, y + 4.0, 3.0, flicker);
    draw_line(x - 20.0, y + 4.0, x + 12.0, y + 28.0, 3.0, flicker);
    draw_line(x + 12.0, y + 28.0, x + 24.0, y + 12.0, 2.0, flicker);
}

fn draw_fuel_hazard(x: f32, y: f32, elapsed: f32) {
    let pulse = 16.0 + (elapsed * 3.0).sin().abs() * 6.0;
    draw_circle_lines(x, y, pulse, 2.0, visual_theme::warning());
    draw_circle(
        x,
        y,
        9.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.7),
    );
    draw_line(
        x - 22.0,
        y + 20.0,
        x + 22.0,
        y + 20.0,
        2.0,
        visual_theme::amber(),
    );
}

fn draw_decompression_hazard(x: f32, y: f32) {
    draw_rectangle(x - 24.0, y - 18.0, 48.0, 36.0, visual_theme::space());
    draw_rectangle_lines(x - 24.0, y - 18.0, 48.0, 36.0, 2.0, visual_theme::warning());
    for index in 0..3 {
        let offset = index as f32 * 10.0 - 10.0;
        draw_line(
            x - 8.0,
            y + offset,
            x + 18.0,
            y + offset,
            2.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_debris_hazard(x: f32, y: f32, elapsed: f32) {
    let angle = elapsed * 1.7;
    let points = [
        vec2(x + angle.cos() * 20.0, y + angle.sin() * 20.0),
        vec2(x - angle.sin() * 14.0, y + angle.cos() * 14.0),
        vec2(x - angle.cos() * 20.0, y - angle.sin() * 20.0),
        vec2(x + angle.sin() * 14.0, y - angle.cos() * 14.0),
    ];
    for pair in points.windows(2) {
        draw_line(
            pair[0].x,
            pair[0].y,
            pair[1].x,
            pair[1].y,
            3.0,
            visual_theme::amber(),
        );
    }
    draw_line(
        points[3].x,
        points[3].y,
        points[0].x,
        points[0].y,
        3.0,
        visual_theme::amber(),
    );
}

fn draw_reactor_hazard(x: f32, y: f32, elapsed: f32) {
    let radius = 24.0 + (elapsed * 4.0).sin().abs() * 8.0;
    draw_circle_lines(x, y, radius, 3.0, visual_theme::warning());
    draw_circle_lines(x, y, radius * 0.55, 2.0, visual_theme::amber());
    draw_line(x - radius, y, x + radius, y, 2.0, visual_theme::warning());
    draw_line(x, y - radius, x, y + radius, 2.0, visual_theme::warning());
}

fn draw_radiation_hazard(x: f32, y: f32, elapsed: f32) {
    let radius = 20.0 + (elapsed * 2.5).sin().abs() * 5.0;
    draw_circle_lines(
        x,
        y,
        radius,
        2.0,
        visual_theme::with_alpha(visual_theme::warning(), 0.72),
    );
    draw_circle_lines(
        x,
        y,
        radius * 0.56,
        2.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.78),
    );
    for index in 0..3 {
        let angle = index as f32 * 2.1 + elapsed * 0.4;
        draw_line(
            x,
            y,
            x + angle.cos() * radius,
            y + angle.sin() * radius,
            2.0,
            visual_theme::warning(),
        );
    }
}

fn draw_defense_hazard(x: f32, y: f32) {
    draw_rectangle(
        x - 22.0,
        y - 14.0,
        44.0,
        28.0,
        visual_theme::structure_dark(),
    );
    draw_rectangle_lines(x - 22.0, y - 14.0, 44.0, 28.0, 2.0, visual_theme::warning());
    draw_circle(x, y, 6.0, visual_theme::warning());
    draw_line(
        x + 18.0,
        y,
        x + 48.0,
        y - 22.0,
        2.0,
        visual_theme::warning(),
    );
}

fn draw_magnetic_hazard(x: f32, y: f32, elapsed: f32) {
    let radius = 19.0 + (elapsed * 2.0).sin().abs() * 8.0;
    draw_circle_lines(x, y, radius, 2.0, visual_theme::cyan());
    draw_circle_lines(x, y, radius * 0.45, 2.0, visual_theme::cyan_dim());
    draw_line(
        x - radius - 8.0,
        y,
        x + radius + 8.0,
        y,
        2.0,
        visual_theme::cyan(),
    );
}

fn draw_generic_hazard(x: f32, y: f32) {
    draw_rectangle_lines(x - 18.0, y - 18.0, 36.0, 36.0, 2.0, visual_theme::warning());
    draw_line(
        x - 13.0,
        y - 13.0,
        x + 13.0,
        y + 13.0,
        2.0,
        visual_theme::warning(),
    );
    draw_line(
        x + 13.0,
        y - 13.0,
        x - 13.0,
        y + 13.0,
        2.0,
        visual_theme::warning(),
    );
}

fn draw_target_mount(
    layout: SalvageLayout,
    target_id: &str,
    session: &GameSession,
    data: &GameData,
    contract_target: Option<&str>,
    scanned: bool,
    selected_target: Option<&str>,
    extraction_target: Option<&str>,
    extraction_progress: f32,
) {
    let Some(rect) = layout.target_rect(target_id) else {
        return;
    };
    let removed = session.target_is_removed(target_id);
    let selected = selected_target == Some(target_id);
    let extracting = extraction_target == Some(target_id);
    if removed {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::space());
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, visual_theme::warning());
        draw_line(
            rect.x,
            rect.y,
            rect.right(),
            rect.bottom(),
            2.0,
            visual_theme::warning(),
        );
        draw_line(
            rect.right(),
            rect.y,
            rect.x,
            rect.bottom(),
            2.0,
            visual_theme::warning(),
        );
        draw_text(
            if contract_target == Some(target_id) {
                "CONTRACT LOST"
            } else {
                "EMPTY MOUNT"
            },
            rect.x,
            rect.bottom() + 16.0,
            11.0,
            if contract_target == Some(target_id) {
                visual_theme::warning()
            } else {
                visual_theme::text_dim()
            },
        );
        return;
    }
    let target = data.salvage_objects.get(target_id);
    let base = if target_id == "engine_assembly" {
        visual_theme::warning()
    } else if target_id == "navigation_computer" {
        visual_theme::cyan()
    } else {
        visual_theme::amber()
    };
    let shake = if extracting {
        (extraction_progress * 48.0).sin() * 3.0
    } else {
        0.0
    };
    let draw_rect = Rect::new(rect.x + shake, rect.y, rect.w, rect.h);
    draw_rectangle(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        if scanned {
            visual_theme::with_alpha(base, 0.78)
        } else {
            visual_theme::with_alpha(visual_theme::structure_light(), 0.38)
        },
    );
    match target_id {
        "industrial_battery" => {
            draw_rectangle_lines(
                draw_rect.x + 12.0,
                draw_rect.y + 10.0,
                34.0,
                52.0,
                3.0,
                visual_theme::structure_dark(),
            );
            draw_circle(
                draw_rect.x + 68.0,
                draw_rect.y + 22.0,
                6.0,
                visual_theme::amber(),
            );
            draw_line(
                draw_rect.x + 54.0,
                draw_rect.y + 32.0,
                draw_rect.right() - 10.0,
                draw_rect.y + 54.0,
                2.0,
                visual_theme::structure_dark(),
            );
        }
        "navigation_computer" => {
            for index in 0..3 {
                draw_rectangle(
                    draw_rect.x + 12.0 + index as f32 * 30.0,
                    draw_rect.y + 25.0,
                    20.0,
                    26.0,
                    visual_theme::structure_dark(),
                );
                draw_circle(
                    draw_rect.x + 22.0 + index as f32 * 30.0,
                    draw_rect.y + 34.0,
                    3.0,
                    visual_theme::cyan(),
                );
            }
        }
        _ => {
            draw_circle(
                draw_rect.center().x,
                draw_rect.center().y,
                draw_rect.h * 0.3,
                visual_theme::structure_dark(),
            );
            draw_line(
                draw_rect.x + 14.0,
                draw_rect.y + 16.0,
                draw_rect.right() - 14.0,
                draw_rect.bottom() - 16.0,
                3.0,
                base,
            );
        }
    }
    if selected || extracting {
        let outline = if extracting {
            visual_theme::cyan()
        } else {
            visual_theme::text()
        };
        draw_rectangle_lines(
            draw_rect.x - 5.0,
            draw_rect.y - 5.0,
            draw_rect.w + 10.0,
            draw_rect.h + 10.0,
            3.0,
            outline,
        );
        draw_text(
            if extracting { "WORKING" } else { "SELECTED" },
            draw_rect.x,
            draw_rect.y - 10.0,
            12.0,
            outline,
        );
    } else if scanned {
        draw_rectangle_lines(
            draw_rect.x - 3.0,
            draw_rect.y - 3.0,
            draw_rect.w + 6.0,
            draw_rect.h + 6.0,
            1.0,
            visual_theme::with_alpha(base, 0.8),
        );
    }
    if scanned && contract_target == Some(target_id) {
        draw_text(
            "OBJECTIVE",
            draw_rect.x,
            draw_rect.bottom() + 16.0,
            10.0,
            visual_theme::amber(),
        );
    }
    if let Some(target) = target {
        if scanned && target.hazard.is_some() {
            draw_circle(
                draw_rect.right() - 10.0,
                draw_rect.y + 10.0,
                7.0,
                visual_theme::warning(),
            );
            draw_text(
                "!",
                draw_rect.right() - 12.0,
                draw_rect.y + 15.0,
                12.0,
                WHITE,
            );
        }
    }
}
