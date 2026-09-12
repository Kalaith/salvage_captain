//! Saved equipment arrangements inside the Equipment view.

use super::*;
use crate::state::loadout;
use crate::ui::port_panel::text_at;

pub fn draw_port_loadouts(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    if button(
        ctx,
        Rect::new(frame.x, frame.y, frame.w, 44.0),
        &ctx.data.port_ui.back,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleLoadoutPanel);
    }
    for slot in 0..loadout::SLOT_COUNT {
        draw_slot(
            ctx,
            Rect::new(
                frame.x,
                frame.y + 56.0 + slot as f32 * 132.0,
                frame.w,
                124.0,
            ),
            slot,
            actions,
        );
    }
}

fn draw_slot(ctx: &UiContext<'_>, card: Rect, slot: usize, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    let preset = ctx.session.loadout_slot(slot);
    let active = preset.is_some() && ctx.session.loadout_matches_current(slot);
    panel(card, visual_theme::panel_soft());
    text_at(
        &format!("{} {}", copy.slot, slot + 1),
        Rect::new(card.x + 12.0, card.y + 8.0, 120.0, 28.0),
        visual_theme::cyan(),
    );
    let detail = preset.map_or_else(
        || copy.empty_slot.clone(),
        |preset| {
            copy.stored_slot
                .replace("{count}", &preset.placements.len().to_string())
        },
    );
    text_at(
        &detail,
        Rect::new(card.x + 136.0, card.y + 8.0, card.w - 148.0, 28.0),
        visual_theme::text(),
    );
    let capacity = ctx.session.loadout_capacity(slot, ctx.data).map_or_else(
        || copy.loadout_hint.clone(),
        |(fuel, hull)| {
            copy.capacity
                .replace("{fuel}", &fuel.to_string())
                .replace("{hull}", &hull.to_string())
        },
    );
    text_at(
        &capacity,
        Rect::new(card.x + 12.0, card.y + 40.0, card.w - 24.0, 28.0),
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(card.x + 12.0, card.y + 74.0, 192.0, 42.0),
        &copy.store,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::StoreLoadout(slot));
    }
    if button(
        ctx,
        Rect::new(card.x + 216.0, card.y + 74.0, 196.0, 42.0),
        if active { &copy.active } else { &copy.apply },
        preset.is_some() && !active,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ApplyLoadout(slot));
    }
}
