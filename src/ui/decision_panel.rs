//! Yard debrief: every recovered object becomes a deliberate captain's choice.

use super::*;
use crate::state::workspace::TransferMode;
use crate::state::{CrewRole, DroneDirective, ReturnPolicy, VoyageRecord, WorkspaceScanProfile};
use crate::ui::ship_visual;
use crate::ui::visual_theme;
mod manifest;

struct DebriefRunSummary<'a> {
    run_number: usize,
    site_name: &'a str,
    scan_profile: WorkspaceScanProfile,
    voyage_plan: crate::engine::VoyagePlan,
    crew_role: CrewRole,
    crew_readiness: u8,
    return_policy: ReturnPolicy,
    drone_directive: DroneDirective,
    reconnaissance_level: u8,
    return_fuel: i32,
}
struct DebriefMemorySummary<'a> {
    ship_wear: u8,
    service_cost: i64,
    unlocked_blueprints: usize,
    total_blueprints: usize,
    standing_progress: &'a str,
    recovered_count: u32,
    external_load: u32,
    recovered_value: i64,
    log_count: usize,
    survey_count: usize,
    cleared_sections: usize,
    clearance_payout: i64,
}

pub fn draw_results(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let frame = Rect::new(0.0, 84.0, 1280.0, 636.0);
    panel(frame, visual_theme::panel_soft());
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        42.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "YARD DEBRIEF  //  RETURNED CARGO",
        frame.x + 18.0,
        frame.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "SELL  /  INSTALL  /  BREAK DOWN",
        frame.right() - 224.0,
        frame.y + 27.0,
        11.0,
        visual_theme::amber(),
    );
    draw_debrief(ctx);
    draw_yard_preview(ctx);
    manifest::draw_result_manifest(ctx, actions);
}

fn draw_debrief(ctx: &UiContext<'_>) {
    draw_debrief_risk(ctx);
    draw_debrief_summary(ctx);
    draw_debrief_contract(ctx);
    draw_debrief_clearance(ctx);
}

fn draw_debrief_risk(ctx: &UiContext<'_>) {
    let risk = ctx.session.last_risk.as_ref();
    let accent = risk.map_or(visual_theme::safe(), |risk| match risk.outcome {
        RiskOutcome::OrdinaryReturn => visual_theme::safe(),
        _ => visual_theme::warning(),
    });
    draw_text(
        risk.map_or("NO FLIGHT REPORT", |risk| risk_label(risk.outcome)),
        50.0,
        184.0,
        17.0,
        accent,
    );
    if let Some(risk) = risk {
        draw_text(
            clipped(&risk.explanation, 72),
            50.0,
            208.0,
            14.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_debrief_summary(ctx: &UiContext<'_>) {
    if let Some(record) = ctx.session.last_voyage() {
        let site_name = ctx
            .data
            .sites
            .get(&record.site_id)
            .map_or(record.site_id.as_str(), |site| site.display_name.as_str());
        let log_count = ctx
            .session
            .site_progress
            .get(&record.site_id)
            .map_or(0, |progress| progress.operation_log.len());
        let survey_count = ctx.session.site_survey_count(&record.site_id);
        let scan_profile = record.scan_profile;
        let unlocked_blueprints = ctx.session.unlocked_module_count(ctx.data);
        let total_blueprints = ctx.data.modules.iter().count();
        let standing_progress = debrief_standing_label(ctx.session);
        draw_text(
            clipped(
                &debrief_run_label(&DebriefRunSummary {
                    run_number: ctx.session.voyage_log.len(),
                    site_name: &site_name.to_uppercase(),
                    scan_profile,
                    voyage_plan: record.voyage_plan,
                    crew_role: ctx.session.crew_role(),
                    crew_readiness: ctx.session.crew_readiness(),
                    return_policy: ctx.session.last_return_policy,
                    drone_directive: record.drone_directive,
                    reconnaissance_level: record.reconnaissance_level,
                    return_fuel: record.return_fuel,
                }),
                118,
            ),
            50.0,
            236.0,
            12.0,
            visual_theme::cyan(),
        );
        draw_text(
            clipped(
                &debrief_memory_label(&DebriefMemorySummary {
                    ship_wear: ctx.session.ship_wear(),
                    service_cost: ctx.session.maintenance_cost(ctx.data),
                    unlocked_blueprints,
                    total_blueprints,
                    standing_progress: &standing_progress,
                    recovered_count: record.recovered_count,
                    external_load: record.external_load,
                    recovered_value: record.recovered_value,
                    log_count,
                    survey_count,
                    cleared_sections: record.cleared_sections.len(),
                    clearance_payout: record.clearance_payout,
                }),
                118,
            ),
            50.0,
            252.0,
            12.0,
            visual_theme::cyan(),
        );
    } else {
        draw_text(
            "The yard can turn this haul into capability, cash, or raw stock.",
            50.0,
            236.0,
            13.0,
            visual_theme::text(),
        );
    }
}

fn draw_debrief_contract(ctx: &UiContext<'_>) {
    if let Some(site_id) = &ctx.session.selected_site {
        if let Some(site) = ctx.data.sites.get(site_id) {
            let objective = ctx.session.contract_objective_status(site_id, ctx.data);
            let target_name = objective.as_ref().map_or_else(
                || "UNKNOWN OBJECTIVE".to_owned(),
                |objective| {
                    ctx.data.salvage_objects.get(&objective.target_id).map_or(
                        objective.target_id.clone(),
                        |target| {
                            if target.workspace_name.is_empty() {
                                target.display_name.clone()
                            } else {
                                target.workspace_name.clone()
                            }
                        },
                    )
                },
            );
            let private_haul = ctx
                .session
                .last_voyage()
                .is_some_and(|record| record.site_id == *site_id && !record.contract_accepted);
            let contract_label = if private_haul {
                "PRIVATE HAUL  //  CONTRACT DECLINED  //  STREAK HELD".to_owned()
            } else {
                match objective.as_ref().map(|objective| objective.state) {
                    Some(crate::state::contracts::ContractObjectiveState::Complete) => format!(
                    "CONTRACT COMPLETE  //  OBJECTIVE {}  //  BONUS +{} CREDITS  //  STREAK x{}",
                    target_name.to_uppercase(),
                    site.contract_reward,
                    ctx.session.contract_streak()
                    ),
                    Some(crate::state::contracts::ContractObjectiveState::Failed) => format!(
                        "CONTRACT FAILED  //  OBJECTIVE {} LOST  //  NO BONUS  //  STREAK RESET",
                        target_name.to_uppercase()
                    ),
                    _ => format!(
                        "CONTRACT OPEN  //  OBJECTIVE {}  //  +{} CREDITS  //  NEXT STREAK +¢{}",
                        target_name.to_uppercase(),
                        site.contract_reward,
                        ctx.session.next_contract_streak_bonus()
                    ),
                }
            };
            let contract_color = if private_haul {
                visual_theme::cyan()
            } else {
                match objective.as_ref().map(|objective| objective.state) {
                    Some(crate::state::contracts::ContractObjectiveState::Complete) => {
                        visual_theme::safe()
                    }
                    Some(crate::state::contracts::ContractObjectiveState::Failed) => {
                        visual_theme::warning()
                    }
                    _ => visual_theme::amber(),
                }
            };
            let contract_readout = ctx.session.last_voyage().map_or_else(
                || contract_label.clone(),
                |record| format!("{contract_label}  //  {}", insurance_debrief_label(record)),
            );
            draw_text(contract_readout, 50.0, 276.0, 12.0, contract_color);
        }
    }
}

fn draw_debrief_clearance(ctx: &UiContext<'_>) {
    if let Some(record) = ctx.session.last_voyage() {
        draw_text(
            clearance_settlement_label(record),
            50.0,
            292.0,
            12.0,
            if record.clearance_payout > 0 {
                visual_theme::safe()
            } else {
                visual_theme::text_dim()
            },
        );
    }
}

fn clearance_settlement_label(record: &VoyageRecord) -> String {
    format!(
        "FRAME CLEARANCE  //  {} CLEARED  //  BOUNTY +¢{}",
        record.cleared_sections.len(),
        record.clearance_payout
    )
}

fn insurance_debrief_label(record: &VoyageRecord) -> String {
    if !record.insured {
        return "COVER  NONE  //  SELF-INSURED RETURN".to_owned();
    }
    if record.insurance_payout > 0 {
        format!(
            "COVER  ACTIVE  //  PREMIUM ¢{}  //  CLAIM PAID ¢{}  //  {}",
            record.insurance_premium,
            record.insurance_payout,
            insurance_balance_label(record.insurance_premium, record.insurance_payout)
        )
    } else {
        format!(
            "COVER  ACTIVE  //  PREMIUM ¢{}  //  NO CLAIM FILED  //  {}",
            record.insurance_premium,
            insurance_balance_label(record.insurance_premium, record.insurance_payout)
        )
    }
}

fn insurance_balance_label(premium: i64, payout: i64) -> String {
    let balance = payout - premium;
    if balance >= 0 {
        format!("NET +¢{balance}")
    } else {
        format!("NET -¢{}", balance.abs())
    }
}

fn debrief_run_label(summary: &DebriefRunSummary<'_>) -> String {
    format!(
        "RUN {}  //  PLAN {}  //  CREW {}  //  READY {}%  //  POLICY {}  //  {}  //  {}  //  DRONE {}  //  SCAN {}  //  RETURN {} FUEL",
        summary.run_number,
        summary.voyage_plan.label(),
        summary.crew_role.short_label(),
        summary.crew_readiness,
        summary.return_policy.short_label(),
        debrief_intelligence_label(summary.reconnaissance_level),
        summary.site_name,
        summary.drone_directive.short_label(),
        summary.scan_profile.short_label(),
        summary.return_fuel
    )
}

fn debrief_memory_label(summary: &DebriefMemorySummary<'_>) -> String {
    format!(
        "WEAR {}%  //  SERVICE ¢{}  //  BP {:02}/{:02}  //  {}  //  RECOV {}  //  EXT {}  //  VALUE ¢{}  //  FIELD LOG {:02}  //  SURV {:02}  //  CLEAR {}  //  BOUNTY ¢{}",
        summary.ship_wear,
        summary.service_cost,
        summary.unlocked_blueprints,
        summary.total_blueprints,
        summary.standing_progress,
        summary.recovered_count,
        summary.external_load,
        summary.recovered_value,
        summary.log_count,
        summary.survey_count,
        summary.cleared_sections,
        summary.clearance_payout
    )
}

fn debrief_intelligence_label(level: u8) -> String {
    if level == 0 {
        "INTEL NONE".to_owned()
    } else {
        format!("INTEL L{level}")
    }
}

fn debrief_standing_label(session: &GameSession) -> String {
    let standing = session.salvage_standing();
    session.next_standing_threshold().map_or_else(
        || format!("STAND {} // REP {}", standing.label(), session.reputation),
        |threshold| {
            format!(
                "STAND {} // REP {}/{}",
                standing.label(),
                session.reputation,
                threshold
            )
        },
    )
}

fn draw_yard_preview(ctx: &UiContext<'_>) {
    let preview = Rect::new(910.0, 154.0, 320.0, 146.0);
    panel(preview, visual_theme::panel());
    draw_text(
        "YARD INSTALL PREVIEW",
        preview.x + 16.0,
        preview.y + 24.0,
        12.0,
        visual_theme::text_dim(),
    );
    ship_visual::draw_ship(
        Rect::new(preview.x + 36.0, preview.y + 42.0, 250.0, 82.0),
        ctx.session,
        ctx.data,
        0.0,
        false,
    );
    draw_text(
        "MODULE MOUNTS GLOW WHEN INSTALLABLE",
        preview.x + 16.0,
        preview.bottom() - 10.0,
        10.0,
        visual_theme::cyan(),
    );
}
