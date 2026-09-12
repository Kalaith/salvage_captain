//! Inspect equipment before buying, with bounded stock pages.

use super::*;

pub(super) fn draw(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    draw_selected(ctx, Rect::new(frame.x, frame.y, frame.w, 190.0), actions);
    text_at(
        &copy.stock,
        Rect::new(frame.x, 380.0, 220.0, 30.0),
        visual_theme::cyan(),
    );
    if button(
        ctx,
        Rect::new(frame.right() - 148.0, 368.0, 148.0, 44.0),
        &copy.loadouts,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleLoadoutPanel);
    }
    let mut stock: Vec<_> = ctx.data.modules.iter().map(|(_, module)| module).collect();
    stock.sort_by_key(|module| &module.id);
    let page = ctx
        .port_stock_page
        .min(stock.len().div_ceil(4).saturating_sub(1));
    for (index, module) in stock.iter().skip(page * 4).take(4).enumerate() {
        let card = stock_card_rect(index);
        panel(card, visual_theme::panel_soft());
        text_at(
            &module.display_name,
            Rect::new(card.x + 10.0, card.y + 6.0, card.w - 20.0, 28.0),
            visual_theme::text(),
        );
        let label = if installed(ctx, module) {
            copy.installed.clone()
        } else {
            format!("{} CR", module.purchase_cost)
        };
        text_at(
            &label,
            Rect::new(card.x + 10.0, card.y + 34.0, card.w - 20.0, 24.0),
            visual_theme::text_dim(),
        );
        if ctx.port_selected_module == Some(module.id.as_str()) {
            draw_rectangle_lines(card.x, card.y, card.w, card.h, 2.0, visual_theme::amber());
        }
        if ctx.interaction_enabled && ctx.pointer.released_on(card) {
            actions.push(UiAction::SelectPortModule(module.id.clone()));
        }
    }
    for (next, x, label, enabled) in [
        (false, frame.x, &copy.previous, page > 0),
        (
            true,
            frame.right() - 148.0,
            &copy.next,
            (page + 1) * 4 < stock.len(),
        ),
    ] {
        if button(
            ctx,
            Rect::new(x, 572.0, 148.0, 44.0),
            label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::PortStockPage(next));
        }
    }
    text_at(
        &format!("{} / {}", page + 1, stock.len().div_ceil(4)),
        Rect::new(frame.x + 177.0, 585.0, 80.0, 26.0),
        visual_theme::text_dim(),
    );
}

fn installed(ctx: &UiContext<'_>, module: &ModuleData) -> bool {
    ctx.session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == module.id)
}

fn draw_selected(ctx: &UiContext<'_>, card: Rect, actions: &mut Vec<UiAction>) {
    panel(card, visual_theme::panel_soft());
    let copy = &ctx.data.port_ui;
    let Some(module) = ctx
        .port_selected_module
        .and_then(|id| ctx.data.modules.get(id))
    else {
        text_at(
            &copy.mount_hint,
            Rect::new(card.x + 12.0, card.y + 16.0, card.w - 24.0, 80.0),
            visual_theme::text(),
        );
        return;
    };
    visual_theme::body(
        &module.display_name,
        Rect::new(card.x + 12.0, card.y + 10.0, card.w - 24.0, 32.0),
        26.0,
        visual_theme::text(),
    );
    text_at(
        &module.description,
        Rect::new(card.x + 12.0, card.y + 46.0, card.w - 24.0, 58.0),
        visual_theme::text_dim(),
    );
    let is_installed = installed(ctx, module);
    let unlocked = ctx.session.module_is_unlocked(&module.id, ctx.data);
    let fits = ctx
        .session
        .ship_layout
        .first_fit(&module.id, module.footprint, true)
        .is_some();
    let cost = if is_installed {
        module.remove_cost
    } else {
        module.purchase_cost
    };
    let affordable = ctx.session.economy.credits >= cost;
    let status = if is_installed {
        if ctx.session.damaged_modules.contains(&module.id) {
            copy.offline.clone()
        } else {
            copy.installed.clone()
        }
    } else if !unlocked {
        copy.locked
            .replace("{credits}", &module.unlock_credits.to_string())
    } else if !fits {
        copy.no_fit.clone()
    } else if !affordable {
        copy.low_funds.clone()
    } else {
        copy.preview.clone()
    };
    text_at(
        &status,
        Rect::new(card.x + 12.0, card.y + 108.0, card.w - 24.0, 28.0),
        visual_theme::amber(),
    );
    let label = format!(
        "{}  {} CR",
        if is_installed {
            &copy.remove
        } else {
            &copy.buy
        },
        cost
    );
    if button(
        ctx,
        Rect::new(card.x + 12.0, card.bottom() - 50.0, card.w - 24.0, 42.0),
        &label,
        affordable && (is_installed || (unlocked && fits)),
        ButtonTone::Secondary,
    ) {
        actions.push(if is_installed {
            UiAction::RemoveModule(module.id.clone())
        } else {
            UiAction::PurchaseModule(module.id.clone())
        });
    }
}
