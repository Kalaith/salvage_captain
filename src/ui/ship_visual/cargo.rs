//! Cargo rig and module mount rendering for the ship silhouette.

use super::*;

pub(super) fn draw_external_cargo(
    hull: Rect,
    session: &GameSession,
    data: &GameData,
    elapsed: f32,
    show_labels: bool,
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
    draw_external_loads(hull, &cargo_ids, data, elapsed);
    if !show_labels {
        return;
    }
    if cargo_ids.len() > visible_count {
        let overflow_label = format!("+{} MORE EXTERNAL", cargo_ids.len() - visible_count);
        draw_text(
            &overflow_label,
            hull.x + hull.w * 0.64,
            hull.bottom() + 66.0,
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
        hull.bottom() + 52.0,
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

fn draw_external_loads(hull: Rect, cargo_ids: &[String], data: &GameData, elapsed: f32) {
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
        let slot_w = (hull.w * (0.62 / visible_count.max(1) as f32)).clamp(14.0, 64.0);
        let slot_h = (hull.h * 0.18).clamp(10.0, 30.0);
        let accent = cargo_accent(&object.visual_silhouette);
        draw_line(
            slot_x + slot_w * 0.5,
            hull.y + hull.h * (287.0 / 360.0),
            slot_x + slot_w * 0.5,
            slot_y,
            2.0,
            visual_theme::cyan_dim(),
        );
        draw_circle(
            slot_x + slot_w * 0.5,
            hull.y + hull.h * (287.0 / 360.0),
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
        for strap in [0.22, 0.74] {
            draw_rectangle(
                slot_x + slot_w * strap,
                slot_y,
                slot_w * 0.07,
                slot_h,
                visual_theme::structure_dark(),
            );
        }
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
}
