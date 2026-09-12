//! Material conversion quotes beside service preparation.

use super::*;
use crate::engine::refinery::RefineryResource;

pub(crate) fn draw_refinery_console(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    text_at(
        &ctx.data.port_ui.refinery,
        Rect::new(frame.x, frame.y, frame.w, 26.0),
        visual_theme::cyan(),
    );
    for (index, (resource, action)) in [
        (RefineryResource::Alloy, UiAction::RefineAlloy),
        (RefineryResource::Electronics, UiAction::RefineElectronics),
    ]
    .into_iter()
    .enumerate()
    {
        let quote = ctx.session.refinery_quote(resource, ctx.data);
        let label = format!(
            "{} {}/{} > {} CR",
            resource.label(),
            quote.available,
            quote.batch_size,
            quote.payout
        );
        if button(
            ctx,
            Rect::new(frame.x + index as f32 * 216.0, frame.y + 30.0, 208.0, 44.0),
            &label,
            quote.can_refine(),
            ButtonTone::Secondary,
        ) {
            actions.push(action);
        }
    }
}
