//! Cargo rig and module mount rendering for the ship silhouette.

use super::*;

pub(super) fn draw_external_cargo(
    hull: Rect,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
) {
    let packed_ids: Vec<String> = session
        .expedition
        .as_ref()
        .map(|expedition| {
            expedition
                .cargo
                .iter()
                .filter(|cargo| cargo.status == CargoStatus::Packed)
                .filter_map(|cargo| {
                    data.salvage_objects
                        .get(&cargo.object_id)
                        .filter(|object| TransferMode::from_target(object).uses_external_rig())
                        .map(|_| cargo.object_id.clone())
                })
                .collect()
        })
        .unwrap_or_default();
    let returned_ids: Vec<String> = if packed_ids.is_empty() {
        session
            .returned
            .iter()
            .filter_map(|cargo| {
                data.salvage_objects
                    .get(&cargo.object_id)
                    .filter(|object| TransferMode::from_target(object).uses_external_rig())
                    .map(|_| cargo.object_id.clone())
            })
            .collect()
    } else {
        Vec::new()
    };
    let cargo_ids = if packed_ids.is_empty() {
        returned_ids
    } else {
        packed_ids
    };
    let visible_count = cargo_ids.len().min(5);
    let slot_spacing = if visible_count > 1 {
        0.64 / (visible_count - 1) as f32
    } else {
        0.0
    };
    for (index, object_id) in cargo_ids.iter().take(visible_count).enumerate() {
        let Some(object) = data.salvage_objects.get(object_id) else {
            continue;
        };
        let slot = index as f32;
        let slot_x = hull.x + hull.w * (0.18 + slot * slot_spacing);
        let slot_y = hull.bottom() + 10.0 + (elapsed * 1.3 + slot).sin() * 1.5;
        let slot_w = (hull.w * (0.62 / visible_count.max(1) as f32)).clamp(22.0, 46.0);
        let slot_h = (hull.h * 0.18).clamp(12.0, 24.0);
        let accent = cargo_accent(&object.visual_silhouette);
        draw_line(
            slot_x + slot_w * 0.5,
            hull.bottom() - 2.0,
            slot_x + slot_w * 0.5,
            slot_y,
            2.0,
            visual_theme::cyan_dim(),
        );
        draw_circle(
            slot_x + slot_w * 0.5,
            hull.bottom() - 2.0,
            4.0,
            visual_theme::cyan(),
        );
        draw_rectangle(
            slot_x,
            slot_y,
            slot_w,
            slot_h,
            visual_theme::with_alpha(accent, 0.82),
        );
        draw_rectangle_lines(slot_x, slot_y, slot_w, slot_h, 2.0, accent);
        draw_line(
            slot_x + slot_w * 0.18,
            slot_y + slot_h * 0.22,
            slot_x + slot_w * 0.82,
            slot_y + slot_h * 0.78,
            1.0,
            visual_theme::structure_light(),
        );
        draw_circle(
            slot_x + slot_w * 0.82,
            slot_y + slot_h * 0.2,
            2.5,
            visual_theme::amber(),
        );
    }
    if cargo_ids.len() > visible_count {
        let overflow_label = format!("+{} MORE EXTERNAL", cargo_ids.len() - visible_count);
        draw_text(
            &overflow_label,
            hull.x + hull.w * 0.64,
            hull.bottom() + 38.0,
            9.0,
            visual_theme::warning(),
        );
    }
    let clamp_label = format!(
        "CLAMPS {}/{}",
        session.external_cargo_count(data, None),
        session.external_capacity(data)
    );
    draw_text(
        &clamp_label,
        hull.x + hull.w * 0.66,
        hull.bottom() + 30.0,
        9.0,
        visual_theme::amber(),
    );
}

fn cargo_accent(silhouette: &str) -> Color {
    if silhouette.contains("computer") || silhouette.contains("sensor") {
        visual_theme::cyan()
    } else if silhouette.contains("reactor") || silhouette.contains("engine") {
        visual_theme::warning()
    } else {
        visual_theme::amber()
    }
}

pub fn module_mount_rect(rect: Rect, module: &ModuleData) -> Rect {
    let anchor = mount_anchor(rect, &module.mount);
    Rect::new(anchor.x - 32.0, anchor.y - 32.0, 64.0, 64.0)
}

pub fn tractor_emitter_rect(rect: Rect) -> Rect {
    Rect::new(
        rect.x + rect.w * 0.88,
        rect.y + rect.h * 0.03,
        rect.w * 0.12,
        rect.h * 0.38,
    )
}

fn mount_anchor(rect: Rect, mount: &str) -> Vec2 {
    let (x, y) = match mount {
        "aft_core" => (0.12, 0.5),
        "port_tank" => (0.38, 0.23),
        "bridge_nav" => (0.7, 0.18),
        "bow_scanner" => (0.82, 0.43),
        "starboard_reactor" => (0.57, 0.58),
        "starboard_shield" => (0.73, 0.62),
        "port_drone" => (0.31, 0.59),
        "port_battery" => (0.48, 0.59),
        "hull_patch" => (0.52, 0.37),
        _ => (0.5, 0.48),
    };
    vec2(rect.x + rect.w * x, rect.y + rect.h * y)
}

pub(super) fn draw_module_mounts(
    hull: Rect,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    selected_module: Option<&str>,
) {
    for placement in session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
    {
        let Some(module) = data.modules.get(&placement.id) else {
            continue;
        };
        let anchor = mount_anchor(hull, &module.mount);
        let selected = selected_module == Some(placement.id.as_str());
        if selected {
            draw_circle(
                anchor.x,
                anchor.y,
                25.0,
                visual_theme::with_alpha(visual_theme::amber(), 0.14),
            );
            draw_rectangle_lines(
                anchor.x - 21.0,
                anchor.y - 21.0,
                42.0,
                42.0,
                3.0,
                visual_theme::amber(),
            );
        }
        if session.damaged_modules.iter().any(|id| id == &placement.id) {
            draw_rectangle_lines(
                anchor.x - 14.0,
                anchor.y - 14.0,
                28.0,
                28.0,
                3.0,
                visual_theme::warning(),
            );
            draw_line(
                anchor.x - 9.0,
                anchor.y - 9.0,
                anchor.x + 9.0,
                anchor.y + 9.0,
                2.0,
                visual_theme::warning(),
            );
            draw_line(
                anchor.x + 9.0,
                anchor.y - 9.0,
                anchor.x - 9.0,
                anchor.y + 9.0,
                2.0,
                visual_theme::warning(),
            );
            continue;
        }
        match module.visual_kind.as_str() {
            "engine" => {
                draw_rectangle(
                    anchor.x - 15.0,
                    anchor.y - 12.0,
                    30.0,
                    24.0,
                    visual_theme::structure_light(),
                );
                draw_line(
                    anchor.x - 9.0,
                    anchor.y - 8.0,
                    anchor.x + 9.0,
                    anchor.y - 8.0,
                    2.0,
                    visual_theme::amber(),
                );
            }
            "scanner" => {
                draw_line(
                    anchor.x,
                    anchor.y,
                    anchor.x + 19.0,
                    anchor.y - 19.0,
                    2.0,
                    visual_theme::cyan(),
                );
                draw_circle(anchor.x + 19.0, anchor.y - 19.0, 5.0, visual_theme::cyan());
            }
            "reactor" => {
                draw_circle(anchor.x, anchor.y, 12.0, visual_theme::warning());
                draw_circle(anchor.x, anchor.y, 5.0, visual_theme::amber());
            }
            "shield" => draw_rectangle(
                anchor.x - 10.0,
                anchor.y - 8.0,
                20.0,
                16.0,
                visual_theme::safe(),
            ),
            "drone_bay" => {
                draw_rectangle(
                    anchor.x - 16.0,
                    anchor.y - 6.0,
                    32.0,
                    12.0,
                    visual_theme::structure_light(),
                );
                let flight = (elapsed * 2.0).sin() * 7.0;
                draw_circle(
                    anchor.x - 9.0,
                    anchor.y - 16.0 + flight,
                    4.0,
                    visual_theme::cyan(),
                );
                draw_circle(
                    anchor.x + 9.0,
                    anchor.y - 16.0 - flight,
                    4.0,
                    visual_theme::cyan(),
                );
            }
            "antenna" => draw_line(
                anchor.x,
                anchor.y,
                anchor.x,
                anchor.y - 28.0,
                2.0,
                visual_theme::text_dim(),
            ),
            "tank" | "battery" => {
                draw_rectangle(
                    anchor.x - 15.0,
                    anchor.y - 6.0,
                    30.0,
                    12.0,
                    visual_theme::structure_light(),
                );
                draw_line(
                    anchor.x - 10.0,
                    anchor.y,
                    anchor.x + 10.0,
                    anchor.y,
                    2.0,
                    visual_theme::amber(),
                );
            }
            "plating" => draw_line(
                anchor.x - 14.0,
                anchor.y - 10.0,
                anchor.x + 14.0,
                anchor.y + 10.0,
                4.0,
                visual_theme::safe(),
            ),
            _ => draw_circle(anchor.x, anchor.y, 7.0, visual_theme::structure_light()),
        }
    }
    if let Some(module_id) = selected_module {
        let installed = session
            .ship_layout
            .placements
            .iter()
            .any(|item| item.permanent && item.id == module_id);
        if !installed {
            if let Some(module) = data.modules.get(module_id) {
                draw_mount_preview(hull, module, elapsed);
            }
        }
    }
}

fn draw_mount_preview(hull: Rect, module: &ModuleData, elapsed: f32) {
    let anchor = mount_anchor(hull, &module.mount);
    let pulse = 0.38 + (elapsed * 3.0).sin().abs() * 0.24;
    let accent = match module.visual_kind.as_str() {
        "scanner" | "antenna" => visual_theme::cyan(),
        "reactor" | "engine" => visual_theme::warning(),
        _ => visual_theme::amber(),
    };
    draw_circle(
        anchor.x,
        anchor.y,
        29.0,
        visual_theme::with_alpha(accent, 0.08),
    );
    draw_rectangle_lines(
        anchor.x - 24.0,
        anchor.y - 24.0,
        48.0,
        48.0,
        2.0,
        visual_theme::with_alpha(accent, pulse),
    );
    draw_line(
        anchor.x - 16.0,
        anchor.y - 16.0,
        anchor.x + 16.0,
        anchor.y + 16.0,
        2.0,
        visual_theme::with_alpha(accent, pulse),
    );
    draw_line(
        anchor.x + 16.0,
        anchor.y - 16.0,
        anchor.x - 16.0,
        anchor.y + 16.0,
        2.0,
        visual_theme::with_alpha(accent, pulse),
    );
    draw_text(
        "INSTALL PREVIEW",
        anchor.x - 43.0,
        anchor.y + 39.0,
        9.0,
        accent,
    );
}
