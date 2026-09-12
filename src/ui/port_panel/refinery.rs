//! Shipyard refinery controls that turn recovered materials into credits.

use super::*;

pub(super) fn draw_refinery_console(
    ctx: &UiContext<'_>,
    console: Rect,
    y: f32,
    actions: &mut Vec<UiAction>,
) {
    let rect = Rect::new(console.x + 14.0, y, console.w - 28.0, 58.0);
    panel(
        rect,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.9),
    );
    draw_text(
        "REFINERY  //  STOCK TO CASH",
        rect.x + 12.0,
        rect.y + 16.0,
        10.0,
        visual_theme::text_dim(),
    );
    let alloy_quote = ctx
        .session
        .refinery_quote(crate::engine::refinery::RefineryResource::Alloy, ctx.data);
    let electronics_quote = ctx.session.refinery_quote(
        crate::engine::refinery::RefineryResource::Electronics,
        ctx.data,
    );
    draw_text(
        format!(
            "POTENTIAL ¢{}",
            refinery_total_payout(alloy_quote, electronics_quote)
        ),
        rect.right() - 104.0,
        rect.y + 16.0,
        9.0,
        visual_theme::amber(),
    );
    let gap = 8.0;
    let button_width = (rect.w - 24.0 - gap) * 0.5;
    for (index, (resource, action)) in [
        (
            crate::engine::refinery::RefineryResource::Alloy,
            UiAction::RefineAlloy,
        ),
        (
            crate::engine::refinery::RefineryResource::Electronics,
            UiAction::RefineElectronics,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let quote = match resource {
            crate::engine::refinery::RefineryResource::Alloy => alloy_quote,
            crate::engine::refinery::RefineryResource::Electronics => electronics_quote,
        };
        let button_rect = Rect::new(
            rect.x + 12.0 + index as f32 * (button_width + gap),
            rect.y + 24.0,
            button_width,
            26.0,
        );
        if button(
            ctx,
            button_rect,
            &refinery_button_label(quote),
            quote.can_refine(),
            ButtonTone::Positive,
        ) {
            actions.push(action);
        }
    }
}

pub(super) fn refinery_total_payout(
    alloy: crate::engine::refinery::RefineryQuote,
    electronics: crate::engine::refinery::RefineryQuote,
) -> i64 {
    i64::from(alloy.batches_available()) * alloy.payout
        + i64::from(electronics.batches_available()) * electronics.payout
}

pub(super) fn refinery_button_label(quote: crate::engine::refinery::RefineryQuote) -> String {
    format!(
        "{} {}/{}  ->  ¢{}",
        quote.resource.label(),
        quote.available,
        quote.batch_size,
        quote.payout
    )
}
