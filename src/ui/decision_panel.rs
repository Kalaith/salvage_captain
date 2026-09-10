//! Results cards expose the three post-expedition dispositions.

use super::*;

pub fn draw_results(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel_title(Rect::new(24.0, 112.0, 1232.0, 494.0), state::results::TITLE);
    if let Some(risk) = &ctx.session.last_risk {
        let accent = match risk.outcome {
            RiskOutcome::OrdinaryReturn => dark::POSITIVE,
            _ => dark::WARNING,
        };
        draw_text(
            format!("{}  -  {}", risk_label(risk.outcome), risk.explanation),
            54.0,
            164.0,
            17.0,
            accent,
        );
        draw_text(
            "Resolve every returned object before leaving the results screen.",
            54.0,
            190.0,
            15.0,
            dark::TEXT_DIM,
        );
    }
    if ctx.session.returned.is_empty() {
        draw_text(
            "No cargo made it back. The grid is ready for another run.",
            54.0,
            270.0,
            23.0,
            dark::TEXT_BRIGHT,
        );
        if button(
            ctx,
            Rect::new(54.0, 314.0, 220.0, 48.0),
            "BACK TO PORT",
            true,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::GoToPort);
        }
        return;
    }
    for (index, returned) in ctx.session.returned.iter().enumerate() {
        let y = 222.0 + index as f32 * 68.0;
        let Some(object) = ctx.data.salvage_objects.get(&returned.object_id) else {
            continue;
        };
        draw_result_card(ctx, object, Rect::new(54.0, y, 1172.0, 56.0), actions);
    }
    draw_text(
        "Sell is immediate cash. Install preserves capability but charges the yard. Break down feeds Alloy / Electronics.",
        54.0,
        574.0,
        15.0,
        dark::TEXT_DIM,
    );
}

fn draw_result_card(
    ctx: &UiContext<'_>,
    object: &crate::data::SalvageObjectData,
    rect: Rect,
    actions: &mut Vec<UiAction>,
) {
    panel(rect, Color::new(0.10, 0.14, 0.18, 1.0));
    draw_text(
        &object.display_name,
        rect.x + 14.0,
        rect.y + 23.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        format!(
            "{}  {}x{}  ¢{}  A{} E{}",
            object.category,
            object.footprint.width,
            object.footprint.height,
            object.sale_value,
            object.alloy_yield,
            object.electronics_yield
        ),
        rect.x + 14.0,
        rect.y + 44.0,
        13.0,
        dark::TEXT_DIM,
    );
    let bx = rect.right() - 390.0;
    if button(
        ctx,
        Rect::new(bx, rect.y + 8.0, 116.0, 40.0),
        "SELL",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Disposition(object.id.clone(), Disposition::Sell));
    }
    if button(
        ctx,
        Rect::new(bx + 124.0, rect.y + 8.0, 116.0, 40.0),
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
        Rect::new(bx + 248.0, rect.y + 8.0, 136.0, 40.0),
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
