//! Touch-first control for the operator's active drone directive.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use super::{button, clipped, ButtonTone, UiAction, UiContext};
use crate::state::DroneDirective;
use macroquad::prelude::*;

pub(crate) fn draw_operator_control(
    ctx: &UiContext<'_>,
    layout: SalvageLayout,
    actions: &mut Vec<UiAction>,
) {
    let support = ctx.session.module_stats(ctx.data).drone_support;
    if support <= 0 {
        return;
    }
    let directive = ctx.session.workspace_drone_directive();
    let rect = Rect::new(
        layout.command.x + 16.0,
        layout.command.y + 92.0,
        150.0,
        40.0,
    );
    let can_change = support > 0
        && ctx.workspace_camera_shift >= 1.0
        && ctx.workspace_scan_progress <= 0.0
        && ctx.workspace_extraction_target.is_none();
    let label = directive_button_label(support, directive);
    if button(
        ctx,
        rect,
        &label,
        can_change,
        if directive == DroneDirective::Standby {
            ButtonTone::Warning
        } else {
            ButtonTone::Secondary
        },
    ) {
        actions.push(UiAction::CycleDroneDirective);
    }
    draw_text(
        if support > 0 {
            clipped(directive.description(), 31)
        } else {
            "Install a Drone Bay to issue field orders.".to_owned()
        },
        layout.command.x + 180.0,
        layout.command.y + 108.0,
        10.0,
        if support > 0 {
            visual_theme::text_dim()
        } else {
            visual_theme::warning()
        },
    );
    if support > 0 {
        draw_text(
            if ctx.session.workspace_drones_deployed() {
                "FIELD MESH LIVE"
            } else {
                "FIELD MESH RECALLED"
            },
            layout.command.x + 180.0,
            layout.command.y + 124.0,
            10.0,
            if ctx.session.workspace_drones_deployed() {
                visual_theme::cyan()
            } else {
                visual_theme::text_dim()
            },
        );
    }
}

fn directive_button_label(support: i32, directive: DroneDirective) -> String {
    if support <= 0 {
        "DRONE BAY OFFLINE".to_owned()
    } else {
        format!("DRONE // {}", directive.short_label())
    }
}

#[cfg(test)]
mod tests;
