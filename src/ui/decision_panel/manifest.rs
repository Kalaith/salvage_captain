//! Returned cargo manifest and debrief card rendering.

use super::*;

pub(super) fn draw_result_manifest(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
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
        refinery_forecast_label(ctx.session.economy, &ctx.session.returned, ctx.data),
        50.0,
        314.0,
        11.0,
        visual_theme::amber(),
    );
    draw_text(
        "RETURNED HARDWARE",
        50.0,
        332.0,
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
        let y = 348.0 + index as f32 * 56.0;
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
        626.0,
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
