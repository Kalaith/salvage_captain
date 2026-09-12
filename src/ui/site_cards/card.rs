//! Departure controls for a single mission card.

use super::*;

struct SiteCardActionState {
    button_width: f32,
    button_gap: f32,
    insurance_quote: Option<crate::engine::InsuranceQuote>,
    can_depart: bool,
    can_depart_insured: bool,
    reconnaissance_quote: Option<crate::engine::ReconnaissanceQuote>,
    can_buy_reconnaissance: bool,
}

pub(super) fn draw_site_card_actions(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
    rect: Rect,
) {
    let can_depart = ctx
        .session
        .can_depart_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let insurance_quote =
        ctx.session
            .insurance_quote_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let can_depart_insured =
        ctx.session
            .can_depart_insured_with_plan(&site.id, ctx.data, ctx.voyage_plan);
    let reconnaissance_quote = ctx.session.reconnaissance_quote(&site.id, ctx.data);
    let can_buy_reconnaissance = ctx.session.can_buy_reconnaissance(&site.id, ctx.data);
    let button_gap = 8.0;
    let button_width = (rect.w - 36.0 - button_gap) / 2.0;
    if button(
        ctx,
        Rect::new(rect.x + 18.0, rect.bottom() - 78.0, button_width, 34.0),
        if can_depart {
            "DEPART"
        } else {
            "NOT ENOUGH FUEL"
        },
        can_depart,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Depart(site.id.clone()));
    }
    if button(
        ctx,
        Rect::new(
            rect.x + 18.0 + button_width + button_gap,
            rect.bottom() - 78.0,
            button_width,
            34.0,
        ),
        "PRIVATE HAUL",
        can_depart,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::DepartPrivate(site.id.clone()));
    }
    draw_secondary_actions(
        ctx,
        actions,
        site,
        rect,
        SiteCardActionState {
            button_width,
            button_gap,
            insurance_quote,
            can_depart,
            can_depart_insured,
            reconnaissance_quote,
            can_buy_reconnaissance,
        },
    );
}

fn draw_secondary_actions(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
    rect: Rect,
    state: SiteCardActionState,
) {
    if button(
        ctx,
        Rect::new(
            rect.x + 18.0,
            rect.bottom() - 42.0,
            state.button_width,
            34.0,
        ),
        &super::insurance_button_label(
            state.insurance_quote,
            state.can_depart,
            state.can_depart_insured,
        ),
        state.can_depart_insured,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::DepartInsured(site.id.clone()));
    }
    if button(
        ctx,
        Rect::new(
            rect.x + 18.0 + state.button_width + state.button_gap,
            rect.bottom() - 42.0,
            state.button_width,
            34.0,
        ),
        &super::reconnaissance_button_label(
            state.reconnaissance_quote,
            state.can_depart,
            state.can_buy_reconnaissance,
        ),
        state.can_buy_reconnaissance,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BuyReconnaissance(site.id.clone()));
    }
}
