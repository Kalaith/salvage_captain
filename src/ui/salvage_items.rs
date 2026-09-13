//! Recovery manifest and hold-packing deck.

use super::*;
use crate::engine::{exposure_label, RiskResult};
use crate::state::workspace::TransferMode;
use crate::state::DroneDirective;
use crate::ui::visual_theme;
mod manifest;

pub fn draw_packing(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_hold_panel(ctx, actions);
    manifest::draw_manifest(ctx, actions);
}

fn draw_hold_panel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let hold = Rect::new(24.0, 84.0, 450.0, 636.0);
    draw_hold_frame(&hold);
    draw_hold_controls(ctx, actions, hold);
    draw_ship_grid(ctx, Rect::new(52.0, 198.0, 394.0, 270.0), true, actions);
    draw_hold_route_status(ctx, hold);
    let risk_preview = ctx.session.expedition_risk_preview(ctx.data);
    draw_hold_risk_status(ctx, hold, risk_preview.as_ref());
    draw_hold_power_status(ctx, hold, risk_preview.as_ref());
}

fn draw_hold_frame(hold: &Rect) {
    panel(*hold, visual_theme::panel_soft());
    draw_rectangle(hold.x, hold.y, hold.w, 42.0, visual_theme::structure_dark());
    draw_text(
        "RETURN HOLD",
        hold.x + 18.0,
        hold.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "REVIEW BEFORE RETURN",
        hold.right() - 178.0,
        hold.y + 27.0,
        10.0,
        visual_theme::amber(),
    );
    draw_text(
        "TAP MOVE, THEN A GRID CELL TO REPOSITION CARGO",
        hold.x + 20.0,
        hold.y + 68.0,
        11.0,
        visual_theme::text_dim(),
    );
}

fn draw_hold_controls(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, hold: Rect) {
    draw_text(
        packing_crew_label(
            ctx.session.external_cargo_count(ctx.data, None),
            ctx.session.external_capacity(ctx.data),
            ctx.session.crew_role(),
            ctx.session.crew_readiness(),
        ),
        hold.x + 20.0,
        hold.y + 88.0,
        11.0,
        visual_theme::amber(),
    );
    let return_policy = ctx
        .session
        .return_policy()
        .unwrap_or(crate::state::ReturnPolicy::Standard);
    if button(
        ctx,
        Rect::new(hold.right() - 154.0, hold.y + 70.0, 134.0, 24.0),
        &return_policy_button_label(return_policy),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleReturnPolicy);
    }
    draw_text(
        clipped(return_policy.description(), 24),
        hold.right() - 154.0,
        hold.y + 100.0,
        9.0,
        visual_theme::cyan(),
    );
    let (cargo_count, clamp_count, tow_count) = transfer_counts(ctx);
    draw_text(
        packing_cargo_capacity_label(cargo_count, ctx.session.internal_cargo_capacity()),
        hold.x + 20.0,
        hold.y + 106.0,
        10.0,
        if cargo_count as i32 > ctx.session.internal_cargo_capacity() {
            visual_theme::warning()
        } else {
            visual_theme::cyan()
        },
    );
    draw_text(
        format!("CLAMP {:02}", clamp_count),
        hold.x + 116.0,
        hold.y + 106.0,
        10.0,
        visual_theme::amber(),
    );
    draw_text(
        format!("TOW {:02}", tow_count),
        hold.x + 218.0,
        hold.y + 106.0,
        10.0,
        visual_theme::warning(),
    );
}

fn draw_hold_route_status(ctx: &UiContext<'_>, hold: Rect) {
    let site_label =
        ctx.session
            .expedition
            .as_ref()
            .map_or("UNKNOWN SITE".to_owned(), |expedition| {
                ctx.data.sites.get(&expedition.site_id).map_or_else(
                    || expedition.site_id.clone(),
                    |site| site.display_name.clone(),
                )
            });
    let route_label = ctx
        .session
        .expedition
        .as_ref()
        .map_or("ROUTE --".to_owned(), |expedition| {
            packing_route_label(ctx.session, &expedition.site_id)
        });
    draw_text(
        format!("{}  //  {route_label}", site_label.to_uppercase()),
        hold.x + 20.0,
        hold.y + 418.0,
        15.0,
        visual_theme::text(),
    );
    let objective = ctx.session.expedition.as_ref().and_then(|expedition| {
        ctx.session
            .contract_objective_status(&expedition.site_id, ctx.data)
    });
    if let Some(objective) = objective {
        draw_hold_objective(ctx, hold, &objective);
    } else if ctx
        .session
        .expedition
        .as_ref()
        .is_some_and(|expedition| !expedition.contract_accepted)
    {
        draw_text(
            "PRIVATE HAUL  //  CONTRACT DECLINED  //  STREAK HELD",
            hold.x + 20.0,
            hold.y + 436.0,
            11.0,
            visual_theme::cyan(),
        );
    }
}

fn draw_hold_objective(
    ctx: &UiContext<'_>,
    hold: Rect,
    objective: &crate::state::contracts::ContractObjectiveStatus,
) {
    let target_name = ctx.data.salvage_objects.get(&objective.target_id).map_or(
        objective.target_id.clone(),
        |target| {
            if target.workspace_name.is_empty() {
                target.display_name.clone()
            } else {
                target.workspace_name.clone()
            }
        },
    );
    draw_text(
        format!(
            "OBJECTIVE {}  //  {}",
            objective.state.label(),
            clipped(&target_name.to_uppercase(), 30)
        ),
        hold.x + 20.0,
        hold.y + 436.0,
        11.0,
        match objective.state {
            crate::state::contracts::ContractObjectiveState::Failed => visual_theme::warning(),
            crate::state::contracts::ContractObjectiveState::Complete => visual_theme::safe(),
            crate::state::contracts::ContractObjectiveState::Open
            | crate::state::contracts::ContractObjectiveState::Recovered => visual_theme::amber(),
        },
    );
}

fn draw_hold_risk_status(ctx: &UiContext<'_>, hold: Rect, risk_preview: Option<&RiskResult>) {
    let risk = risk_preview.map_or(0, |preview| preview.danger_score);
    let external_load = ctx.session.external_cargo_count(ctx.data, None);
    let return_policy = ctx
        .session
        .return_policy()
        .unwrap_or(crate::state::ReturnPolicy::Standard);
    draw_text(
        clipped(
            &format!(
                "RISK PREVIEW  {:02}%  //  {}  //  EXT STRAIN +{}  //  {}",
                risk,
                exposure_label(risk),
                external_load,
                return_policy_effect_label(
                    return_policy,
                    risk_preview.map(|preview| preview.outcome),
                )
            ),
            64,
        ),
        hold.x + 20.0,
        hold.y + 454.0,
        12.0,
        danger_color(risk),
    );
    draw_text(
        drone_order_label(
            ctx.session.workspace_drone_directive(),
            ctx.session.workspace_drones_deployed(),
        ),
        hold.x + 20.0,
        hold.y + 474.0,
        11.0,
        visual_theme::cyan(),
    );
    draw_text(
        manifest::packing_coverage_label(ctx, risk_preview),
        hold.x + 20.0,
        hold.y + 492.0,
        12.0,
        if ctx
            .session
            .expedition
            .as_ref()
            .is_some_and(|expedition| expedition.insured)
        {
            visual_theme::safe()
        } else {
            visual_theme::text_dim()
        },
    );
}

fn draw_hold_power_status(ctx: &UiContext<'_>, hold: Rect, risk_preview: Option<&RiskResult>) {
    let power_cycles = ctx
        .session
        .expedition
        .as_ref()
        .map_or(0, |expedition| expedition.power_cycles_used);
    let external_load = ctx.session.external_cargo_count(ctx.data, None);
    draw_text(
        manifest::return_burn_label(
            ctx.session.economy.fuel,
            ctx.data.config.safe_return_buffer.max(0),
        ),
        hold.x + 20.0,
        hold.y + 514.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        manifest::power_cycle_label(power_cycles),
        hold.x + 20.0,
        hold.y + 532.0,
        11.0,
        visual_theme::cyan(),
    );
    draw_text(
        manifest::packing_wear_label(
            ctx.session.ship_wear(),
            risk_preview.map(|preview| preview.outcome),
            external_load,
            power_cycles,
            &ctx.data.config.maintenance,
        ),
        hold.x + 20.0,
        hold.y + 550.0,
        11.0,
        visual_theme::amber(),
    );
    let clearance = ctx
        .session
        .expedition
        .as_ref()
        .map_or((0, 0), |expedition| {
            ctx.session
                .site_clearance_ready_summary(&expedition.site_id, ctx.data)
        });
    draw_text(
        manifest::clearance_forecast_label(clearance.0, clearance.1),
        hold.x + 20.0,
        hold.y + 568.0,
        11.0,
        if clearance.0 > 0 {
            visual_theme::amber()
        } else {
            visual_theme::text_dim()
        },
    );
}

fn transfer_counts(ctx: &UiContext<'_>) -> (usize, usize, usize) {
    let mut counts = (0, 0, 0);
    let Some(expedition) = &ctx.session.expedition else {
        return counts;
    };
    for cargo in expedition
        .cargo
        .iter()
        .filter(|cargo| matches!(cargo.status, CargoStatus::Pending | CargoStatus::Packed))
    {
        let Some(object) = ctx.data.salvage_objects.get(&cargo.object_id) else {
            continue;
        };
        match TransferMode::from_target(object) {
            TransferMode::InternalCargo => counts.0 += 1,
            TransferMode::ExternalClamp => counts.1 += 1,
            TransferMode::Tow => counts.2 += 1,
        }
    }
    counts
}

fn packing_crew_label(
    external_used: i32,
    external_capacity: i32,
    crew_role: crate::state::CrewRole,
    readiness: u8,
) -> String {
    format!(
        "EXTERNAL CLAMPS  {external_used}/{external_capacity}  //  CREW {}  //  READY {}%",
        crew_role.short_label(),
        readiness
    )
}

fn packing_cargo_capacity_label(cargo_count: usize, capacity: i32) -> String {
    format!("CARGO {:02}/{:02}", cargo_count, capacity.max(0))
}

fn return_policy_button_label(policy: crate::state::ReturnPolicy) -> String {
    format!("POLICY  {}", policy.short_label())
}

fn packing_route_label(session: &GameSession, site_id: &str) -> String {
    format!(
        "ROUTE {} // -{}",
        session.route_familiarity_label(site_id),
        session.route_familiarity_danger_reduction(site_id)
    )
}

fn return_policy_effect_label(
    policy: crate::state::ReturnPolicy,
    outcome: Option<crate::engine::RiskOutcome>,
) -> &'static str {
    match outcome {
        Some(crate::engine::RiskOutcome::LostSalvage) => match policy {
            crate::state::ReturnPolicy::Standard => "HIGHEST LOAD AT RISK",
            crate::state::ReturnPolicy::ProtectObjective => "PROTECT OBJECTIVE",
            crate::state::ReturnPolicy::ProtectValue => "SAVE HIGH VALUE",
        },
        Some(crate::engine::RiskOutcome::ForcedAbandon) => match policy {
            crate::state::ReturnPolicy::Standard => "LOWEST LOAD AT RISK",
            crate::state::ReturnPolicy::ProtectObjective => "PROTECT OBJECTIVE",
            crate::state::ReturnPolicy::ProtectValue => "SAVE HIGH VALUE",
        },
        Some(_) => "NO CARGO CASUALTY",
        None => "OUTCOME PENDING",
    }
}

fn drone_order_label(directive: DroneDirective, drones_active: bool) -> String {
    match (drones_active, directive) {
        (true, DroneDirective::Survey) => "DRONE ORDER  SURVEY // SAFETY".to_owned(),
        (true, DroneDirective::PullSupport) => "DRONE ORDER  PULL // SPEED".to_owned(),
        (_, DroneDirective::Standby) => "DRONE ORDER  STANDBY // NO ASSIST".to_owned(),
        (false, _) => "DRONE ORDER  RECALLING // NO ASSIST".to_owned(),
    }
}
