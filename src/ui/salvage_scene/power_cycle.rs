//! Command-console presentation for the one-shot field power recovery.

use super::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::ButtonTone;

pub(super) fn status_label(available: bool, used: u8) -> Option<&'static str> {
    if used > 0 {
        Some("POWER RESET SPENT")
    } else if available {
        Some("POWER RESET READY // FUEL 1")
    } else {
        None
    }
}

pub(super) fn status_label_with_cells(
    cell_available: bool,
    cells: u8,
    reset_available: bool,
    reset_used: u8,
) -> Option<String> {
    if cell_available {
        Some(format!("POWER CELL READY // {cells} STOCKED"))
    } else {
        status_label(reset_available, reset_used).map(str::to_owned)
    }
}

pub(super) fn draw_command_button(
    ctx: &UiContext<'_>,
    layout: SalvageLayout,
    actions: &mut Vec<UiAction>,
    can_scan: bool,
) {
    let scan_power_available = ctx
        .session
        .workspace_energy()
        .is_some_and(|(remaining, _)| remaining >= ctx.data.config.workspace_scan_energy_cost);
    let power_cycle_button = ctx.session.can_power_cycle_workspace(ctx.data)
        && ctx.workspace_camera_shift >= 1.0
        && ctx.workspace_extraction_target.is_none()
        && (ctx.workspace_scanned || !scan_power_available);
    let field_power_button = ctx.session.can_use_field_power_cell()
        && ctx.workspace_camera_shift >= 1.0
        && ctx.workspace_extraction_target.is_none()
        && ctx.workspace_scan_progress <= 0.0
        && (ctx.workspace_scanned || !scan_power_available);
    let label = if ctx.workspace_camera_shift < 1.0 {
        "SHIFTING"
    } else if ctx.workspace_scan_progress > 0.0 {
        "SCANNING"
    } else if field_power_button {
        "FIELD CELL"
    } else if power_cycle_button {
        "POWER CYCLE"
    } else if ctx.workspace_scanned {
        "SCANNED"
    } else if !scan_power_available {
        "NO POWER"
    } else {
        "SCAN"
    };
    if button(
        ctx,
        Rect::new(
            layout.command.x + 16.0,
            layout.command.y + 38.0,
            150.0,
            48.0,
        ),
        label,
        can_scan || field_power_button || power_cycle_button,
        ButtonTone::Primary,
    ) {
        actions.push(if field_power_button {
            UiAction::UseFieldPowerCell
        } else if power_cycle_button {
            UiAction::PowerCycle
        } else {
            UiAction::Scan
        });
    }
}
