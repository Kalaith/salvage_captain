//! Touch-first control for the operator's active drone directive.

use super::scene_layout::SalvageLayout;
use super::{button, ButtonTone, UiAction, UiContext};
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
        layout.command.x + 178.0,
        layout.command.y + 98.0,
        180.0,
        44.0,
    );
    let can_change = support > 0
        && ctx.workspace_camera_shift >= 1.0
        && ctx.workspace_scan_progress <= 0.0
        && ctx.workspace_extraction_target.is_none();
    let label = directive_button_label(support, directive);
    if button(ctx, rect, &label, can_change, ButtonTone::Secondary) {
        actions.push(UiAction::CycleDroneDirective);
    }
}

fn directive_button_label(support: i32, directive: DroneDirective) -> String {
    if support <= 0 {
        "DRONE BAY OFFLINE".to_owned()
    } else {
        format!("DRONE // {}", directive.short_label())
    }
}
