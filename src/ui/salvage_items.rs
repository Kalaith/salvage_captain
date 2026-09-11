//! Recovery manifest and hold-packing deck.

use super::*;
use crate::engine::exposure_label;
use crate::state::workspace::TransferMode;
use crate::ui::visual_theme;

#[cfg(test)]
mod tests;

pub fn draw_packing(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_hold_panel(ctx, actions);
    draw_manifest(ctx, actions);
}

fn draw_hold_panel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let hold = Rect::new(24.0, 84.0, 450.0, 636.0);
    panel(hold, visual_theme::panel_soft());
    draw_rectangle(hold.x, hold.y, hold.w, 42.0, visual_theme::structure_dark());
    draw_text(
        "RETURN HOLD",
        hold.x + 18.0,
        hold.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "PACK BEFORE YOU BURN FUEL",
        hold.right() - 178.0,
        hold.y + 27.0,
        10.0,
        visual_theme::amber(),
    );
    draw_text(
        "TOUCH THE GRID TO PLACE RECOVERED HARDWARE",
        hold.x + 20.0,
        hold.y + 68.0,
        11.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!(
            "EXTERNAL CLAMPS  {}/{}",
            ctx.session.external_cargo_count(ctx.data, None),
            ctx.session.external_capacity(ctx.data)
        ),
        hold.x + 20.0,
        hold.y + 88.0,
        11.0,
        visual_theme::amber(),
    );
    let (cargo_count, clamp_count, tow_count) = transfer_counts(ctx);
    draw_text(
        format!("CARGO {:02}", cargo_count),
        hold.x + 20.0,
        hold.y + 106.0,
        10.0,
        visual_theme::cyan(),
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
    draw_ship_grid(ctx, Rect::new(52.0, 198.0, 394.0, 270.0), true, actions);
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
    let risk_preview = ctx.session.expedition_risk_preview(ctx.data);
    let risk = risk_preview
        .as_ref()
        .map_or(0, |preview| preview.danger_score);
    let external_load = ctx.session.external_cargo_count(ctx.data, None);
    let objective = ctx.session.expedition.as_ref().and_then(|expedition| {
        ctx.session
            .contract_objective_status(&expedition.site_id, ctx.data)
    });
    draw_text(
        &site_label.to_uppercase(),
        hold.x + 20.0,
        hold.y + 418.0,
        15.0,
        visual_theme::text(),
    );
    if let Some(objective) = objective {
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
                | crate::state::contracts::ContractObjectiveState::Recovered => {
                    visual_theme::amber()
                }
            },
        );
    }
    draw_text(
        format!(
            "RISK PREVIEW  {:02}%  //  {}  //  EXT STRAIN +{}",
            risk,
            exposure_label(risk),
            external_load
        ),
        hold.x + 20.0,
        hold.y + 454.0,
        12.0,
        danger_color(risk),
    );
    draw_text(
        &packing_coverage_label(ctx, risk_preview.as_ref()),
        hold.x + 20.0,
        hold.y + 474.0,
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
    draw_text(
        &return_burn_label(
            ctx.session.economy.fuel,
            ctx.data.config.safe_return_buffer.max(0),
        ),
        hold.x + 20.0,
        hold.y + 496.0,
        12.0,
        visual_theme::text_dim(),
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

fn draw_manifest(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let manifest = Rect::new(496.0, 84.0, 760.0, 636.0);
    panel(manifest, visual_theme::panel());
    draw_rectangle(
        manifest.x,
        manifest.y,
        manifest.w,
        42.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "RECOVERY MANIFEST",
        manifest.x + 18.0,
        manifest.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "CARGO  //  CLAMP  //  TOW",
        manifest.right() - 190.0,
        manifest.y + 27.0,
        11.0,
        visual_theme::cyan(),
    );
    draw_text(
        "Every item has a transfer method, a footprint, and a decision.",
        manifest.x + 22.0,
        manifest.y + 68.0,
        14.0,
        visual_theme::text_dim(),
    );
    if let Some(expedition) = &ctx.session.expedition {
        let objective_target = ctx
            .data
            .sites
            .get(&expedition.site_id)
            .and_then(|site| site.contract_target.as_deref());
        for (index, cargo) in expedition.cargo.iter().enumerate() {
            let y = manifest.y + 92.0 + index as f32 * 58.0;
            draw_cargo_card(
                ctx,
                cargo.object_id.as_str(),
                cargo.status,
                Rect::new(manifest.x + 20.0, y, manifest.w - 40.0, 52.0),
                objective_target == Some(cargo.object_id.as_str()),
                actions,
            );
        }
    }
    let pending = ctx.session.pending_count();
    let action_y = manifest.bottom() - 76.0;
    if button(
        ctx,
        Rect::new(manifest.x + 20.0, action_y, 180.0, 40.0),
        "LEAVE ALL",
        pending > 0,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::LeaveAll);
    }
    if button(
        ctx,
        Rect::new(manifest.x + 212.0, action_y, 250.0, 40.0),
        "RETURN WITH HAUL",
        pending == 0,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::FinishPacking);
    }
    draw_text(
        format!("{} object(s) need a decision", pending),
        manifest.x + 482.0,
        action_y + 25.0,
        13.0,
        if pending == 0 {
            visual_theme::safe()
        } else {
            visual_theme::text_dim()
        },
    );
}

fn draw_cargo_card(
    ctx: &UiContext<'_>,
    object_id: &str,
    status: CargoStatus,
    rect: Rect,
    is_objective: bool,
    actions: &mut Vec<UiAction>,
) {
    let Some(object) = ctx.data.salvage_objects.get(object_id) else {
        return;
    };
    let accent = visual_theme::site_accent(&object.visual_silhouette);
    panel(
        rect,
        if status == CargoStatus::Packed {
            visual_theme::with_alpha(visual_theme::safe(), 0.14)
        } else {
            visual_theme::panel_soft()
        },
    );
    if is_objective {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, visual_theme::amber());
    }
    draw_cargo_silhouette(
        Rect::new(rect.x + 12.0, rect.y + 9.0, 58.0, 34.0),
        &object.visual_silhouette,
        accent,
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
        rect.x + 84.0,
        rect.y + 20.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        clipped(
            &packing_value_label(object, ctx.session.market_quote(object_id, ctx.data)),
            65,
        ),
        rect.x + 84.0,
        rect.y + 38.0,
        10.0,
        visual_theme::text_dim(),
    );
    let status_text = match status {
        CargoStatus::Pending => "PENDING",
        CargoStatus::Packed => "PACKED",
        CargoStatus::LeftBehind => "LEFT",
        CargoStatus::Discarded => "DISCARDED",
        CargoStatus::Lost => "LOST",
    };
    draw_text(
        status_text,
        rect.right() - 224.0,
        rect.y + 16.0,
        10.0,
        if status == CargoStatus::Packed {
            visual_theme::safe()
        } else {
            visual_theme::amber()
        },
    );
    let bx = rect.right() - 224.0;
    let active = matches!(status, CargoStatus::Pending | CargoStatus::Packed);
    let clamp_full = TransferMode::from_target(object).uses_external_rig()
        && ctx.session.external_cargo_count(ctx.data, Some(object_id))
            >= ctx.session.external_capacity(ctx.data);
    if clamp_full && status == CargoStatus::Pending {
        draw_text(
            "CLAMP FULL",
            rect.right() - 224.0,
            rect.y + 16.0,
            10.0,
            visual_theme::warning(),
        );
    }
    for (offset, label, tone, action) in [
        (
            0.0,
            "PLACE",
            ButtonTone::Primary,
            UiAction::AutoPlace(object_id.to_owned()),
        ),
        (
            56.0,
            "ROTATE",
            ButtonTone::Secondary,
            UiAction::Rotate(object_id.to_owned()),
        ),
        (
            112.0,
            "LEAVE",
            ButtonTone::Warning,
            UiAction::Leave(object_id.to_owned()),
        ),
        (
            168.0,
            "DROP",
            ButtonTone::Secondary,
            UiAction::Discard(object_id.to_owned()),
        ),
    ] {
        let enabled = match label {
            "ROTATE" => active && object.rotatable,
            "PLACE" => active && !clamp_full,
            _ => active,
        };
        if button(
            ctx,
            Rect::new(bx + offset, rect.y + 24.0, 52.0, 24.0),
            label,
            enabled,
            tone,
        ) {
            actions.push(action);
        }
    }
    let drag_zone = Rect::new(rect.x, rect.y, 340.0, rect.h);
    if status == CargoStatus::Pending
        && ctx.dragged_item.is_none()
        && ctx.interaction_enabled
        && ctx.pointer_started
        && ctx.pointer.pressing(drag_zone)
    {
        actions.push(UiAction::BeginDrag(object_id.to_owned()));
    }
}

fn packing_market_label(quote: Option<crate::engine::market::MarketQuote>) -> String {
    quote.map_or_else(
        || "MKT UNKNOWN".to_owned(),
        |quote| {
            format!(
                "MKT {} {:+}%",
                quote.band.label(),
                quote.signed_multiplier()
            )
        },
    )
}

fn packing_value_label(
    object: &crate::data::SalvageObjectData,
    quote: Option<crate::engine::market::MarketQuote>,
) -> String {
    format!(
        "BASE ¢{} -> ASK ¢{}  //  A{} E{}  //  MKT {}",
        object.sale_value,
        quote.map_or(object.sale_value, |quote| quote.sale_value),
        object.alloy_yield,
        object.electronics_yield,
        packing_market_label(quote)
    )
}

fn packing_coverage_label(ctx: &UiContext<'_>, risk: Option<&crate::engine::RiskResult>) -> String {
    let Some(expedition) = &ctx.session.expedition else {
        return "COVER UNKNOWN".to_owned();
    };
    if !expedition.insured {
        return "COVER NONE  //  CLAIMS SELF-FUNDED".to_owned();
    }
    let premium = ctx
        .session
        .insurance_quote_with_plan(&expedition.site_id, ctx.data, expedition.voyage_plan)
        .map_or(0, |quote| quote.premium);
    let Some(risk) = risk else {
        return format!("COVER ¢{premium}  //  CLAIM ESTIMATE PENDING");
    };
    let impacted_value = match risk.outcome {
        crate::engine::RiskOutcome::LostSalvage | crate::engine::RiskOutcome::ForcedAbandon => {
            let packed = expedition
                .cargo
                .iter()
                .filter(|item| item.status == CargoStatus::Packed)
                .filter_map(|item| {
                    ctx.data
                        .salvage_objects
                        .get(&item.object_id)
                        .map(|object| (item, object))
                })
                .collect::<Vec<_>>();
            let selected = if risk.outcome == crate::engine::RiskOutcome::LostSalvage {
                packed.iter().max_by_key(|(_, object)| object.sale_value)
            } else {
                packed.iter().min_by_key(|(_, object)| object.sale_value)
            };
            selected
                .map(|(_, object)| {
                    crate::engine::market::quote_for(
                        object,
                        ctx.session.market_cycle,
                        &ctx.data.config.market,
                    )
                    .sale_value
                })
                .unwrap_or(0)
        }
        _ => 0,
    };
    let emergency_bill = if risk.outcome == crate::engine::RiskOutcome::EmergencyRepair {
        i64::from((ctx.data.config.repair_price_per_hull * 3).max(60))
    } else {
        0
    };
    let claim = crate::engine::claim_payout(
        risk.outcome,
        impacted_value,
        emergency_bill,
        &ctx.data.config.insurance,
    );
    packing_claim_label(premium, claim)
}

fn packing_claim_label(premium: i64, claim: i64) -> String {
    format!("COVER ¢{premium}  //  CLAIM EST ¢{claim}")
}

fn return_burn_label(fuel: i32, return_fuel: i32) -> String {
    format!(
        "RETURN BURN {return_fuel} FUEL  //  {remaining} REMAIN AFTER DOCKING",
        remaining = (fuel - return_fuel).max(0)
    )
}

fn draw_cargo_silhouette(rect: Rect, kind: &str, accent: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::space());
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, accent);
    if kind.contains("battery") || kind.contains("power") {
        draw_rectangle(rect.x + 18.0, rect.y + 10.0, 34.0, 22.0, accent);
        draw_line(
            rect.x + 52.0,
            rect.y + 21.0,
            rect.x + 61.0,
            rect.y + 21.0,
            3.0,
            accent,
        );
    } else if kind.contains("computer") || kind.contains("sensor") {
        draw_rectangle(
            rect.x + 14.0,
            rect.y + 9.0,
            42.0,
            25.0,
            visual_theme::cyan_dim(),
        );
        draw_line(
            rect.x + 22.0,
            rect.y + 18.0,
            rect.x + 48.0,
            rect.y + 18.0,
            2.0,
            accent,
        );
    } else {
        draw_rectangle(rect.x + 12.0, rect.y + 14.0, 48.0, 16.0, accent);
        draw_line(
            rect.x + 20.0,
            rect.y + 10.0,
            rect.x + 18.0,
            rect.y + 34.0,
            2.0,
            visual_theme::structure_light(),
        );
        draw_line(
            rect.x + 48.0,
            rect.y + 10.0,
            rect.x + 50.0,
            rect.y + 34.0,
            2.0,
            visual_theme::structure_light(),
        );
    }
}
