//! Yard debrief: every recovered object becomes a deliberate captain's choice.

use super::*;
use crate::state::workspace::TransferMode;
use crate::state::{VoyageRecord, WorkspaceScanProfile};
use crate::ui::ship_visual;
use crate::ui::visual_theme;

#[cfg(test)]
mod tests;

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
    draw_result_manifest(ctx, actions);
}

fn draw_debrief(ctx: &UiContext<'_>) {
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
                &debrief_run_label(
                    ctx.session.voyage_log.len(),
                    &site_name.to_uppercase(),
                    record.recovered_count,
                    record.external_load,
                    record.recovered_value,
                    log_count,
                    survey_count,
                    scan_profile,
                    record.reconnaissance_level,
                    unlocked_blueprints,
                    total_blueprints,
                    &standing_progress,
                ),
                104,
            ),
            50.0,
            236.0,
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
            let contract_label = match objective.as_ref().map(|objective| objective.state) {
                Some(crate::state::contracts::ContractObjectiveState::Complete) => format!(
                    "CONTRACT COMPLETE  //  OBJECTIVE {}  //  BONUS +{} CREDITS",
                    target_name.to_uppercase(),
                    site.contract_reward
                ),
                Some(crate::state::contracts::ContractObjectiveState::Failed) => format!(
                    "CONTRACT FAILED  //  OBJECTIVE {} LOST  //  NO BONUS",
                    target_name.to_uppercase()
                ),
                _ => format!(
                    "CONTRACT OPEN  //  OBJECTIVE {}  //  +{} CREDITS",
                    target_name.to_uppercase(),
                    site.contract_reward
                ),
            };
            let contract_color = match objective.as_ref().map(|objective| objective.state) {
                Some(crate::state::contracts::ContractObjectiveState::Complete) => {
                    visual_theme::safe()
                }
                Some(crate::state::contracts::ContractObjectiveState::Failed) => {
                    visual_theme::warning()
                }
                _ => visual_theme::amber(),
            };
            let contract_readout = ctx.session.last_voyage().map_or_else(
                || contract_label.clone(),
                |record| format!("{contract_label}  //  {}", insurance_debrief_label(record)),
            );
            draw_text(contract_readout, 50.0, 260.0, 12.0, contract_color);
        }
    }
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

fn debrief_run_label(
    run_number: usize,
    site_name: &str,
    recovered_count: u32,
    external_load: u32,
    recovered_value: i64,
    log_count: usize,
    survey_count: usize,
    scan_profile: WorkspaceScanProfile,
    reconnaissance_level: u8,
    unlocked_blueprints: usize,
    total_blueprints: usize,
    standing_progress: &str,
) -> String {
    format!(
        "RUN {}  //  {}  //  {}  //  SCAN {}  //  BP {:02}/{:02}  //  {}  //  RECOV {}  //  EXT {}  //  VALUE ¢{}  //  FIELD LOG {:02}  //  SURV {:02}",
        run_number,
        debrief_intelligence_label(reconnaissance_level),
        site_name,
        scan_profile.short_label(),
        unlocked_blueprints,
        total_blueprints,
        standing_progress,
        recovered_count,
        external_load,
        recovered_value,
        log_count,
        survey_count
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

fn draw_result_manifest(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    if ctx.session.returned.is_empty() {
        draw_text(
            "NO CARGO MADE IT BACK",
            50.0,
            316.0,
            24.0,
            visual_theme::text(),
        );
        draw_text(
            "The hold is clear. The next wreck is waiting.",
            50.0,
            344.0,
            14.0,
            visual_theme::text_dim(),
        );
        if button(
            ctx,
            Rect::new(50.0, 378.0, 220.0, 48.0),
            "BACK TO PORT",
            true,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::GoToPort);
        }
        return;
    }
    draw_text(
        &refinery_forecast_label(ctx.session.economy, &ctx.session.returned, ctx.data),
        50.0,
        278.0,
        11.0,
        visual_theme::amber(),
    );
    draw_text(
        "RETURNED HARDWARE",
        50.0,
        296.0,
        13.0,
        visual_theme::text_dim(),
    );
    let objective_target = ctx
        .session
        .selected_site
        .as_ref()
        .and_then(|site_id| ctx.data.sites.get(site_id))
        .and_then(|site| site.contract_target.as_deref());
    for (index, returned) in ctx.session.returned.iter().enumerate() {
        let y = 312.0 + index as f32 * 56.0;
        let Some(object) = ctx.data.salvage_objects.get(&returned.object_id) else {
            continue;
        };
        draw_result_card(
            ctx,
            returned,
            object,
            Rect::new(44.0, y, 1188.0, 52.0),
            objective_target == Some(object.id.as_str()),
            actions,
        );
    }
    draw_text(
        "Sell is immediate cash. Install preserves capability but charges the yard. Break down feeds Alloy / Electronics.",
        50.0,
        610.0,
        13.0,
        visual_theme::text_dim(),
    );
}

fn draw_result_card(
    ctx: &UiContext<'_>,
    returned: &crate::state::ReturnedItem,
    object: &crate::data::SalvageObjectData,
    rect: Rect,
    is_objective: bool,
    actions: &mut Vec<UiAction>,
) {
    panel(rect, visual_theme::panel());
    if is_objective {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, visual_theme::amber());
    }
    draw_result_silhouette(
        Rect::new(rect.x + 12.0, rect.y + 9.0, 62.0, 34.0),
        &object.visual_silhouette,
    );
    let name = if object.workspace_name.is_empty() {
        object.display_name.as_str()
    } else {
        object.workspace_name.as_str()
    };
    let name_label = if is_objective {
        format!("OBJECTIVE // {}", name.to_uppercase())
    } else {
        name.to_uppercase()
    };
    draw_text(
        clipped(&name_label, 28),
        rect.x + 90.0,
        rect.y + 20.0,
        15.0,
        visual_theme::text(),
    );
    let quote = ctx.session.returned_market_quote(returned, ctx.data);
    let sell_label = result_sell_label(quote);
    let value_label = quote.map_or_else(
        || {
            format!(
                "{}  //  BASE ¢{}  //  ASK UNKNOWN  //  A{}  E{}  //  {}",
                object.category.to_uppercase(),
                object.sale_value,
                object.alloy_yield,
                object.electronics_yield,
                TransferMode::from_target(object).short_label()
            )
        },
        |quote| {
            format!(
                "{}  //  BASE ¢{}  //  ASK ¢{}  //  MKT {} {:+}%  //  A{}  E{}  //  {}",
                object.category.to_uppercase(),
                object.sale_value,
                quote.sale_value,
                quote.band.label(),
                quote.signed_multiplier(),
                object.alloy_yield,
                object.electronics_yield,
                TransferMode::from_target(object).short_label()
            )
        },
    );
    draw_text(
        clipped(&value_label, 78),
        rect.x + 90.0,
        rect.y + 38.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        if object.install_module_id.is_some() {
            "MOUNT CAPABILITY"
        } else {
            "CARGO HOLD"
        },
        rect.x + 570.0,
        rect.y + 20.0,
        10.0,
        if object.install_module_id.is_some() {
            visual_theme::cyan()
        } else {
            visual_theme::safe()
        },
    );
    let transfer_mode = TransferMode::from_target(object);
    let transfer_color = match transfer_mode {
        TransferMode::InternalCargo => visual_theme::cyan(),
        TransferMode::ExternalClamp => visual_theme::amber(),
        TransferMode::Tow => visual_theme::warning(),
    };
    draw_text(
        format!("ROUTE  //  {}", transfer_mode.destination_label()),
        rect.x + 570.0,
        rect.y + 38.0,
        10.0,
        transfer_color,
    );
    let bx = rect.right() - 390.0;
    if button(
        ctx,
        Rect::new(bx, rect.y + 9.0, 116.0, 34.0),
        &sell_label,
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Disposition(object.id.clone(), Disposition::Sell));
    }
    if button(
        ctx,
        Rect::new(bx + 124.0, rect.y + 9.0, 116.0, 34.0),
        "INSTALL",
        object.install_module_id.is_some(),
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Disposition(
            object.id.clone(),
            Disposition::Install,
        ));
    }
    if button(
        ctx,
        Rect::new(bx + 248.0, rect.y + 9.0, 136.0, 34.0),
        "BREAK DOWN",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Disposition(
            object.id.clone(),
            Disposition::BreakDown,
        ));
    }
}

fn result_sell_label(quote: Option<crate::engine::market::MarketQuote>) -> String {
    quote.map_or_else(
        || "SELL".to_owned(),
        |quote| format!("SELL ¢{}", quote.sale_value),
    )
}

fn refinery_forecast_label(
    mut economy: crate::state::EconomyState,
    returned: &[crate::state::ReturnedItem],
    data: &GameData,
) -> String {
    for item in returned {
        if let Some(object) = data.salvage_objects.get(&item.object_id) {
            economy.alloy += object.alloy_yield;
            economy.electronics += object.electronics_yield;
        }
    }
    let alloy = crate::engine::refinery::quote_for(
        crate::engine::refinery::RefineryResource::Alloy,
        economy,
        &data.config.refinery,
    );
    let electronics = crate::engine::refinery::quote_for(
        crate::engine::refinery::RefineryResource::Electronics,
        economy,
        &data.config.refinery,
    );
    let batch_cash = i64::from(alloy.batches_available()) * alloy.payout
        + i64::from(electronics.batches_available()) * electronics.payout;
    format!(
        "REFINERY FORECAST  //  ALLOY {}/{} BATCHES  //  ELEC {}/{} BATCHES  //  CASH ¢{}",
        alloy.batches_available(),
        alloy.batch_size,
        electronics.batches_available(),
        electronics.batch_size,
        batch_cash
    )
}

fn draw_result_silhouette(rect: Rect, kind: &str) {
    let accent = if kind.contains("computer") || kind.contains("sensor") {
        visual_theme::cyan()
    } else if kind.contains("engine") || kind.contains("reactor") {
        visual_theme::warning()
    } else {
        visual_theme::amber()
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::space());
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, accent);
    draw_rectangle(rect.x + 14.0, rect.y + 12.0, rect.w - 28.0, 18.0, accent);
    draw_line(
        rect.x + 22.0,
        rect.y + 9.0,
        rect.x + 22.0,
        rect.y + 34.0,
        2.0,
        visual_theme::structure_light(),
    );
    draw_line(
        rect.right() - 22.0,
        rect.y + 9.0,
        rect.right() - 22.0,
        rect.y + 34.0,
        2.0,
        visual_theme::structure_light(),
    );
}
