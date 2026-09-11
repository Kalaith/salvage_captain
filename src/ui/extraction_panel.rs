//! Compact target readout and visible extraction commands.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use super::*;
use crate::engine::WorkspaceHazard;
use crate::engine::{exposure_label, WorkspaceOutcome};
use crate::state::workspace::{TransferMode, WORKSPACE_STABILIZATION_ENERGY_COST};
use crate::state::{DroneDirective, WorkspaceScanProfile};

const STABILIZE_COMMAND_LABEL: &str = "STABILIZE  -2P";


pub fn draw_target_panel(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    let Some(target_id) = ctx.workspace_selected_target else {
        return;
    };
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    let survey_note = ctx.session.workspace_survey_note(target_id, ctx.data);
    panel(layout.target_panel, visual_theme::panel());
    let target_name = if target.workspace_name.is_empty() {
        target.display_name.as_str()
    } else {
        target.workspace_name.as_str()
    };
    draw_text(
        &target_name.to_uppercase(),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 29.0,
        17.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "{}  //  {}",
            target.category.to_uppercase(),
            survey_memory_label(survey_note)
        ),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 51.0,
        13.0,
        visual_theme::site_accent(&site_theme(ctx)),
    );
    let transfer_mode = TransferMode::from_target(target);
    let scan_profile = ctx
        .session
        .expedition
        .as_ref()
        .map_or(WorkspaceScanProfile::Standard, |expedition| {
            expedition.scan_profile
        });
    let drone_directive = ctx.session.workspace_drone_directive();
    draw_text(
        format!(
            "TRANSFER      {}  //  {}",
            transfer_mode.short_label(),
            transfer_mode.destination_label()
        ),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 68.0,
        11.0,
        visual_theme::amber(),
    );
    draw_text(
        format!("INTEGRITY     {}%", target.integrity),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 82.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("MASS          {:.1}t", target.mass_tons),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 103.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("EST. VALUE    ~{} cr", target.sale_value),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 124.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "EXTRACTION    {:.1}s{}",
            ctx.session
                .extraction_duration(target_id, ctx.data)
                .unwrap_or(target.extraction_duration),
            extraction_support_label(
                scan_profile,
                drone_directive,
                ctx.session.workspace_drones_deployed(),
            )
        ),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 145.0,
        14.0,
        visual_theme::text(),
    );
    let risk_label = risk_label_for_target(target, ctx.workspace_risk);
    let intelligence_suffix = workspace_intelligence_suffix(
        ctx.session.expedition.as_ref().map_or(0, |expedition| {
            ctx.session.reconnaissance_level(&expedition.site_id)
        }),
        ctx.data.config.reconnaissance.danger_reduction_per_level,
    );
    let risk_readout = hazard_signal_for_target(target, ctx.workspace_risk).map_or_else(
        || format!("RISK          {risk_label}{intelligence_suffix}"),
        |signal| format!("RISK          {risk_label}{intelligence_suffix}  //  {signal}"),
    );
    draw_text(
        clipped(&risk_readout, 38),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 166.0,
        14.0,
        if target.hazard.is_some() {
            visual_theme::warning()
        } else {
            visual_theme::safe()
        },
    );
    visual_theme::draw_meter(
        Rect::new(
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 181.0,
            layout.target_panel.w - 32.0,
            24.0,
        ),
        (ctx.session.tractor_capacity_tons(ctx.data) / target.mass_tons).clamp(0.0, 1.0),
        if ctx.session.tractor_capacity_tons(ctx.data) >= target.mass_tons {
            visual_theme::cyan()
        } else {
            visual_theme::warning()
        },
        &format!(
            "TRACTOR  {:.0} / {:.0}t",
            ctx.session.tractor_capacity_tons(ctx.data),
            target.mass_tons
        ),
    );
    let power_color =
        ctx.session
            .workspace_energy()
            .map_or(visual_theme::warning(), |(remaining, _)| {
                if remaining >= target.energy_cost {
                    visual_theme::cyan()
                } else {
                    visual_theme::warning()
                }
            });
    draw_text(
        format!("PULL POWER   {} units", target.energy_cost),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 218.0,
        12.0,
        power_color,
    );
    let stabilized = ctx.session.target_is_stabilized(target_id);
    let can_stabilize = target.hazard.is_some()
        && !stabilized
        && ctx.session.has_capability("stabilizer", ctx.data)
        && ctx.workspace_extraction_target.is_none()
        && ctx
            .session
            .workspace_energy()
            .is_some_and(|(remaining, _)| remaining >= WORKSPACE_STABILIZATION_ENERGY_COST);
    if let Some(hazard) = &target.hazard {
        draw_text(
            hazard_readout(hazard, stabilized),
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 229.0,
            12.0,
            visual_theme::warning(),
        );
        draw_text(
            clipped(&target.hazard_consequence, 34),
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 247.0,
            11.0,
            visual_theme::text_dim(),
        );
    }
    let report_y = layout.target_panel.y + 270.0;
    if ctx.workspace_extraction_target.is_none() {
        if let Some(report) = ctx.workspace_risk {
            let report_color = match report.outcome {
                WorkspaceOutcome::Recovered => visual_theme::safe(),
                WorkspaceOutcome::DamagedHull | WorkspaceOutcome::LostTarget => {
                    visual_theme::warning()
                }
            };
            draw_text(
                format!(
                    "EXPOSURE  {:02}  /  MITIGATION  {:02}  //  {}",
                    report.exposure,
                    report.mitigation,
                    exposure_label(report.exposure)
                ),
                layout.target_panel.x + 16.0,
                report_y,
                11.0,
                report_color,
            );
            draw_text(
                drone_risk_label(
                    ctx.session.workspace_drone_directive(),
                    ctx.session.workspace_drones_deployed(),
                ),
                layout.target_panel.x + 16.0,
                report_y + 14.0,
                10.0,
                report_color,
            );
            draw_text(
                crew_watch_label(ctx.session),
                layout.target_panel.x + 16.0,
                report_y + 28.0,
                10.0,
                report_color,
            );
        }
    }
    if let Some(extraction_target) = ctx.workspace_extraction_target {
        if extraction_target == target_id {
            draw_text(
                ctx.workspace_extraction_phase
                    .map_or("STANDING BY", |phase| transfer_mode.phase_label(phase)),
                layout.target_panel.x + 16.0,
                layout.target_panel.y + 278.0,
                14.0,
                visual_theme::cyan(),
            );
            visual_theme::draw_meter(
                Rect::new(
                    layout.target_panel.x + 16.0,
                    layout.target_panel.y + 288.0,
                    layout.target_panel.w - 32.0,
                    22.0,
                ),
                ctx.workspace_extraction_progress,
                visual_theme::cyan(),
                "EXTRACTION",
            );
            return;
        }
    }
    let blocked = ctx
        .session
        .extraction_block_reason(target_id, ctx.data)
        .ok()
        .flatten();
    let power_cycle_command = blocked
        .as_deref()
        .is_some_and(|reason| reason.starts_with("Power reserve insufficient"))
        && ctx.session.can_power_cycle_workspace(ctx.data)
        && ctx.workspace_extraction_target.is_none();
    if let Some(reason) = blocked.as_deref() {
        draw_text(
            clipped(reason, 30),
            layout.target_panel.x + 16.0,
            layout.target_panel.y
                + if ctx.workspace_risk.is_some() {
                    286.0
                } else {
                    278.0
                },
            12.0,
            visual_theme::warning(),
        );
    }
    let button_y = layout.target_panel.y
        + if ctx.workspace_risk.is_some() {
            326.0
        } else {
            292.0
        };
    let primary_label = if can_stabilize {
        STABILIZE_COMMAND_LABEL
    } else if power_cycle_command {
        "POWER CYCLE"
    } else {
        transfer_mode.command_label()
    };
    if button(
        ctx,
        Rect::new(layout.target_panel.x + 16.0, button_y, 126.0, 44.0),
        primary_label,
        blocked.is_none() || power_cycle_command,
        ButtonTone::Primary,
    ) {
        actions.push(if can_stabilize {
            UiAction::Stabilize(target_id.to_owned())
        } else if power_cycle_command {
            UiAction::PowerCycle
        } else {
            UiAction::Extract(target_id.to_owned())
        });
    }
    if button(
        ctx,
        Rect::new(layout.target_panel.x + 148.0, button_y, 110.0, 44.0),
        "ABANDON",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::AbandonTarget);
    }
}

fn extraction_support_label(
    scan_profile: WorkspaceScanProfile,
    directive: DroneDirective,
    drones_active: bool,
) -> &'static str {
    if !drones_active {
        return match scan_profile {
            WorkspaceScanProfile::Array => "  //  ARRAY",
            WorkspaceScanProfile::Standard => "",
        };
    }
    match (scan_profile, directive) {
        (WorkspaceScanProfile::Array, DroneDirective::Survey) => "  //  ARRAY+SURVEY",
        (WorkspaceScanProfile::Array, DroneDirective::PullSupport) => "  //  ARRAY+PULL",
        (WorkspaceScanProfile::Standard, DroneDirective::Survey) => "  //  SURVEY NET",
        (WorkspaceScanProfile::Standard, DroneDirective::PullSupport) => "  //  PULL SUPPORT",
        (_, DroneDirective::Standby) => "  //  STANDBY",
    }
}

fn drone_risk_label(directive: DroneDirective, drones_active: bool) -> String {
    match (drones_active, directive) {
        (true, DroneDirective::Survey) => "DRONE ORDER  SURVEY // SAFER".to_owned(),
        (true, DroneDirective::PullSupport) => "DRONE ORDER  PULL // FASTER".to_owned(),
        (_, DroneDirective::Standby) => "DRONE ORDER  STANDBY // FULL LOAD".to_owned(),
        (false, _) => "DRONE ORDER  RECALLING // FULL LOAD".to_owned(),
    }
}

fn crew_watch_label(session: &GameSession) -> String {
    format!(
        "CREW WATCH  //  READY {}%  //  ROUTE RISK {:+}",
        session.crew_readiness(),
        session.crew_fatigue_danger_delta()
    )
}

fn survey_memory_label(note: Option<&crate::state::TargetSurveyNote>) -> String {
    note.map_or_else(
        || "SURVEY NEW".to_owned(),
        |note| {
            format!(
                "SURVEY MEM {:02} // D{:02}",
                note.scan_count, note.extraction_difficulty
            )
        },
    )
}

fn hazard_readout(value: &str, stabilized: bool) -> String {
    let Some(hazard) = WorkspaceHazard::from_value(value) else {
        return format!("HAZARD  {}", clipped(&hazard_label(value), 24));
    };
    let compact_label = match hazard {
        WorkspaceHazard::ReactorInstability => "REACTOR",
        WorkspaceHazard::ElectricalArcs => "ELECTRICAL",
        WorkspaceHazard::AutomatedDefenses => "DEFENSE",
        WorkspaceHazard::UnexplodedAmmunition => "ORDNANCE",
        WorkspaceHazard::MagneticInterference => "MAGNETIC",
        WorkspaceHazard::StructuralCollapse => "STRUCTURAL",
    };
    if stabilized {
        format!("HAZARD  {compact_label} // STABILIZED")
    } else {
        format!("HAZARD  {compact_label} // {}", hazard.response_label())
    }
}

fn hazard_signal_for_target(
    target: &crate::data::SalvageObjectData,
    report: Option<&crate::engine::WorkspaceRiskReport>,
) -> Option<&'static str> {
    report
        .and_then(|report| report.hazard)
        .or_else(|| {
            target
                .hazard
                .as_deref()
                .and_then(WorkspaceHazard::from_value)
        })
        .map(WorkspaceHazard::response_label)
}

fn site_theme(ctx: &UiContext<'_>) -> String {
    ctx.session
        .workspace_site(ctx.data)
        .map(|site| site.visual_theme.clone())
        .unwrap_or_else(|_| "merchant".to_owned())
}

fn risk_label_for_target(
    target: &crate::data::SalvageObjectData,
    report: Option<&crate::engine::WorkspaceRiskReport>,
) -> &'static str {
    if let Some(report) = report {
        return exposure_label(report.exposure);
    }
    if target.hazard.is_some() {
        "ELEVATED"
    } else if target.extraction_difficulty >= 50 {
        "MEDIUM"
    } else {
        "LOW"
    }
}

fn workspace_intelligence_suffix(level: u8, danger_reduction_per_level: i32) -> String {
    if level == 0 {
        String::new()
    } else {
        format!(
            " // INTEL -{}",
            i32::from(level) * danger_reduction_per_level
        )
    }
}
