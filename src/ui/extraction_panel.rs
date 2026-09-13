//! A compact extraction inspector and an on-demand specification sheet.

use super::scene_layout::SalvageLayout;
use super::*;
use crate::engine::exposure_label;
use crate::state::workspace::{TransferMode, WORKSPACE_STABILIZATION_ENERGY_COST};

pub fn draw_target_panel(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    let frame = layout.target_panel;
    visual_theme::surface(frame);
    let Some(target_id) = ctx.workspace_selected_target else {
        draw_empty_target_panel(ctx, frame);
        return;
    };
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    let unblocked = draw_target_summary(ctx, frame, target_id, target);
    draw_commands(ctx, frame, target_id, unblocked, actions);
}

fn draw_empty_target_panel(ctx: &UiContext<'_>, frame: Rect) {
    let copy = &ctx.data.salvage_ui;
    visual_theme::body(
        &copy.select_target,
        Rect::new(frame.x + 18.0, frame.y + 8.0, 390.0, 32.0),
        26.0,
        visual_theme::text(),
    );
    visual_theme::body(
        if ctx.workspace_scanned {
            &copy.select_hint
        } else {
            &copy.scan_hint
        },
        Rect::new(frame.x + 18.0, frame.y + 48.0, 430.0, 70.0),
        22.0,
        visual_theme::text_dim(),
    );
    draw_objective(
        ctx,
        Rect::new(frame.x + 476.0, frame.y + 18.0, 338.0, 106.0),
    );
}

fn draw_target_summary(
    ctx: &UiContext<'_>,
    frame: Rect,
    target_id: &str,
    target: &crate::data::SalvageObjectData,
) -> bool {
    let copy = &ctx.data.salvage_ui;
    let name = if target.workspace_name.is_empty() {
        &target.display_name
    } else {
        &target.workspace_name
    };
    visual_theme::body(
        name,
        Rect::new(frame.x + 18.0, frame.y + 8.0, 300.0, 35.0),
        26.0,
        visual_theme::text(),
    );
    visual_theme::body(
        &format!(
            "{} {} cr  /  {} {}",
            copy.value, target.sale_value, copy.power, target.energy_cost
        ),
        Rect::new(frame.x + 18.0, frame.y + 46.0, 300.0, 30.0),
        21.0,
        visual_theme::text(),
    );
    let mode = TransferMode::from_target(target);
    let duration = ctx
        .session
        .extraction_duration(target_id, ctx.data)
        .unwrap_or(target.extraction_duration);
    visual_theme::body(
        &format!(
            "{:.1}t  /  {:.1}s  /  {}",
            target.mass_tons,
            duration,
            mode.short_label()
        ),
        Rect::new(frame.x + 18.0, frame.y + 82.0, 300.0, 28.0),
        20.0,
        visual_theme::text_dim(),
    );
    let risk = ctx.workspace_risk.map_or(
        if target.hazard.is_some() {
            "ELEVATED"
        } else {
            "LOW"
        },
        |report| exposure_label(report.exposure),
    );
    visual_theme::body(
        &format!("{} {}", copy.danger, risk),
        Rect::new(frame.x + 330.0, frame.y + 10.0, 228.0, 30.0),
        23.0,
        if target.hazard.is_some() {
            visual_theme::warning()
        } else {
            visual_theme::text()
        },
    );
    let blocked = ctx
        .session
        .extraction_block_reason(target_id, ctx.data)
        .ok()
        .flatten();
    let hazard = if ctx.session.target_is_stabilized(target_id) {
        "STABILIZED".to_owned()
    } else {
        target
            .hazard
            .as_deref()
            .map(hazard_label)
            .unwrap_or_else(|| copy.no_hazard.clone())
    };
    visual_theme::body(
        blocked.as_deref().unwrap_or(&hazard),
        Rect::new(frame.x + 330.0, frame.y + 48.0, 230.0, 78.0),
        19.0,
        if blocked.is_some() || target.hazard.is_some() {
            visual_theme::warning()
        } else {
            visual_theme::text_dim()
        },
    );
    blocked.is_none()
}

fn draw_objective(ctx: &UiContext<'_>, rect: Rect) {
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    let Some(objective) = ctx
        .session
        .contract_objective_status(&expedition.site_id, ctx.data)
    else {
        return;
    };
    let target = ctx.data.salvage_objects.get(&objective.target_id);
    let name = target.map_or(objective.target_id.as_str(), |value| {
        value.display_name.as_str()
    });
    visual_theme::body(
        &format!(
            "{}: {}\n{}",
            ctx.data.salvage_ui.objective,
            name,
            objective.state.label()
        ),
        rect,
        22.0,
        visual_theme::text(),
    );
}

fn draw_commands(
    ctx: &UiContext<'_>,
    frame: Rect,
    target_id: &str,
    unblocked: bool,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.salvage_ui;
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    let active = ctx.workspace_extraction_target.is_some();
    let removed = ctx.session.target_is_removed(target_id);
    let recovered = ctx.session.expedition.as_ref().is_some_and(|expedition| {
        expedition.cargo.iter().any(|item| {
            item.object_id == target_id
                && matches!(item.status, CargoStatus::Pending | CargoStatus::Packed)
        })
    });
    let can_stabilize = target.hazard.is_some()
        && !ctx.session.target_is_stabilized(target_id)
        && ctx.session.has_capability("stabilizer", ctx.data)
        && ctx
            .session
            .workspace_energy()
            .is_some_and(|(remaining, _)| remaining >= WORKSPACE_STABILIZATION_ENERGY_COST);
    let primary = Rect::new(frame.right() - 262.0, frame.y + 16.0, 244.0, 48.0);
    if active {
        draw_rectangle(
            primary.x,
            primary.y,
            primary.w,
            primary.h,
            visual_theme::structure_dark(),
        );
        draw_rectangle(
            primary.x,
            primary.bottom() - 5.0,
            primary.w * ctx.workspace_extraction_progress.clamp(0.0, 1.0),
            5.0,
            visual_theme::cyan(),
        );
        visual_theme::body(
            ctx.workspace_extraction_phase
                .map_or("EXTRACTION", |phase| {
                    TransferMode::from_target(target).phase_label(phase)
                }),
            Rect::new(primary.x + 10.0, primary.y + 7.0, primary.w - 20.0, 32.0),
            18.0,
            visual_theme::text(),
        );
    } else if button(
        ctx,
        primary,
        if removed {
            if recovered {
                &copy.recovered
            } else {
                &copy.removed
            }
        } else if can_stabilize {
            &copy.stabilize
        } else {
            &copy.extract
        },
        !removed && (unblocked || can_stabilize),
        ButtonTone::Primary,
    ) {
        actions.push(if can_stabilize {
            UiAction::Stabilize(target_id.to_owned())
        } else {
            UiAction::Extract(target_id.to_owned())
        });
    }
    if button(
        ctx,
        Rect::new(primary.x, frame.y + 82.0, 108.0, 44.0),
        &copy.details,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleTargetDetails);
    }
    if button(
        ctx,
        Rect::new(primary.x + 118.0, frame.y + 82.0, 126.0, 44.0),
        &copy.abandon,
        !active && !removed,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::AbandonTarget);
    }
}

pub fn draw_details(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let Some(target_id) = ctx.workspace_selected_target else {
        return;
    };
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    let copy = &ctx.data.salvage_ui;
    draw_rectangle(
        0.0,
        84.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 84.0,
        visual_theme::with_alpha(BLACK, 0.76),
    );
    let frame = Rect::new(240.0, 120.0, 800.0, 536.0);
    visual_theme::surface(frame);
    visual_theme::body(
        &copy.inspection,
        Rect::new(264.0, 138.0, 570.0, 32.0),
        20.0,
        visual_theme::text_dim(),
    );
    visual_theme::body(
        &target.display_name,
        Rect::new(264.0, 178.0, 700.0, 42.0),
        32.0,
        visual_theme::text(),
    );
    let duration = ctx
        .session
        .extraction_duration(target_id, ctx.data)
        .unwrap_or(target.extraction_duration);
    let stats = format!(
        "{}: {}%\n{}: {:.1}t\n{}: {} cr\n{}: {:.1}s\n{}: {}\n{}: {:.1}t",
        copy.integrity,
        target.integrity,
        copy.mass,
        target.mass_tons,
        copy.value,
        target.sale_value,
        copy.duration,
        duration,
        copy.power,
        target.energy_cost,
        copy.tractor,
        ctx.session.tractor_capacity_tons(ctx.data)
    );
    visual_theme::body(
        &stats,
        Rect::new(264.0, 238.0, 340.0, 216.0),
        24.0,
        visual_theme::text(),
    );
    let mode = TransferMode::from_target(target);
    let mut details = format!("{} / {}\n", mode.short_label(), mode.destination_label());
    if let Some(report) = ctx.workspace_risk {
        details.push_str(&format!(
            "{} {} / {} {}\n",
            copy.exposure, report.exposure, copy.mitigation, report.mitigation
        ));
    }
    if let Some(hazard) = &target.hazard {
        details.push_str(&format!(
            "{}\n{}\n",
            hazard_label(hazard),
            target.hazard_consequence
        ));
    }
    if let Ok(condition) = ctx.session.workspace_condition_status(ctx.data) {
        details.push_str(&format!(
            "{}: {}%\n",
            copy.condition, condition.section_condition
        ));
    }
    if let Some(expedition) = &ctx.session.expedition {
        details.push_str(&format!("{}\n", expedition.scan_profile.result_label()));
    }
    if ctx.session.workspace_drones_deployed() {
        details.push_str(ctx.session.workspace_drone_directive().description());
    }
    visual_theme::body(
        &details,
        Rect::new(638.0, 238.0, 372.0, 340.0),
        22.0,
        visual_theme::text_dim(),
    );
    draw_objective(ctx, Rect::new(264.0, 466.0, 342.0, 112.0));
    if button(
        ctx,
        Rect::new(866.0, 596.0, 150.0, 44.0),
        &copy.close,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleTargetDetails);
    }
}
