//! Hazard rendering for wrecks and their salvage mounts.

use super::*;

pub(super) fn draw_hazard_details(wreck: Rect, hazard_tags: &[String], elapsed: f32) {
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

pub struct TargetMountView<'a> {
    pub(super) layout: SalvageLayout,
    pub(super) target_id: &'a str,
    pub(super) session: &'a GameSession,
    pub(super) data: &'a GameData,
    pub(super) contract_target: Option<&'a str>,
    pub(super) scanned: bool,
    pub(super) selected_target: Option<&'a str>,
    pub(super) extraction_target: Option<&'a str>,
    pub(super) extraction_progress: f32,
    pub(super) elapsed: f32,
}

pub fn draw_target_mount(view: TargetMountView<'_>) {
    let TargetMountView {
        layout,
        target_id,
        session,
        data,
        contract_target,
        scanned,
        selected_target,
        extraction_target,
        extraction_progress,
        elapsed,
    } = view;
    let Some(rect) = layout.target_rect(target_id) else {
        return;
    };
    let removed = session.target_is_removed(target_id);
    let selected = selected_target == Some(target_id);
    let extracting = extraction_target == Some(target_id);
    let stabilized = session.target_is_stabilized(target_id);
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
            "EMPTY MOUNT",
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
    let base = if selected || extracting {
        visual_theme::cyan()
    } else {
        visual_theme::structure_light()
    };
    let shake = if extracting {
        (extraction_progress * 48.0).sin() * 3.0
    } else {
        0.0
    };
    let draw_rect = Rect::new(rect.x + shake, rect.y, rect.w, rect.h);
    draw_rectangle(
        draw_rect.x + 12.0,
        draw_rect.y + 14.0,
        draw_rect.w,
        draw_rect.h,
        visual_theme::with_alpha(BLACK, 0.7),
    );
    draw_rectangle(
        draw_rect.x - 6.0,
        draw_rect.y - 5.0,
        draw_rect.w + 12.0,
        draw_rect.h + 16.0,
        visual_theme::space(),
    );
    draw_rectangle(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        if scanned {
            visual_theme::with_alpha(base, 0.72)
        } else {
            visual_theme::with_alpha(visual_theme::structure_light(), 0.38)
        },
    );
    draw_rectangle(
        draw_rect.x,
        draw_rect.bottom() - 10.0,
        draw_rect.w,
        10.0,
        visual_theme::structure_dark(),
    );
    draw_line(
        draw_rect.x,
        draw_rect.y,
        draw_rect.right(),
        draw_rect.y,
        3.0,
        visual_theme::structure_light(),
    );
    draw_line(
        draw_rect.x,
        draw_rect.y,
        draw_rect.x,
        draw_rect.bottom(),
        2.0,
        visual_theme::structure_light(),
    );
    match target.map_or("", |target| target.visual_silhouette.as_str()) {
        "relay" => {
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
        "navigation" => {
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
    if extracting && scanned {
        if let Some(target) = target {
            if let Some(hazard) = target.hazard.as_deref() {
                hazard_visual::draw_target_hazard(
                    draw_rect,
                    hazard,
                    extraction_progress,
                    extraction_progress,
                );
            }
        }
    }
    if stabilized && scanned {
        hazard_visual::draw_stabilization_lock(draw_rect, elapsed);
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
        if scanned {
            let transfer_mode = TransferMode::from_target(target);
            let transfer_color = match transfer_mode {
                TransferMode::InternalCargo => visual_theme::cyan(),
                TransferMode::ExternalClamp => visual_theme::amber(),
                TransferMode::Tow => visual_theme::warning(),
            };
            draw_text(
                transfer_mode.short_label(),
                draw_rect.right() - 40.0,
                draw_rect.bottom() + 16.0,
                10.0,
                transfer_color,
            );
        }
        if scanned {
            if let Some(hazard) = target.hazard.as_deref() {
                draw_circle(
                    draw_rect.right() - 10.0,
                    draw_rect.y + 10.0,
                    7.0,
                    visual_theme::warning(),
                );
                draw_text(
                    hazard_marker(hazard),
                    draw_rect.right() - 12.0,
                    draw_rect.y + 15.0,
                    12.0,
                    WHITE,
                );
            }
        }
    }
}

fn hazard_marker(hazard_value: &str) -> &'static str {
    match WorkspaceHazard::from_value(hazard_value) {
        Some(WorkspaceHazard::ReactorInstability) => "T",
        Some(WorkspaceHazard::ElectricalArcs) => "A",
        Some(WorkspaceHazard::AutomatedDefenses) => "D",
        Some(WorkspaceHazard::UnexplodedAmmunition) => "O",
        Some(WorkspaceHazard::MagneticInterference) => "M",
        Some(WorkspaceHazard::StructuralCollapse) => "H",
        None => "!",
    }
}
