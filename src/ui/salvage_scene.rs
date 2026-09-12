//! The close salvage workspace: scan, select, extract, and return.

use super::drone_command;
use super::drone_visual;
use super::extraction_panel;
use super::scan_overlay;
use super::scene_layout::{self, SalvageLayout};
use super::section_nav;
use super::ship_visual;
use super::transfer_hardware;
use super::visual_theme;
use super::wreck_visual;
use super::*;
use crate::engine::WorkspaceHazard;
use crate::state::workspace::{ExtractionPhase, TransferMode};
use macroquad_toolkit::math::lerp;

mod command_panel;
mod power_cycle;

pub(crate) const SECTION_SHIFT_SECONDS: f32 = 0.75;
pub(crate) const SECTION_ARRIVAL_FLASH_SECONDS: f32 = 0.6;
pub(crate) const SECTION_SETTLED_PROMPT: &str = "Section settled. Tap SCAN to reveal this frame.";

pub(crate) fn section_switch_prompt(message: &str, moving_camera: bool) -> String {
    if moving_camera {
        format!("{message} Camera shift underway; wait for ARRIVAL, then tap SCAN.")
    } else {
        format!("{message} Tap SCAN to reveal this section.")
    }
}

pub(crate) fn section_arrival_ready(camera_shift: f32, workspace_elapsed: f32) -> bool {
    camera_shift >= 1.0 && workspace_elapsed >= 0.8
}

fn section_shift_ease(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    progress * progress * (3.0 - 2.0 * progress)
}

pub fn draw_salvage_workspace(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let layout = scene_layout::salvage_layout();
    let Some(expedition) = &ctx.session.expedition else {
        draw_text(
            "NO ACTIVE SALVAGE RUN",
            54.0,
            176.0,
            24.0,
            visual_theme::warning(),
        );
        return;
    };
    let Some(site) = ctx.data.sites.get(&expedition.site_id) else {
        return;
    };
    let section = ctx.session.workspace_section(ctx.data).ok();
    let condition = ctx
        .session
        .workspace_condition_status(ctx.data)
        .unwrap_or_else(|_| crate::state::workspace::WorkspaceConditionStatus::unknown());
    section_nav::draw_section_nav(ctx, site, actions);
    if let Some(section) = section {
        let hazard_readout = if section.hazard_tags.is_empty() {
            "HAZARDS: NONE LOGGED".to_owned()
        } else {
            format!(
                "HAZARDS {:02} // {}",
                section.hazard_tags.len(),
                section
                    .hazard_tags
                    .iter()
                    .map(|tag| hazard_label(tag))
                    .collect::<Vec<_>>()
                    .join(" / ")
            )
        };
        draw_text(
            hazard_readout,
            layout.viewport.x + 30.0,
            layout.viewport.y + 42.0,
            12.0,
            if section.hazard_tags.is_empty() {
                visual_theme::text_dim()
            } else {
                visual_theme::warning()
            },
        );
    }
    draw_text(
        format!(
            "{}  /  {}",
            site.display_name.to_uppercase(),
            section.map_or("UNKNOWN SECTION", |value| value.display_name.as_str())
        ),
        layout.viewport.x + 30.0,
        layout.viewport.y + 24.0,
        15.0,
        visual_theme::text_dim(),
    );
    command_panel::draw_debris(ctx.workspace_elapsed);
    wreck_visual::draw_wreck(wreck_visual::WreckView {
        layout,
        site,
        session: ctx.session,
        data: ctx.data,
        elapsed: ctx.workspace_elapsed,
        scanned: ctx.workspace_scanned,
        selected_target: ctx.workspace_selected_target,
        extraction_target: ctx.workspace_extraction_target,
        extraction_progress: ctx.workspace_extraction_progress,
        section_targets: section.map_or(&[], |value| value.candidate_targets.as_slice()),
        section_hazards: section.map_or(&[], |value| value.hazard_tags.as_slice()),
        condition,
    });
    ship_visual::draw_ship(
        layout.ship,
        ctx.session,
        ctx.data,
        ctx.workspace_elapsed,
        ctx.workspace_selected_target.is_some(),
    );
    drone_visual::draw_deployed_drones(
        layout,
        ctx.session,
        ctx.data,
        ctx.workspace_elapsed,
        ctx.workspace_selected_target,
        ctx.workspace_extraction_target,
    );
    if let Some(target_id) = ctx.workspace_extraction_target {
        let mode = ctx
            .data
            .salvage_objects
            .get(target_id)
            .map(TransferMode::from_target)
            .unwrap_or(TransferMode::InternalCargo);
        transfer_hardware::draw_transfer_hardware(layout.ship, mode, ctx.workspace_elapsed);
        command_panel::draw_tractor_beam(ctx, layout, target_id);
    }
    scan_overlay::draw_scan_overlay(
        layout,
        ctx.workspace_elapsed,
        ctx.workspace_scan_progress,
        ctx.workspace_scanned,
        &expedition.revealed_targets,
        expedition.scan_profile,
    );
    draw_section_shift(ctx, layout);
    draw_target_selection(ctx, layout, actions);
    command_panel::draw_command_panel(ctx, layout, actions);
    extraction_panel::draw_target_panel(ctx, layout, actions);
    command_panel::draw_notice(ctx);
}

fn draw_section_shift(ctx: &UiContext<'_>, layout: SalvageLayout) {
    if ctx.workspace_camera_shift >= 1.0 {
        if ctx.workspace_arrival_flash <= 0.0 {
            return;
        }
        let intensity = ctx.workspace_arrival_flash.clamp(0.0, 1.0);
        draw_rectangle_lines(
            layout.wreck.x - 7.0,
            layout.wreck.y - 7.0,
            layout.wreck.w + 14.0,
            layout.wreck.h + 14.0,
            3.0,
            visual_theme::with_alpha(visual_theme::safe(), intensity),
        );
        draw_circle_lines(
            layout.wreck.center().x,
            layout.wreck.center().y,
            44.0 + (1.0 - intensity) * 26.0,
            2.0,
            visual_theme::with_alpha(visual_theme::safe(), intensity),
        );
        draw_text(
            if section_arrival_ready(ctx.workspace_camera_shift, ctx.workspace_elapsed) {
                "SECTION LOCKED // SCAN READY"
            } else {
                "SECTION LOCKED // HULL SETTLING"
            },
            layout.wreck.x + 18.0,
            516.0,
            12.0,
            visual_theme::safe(),
        );
        return;
    }
    let progress = ctx.workspace_camera_shift.clamp(0.0, 1.0);
    let sweep_x = layout.wreck.x - 42.0 + section_shift_ease(progress) * (layout.wreck.w + 84.0);
    draw_rectangle(
        layout.ship.x - 24.0,
        layout.ship.y - 26.0,
        layout.wreck.right() - layout.ship.x + 48.0,
        layout.wreck.bottom() - layout.ship.y + 44.0,
        visual_theme::with_alpha(visual_theme::space(), 0.22),
    );
    draw_line(
        sweep_x,
        layout.wreck.y - 12.0,
        sweep_x,
        layout.wreck.bottom() + 12.0,
        3.0,
        visual_theme::with_alpha(visual_theme::cyan(), 0.82),
    );
    draw_text(
        "CAMERA SHIFT // FOLLOWING WORKBOAT",
        layout.wreck.x + 18.0,
        516.0,
        12.0,
        visual_theme::cyan(),
    );
    draw_text(
        format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
        layout.wreck.right() - 116.0,
        516.0,
        11.0,
        visual_theme::text_dim(),
    );
}

fn draw_target_selection(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    if !ctx.workspace_scanned
        || ctx.workspace_extraction_target.is_some()
        || ctx.workspace_camera_shift < 1.0
    {
        return;
    }
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    if !ctx.interaction_enabled || !ctx.pointer.released {
        return;
    }
    for target_id in &expedition.revealed_targets {
        let Some(rect) = layout.target_rect(target_id) else {
            continue;
        };
        if ctx.pointer.released_on(rect) && !ctx.session.target_is_removed(target_id) {
            actions.push(UiAction::SelectTarget(target_id.clone()));
            break;
        }
    }
}
