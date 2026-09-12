//! Safe-port controls for storing and restoring permanent ship arrangements.

use super::*;
use crate::state::loadout::{self, LoadoutPreset};
use crate::ui::visual_theme;

pub fn draw_open_button(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    let stored = (0..loadout::SLOT_COUNT)
        .filter(|slot| ctx.session.loadout_slot(*slot).is_some())
        .count();
    let rect = Rect::new(
        width - 138.0,
        crate::ui::port_panel::HEADER_HEIGHT + 58.0,
        112.0,
        30.0,
    );
    let label = loadout_button_label(stored);
    if button(ctx, rect, &label, true, ButtonTone::Secondary) {
        actions.push(UiAction::ToggleLoadoutPanel);
    }
}

pub fn draw_port_loadouts(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    let height = ctx.viewport_height.max(1.0);
    draw_rectangle(
        0.0,
        crate::ui::port_panel::HEADER_HEIGHT,
        width,
        (height - crate::ui::port_panel::HEADER_HEIGHT).max(0.0),
        visual_theme::with_alpha(visual_theme::space(), 0.48),
    );
    let panel_width = (width * 0.40)
        .clamp(400.0, 520.0)
        .min((width - 36.0).max(260.0));
    let frame = Rect::new(
        (width - panel_width - 18.0).max(18.0),
        104.0,
        panel_width,
        (height - 122.0).max(390.0),
    );
    panel(frame, visual_theme::with_alpha(visual_theme::panel(), 0.98));
    panel_title(frame, "REFIT LOADOUTS // SAFE PORT");
    draw_text(
        "REMEMBER PERMANENT MODULE POSITIONS FOR THE NEXT RUN",
        frame.x + 18.0,
        frame.y + 62.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        capacity_label(ctx),
        frame.x + 18.0,
        frame.y + 76.0,
        10.0,
        visual_theme::amber(),
    );
    let close = Rect::new(frame.right() - 88.0, frame.y + 8.0, 70.0, 26.0);
    if button(ctx, close, "CLOSE", true, ButtonTone::Secondary) {
        actions.push(UiAction::ToggleLoadoutPanel);
    }

    let card_width = frame.w - 36.0;
    for slot in 0..loadout::SLOT_COUNT {
        let card = Rect::new(
            frame.x + 18.0,
            frame.y + 92.0 + slot as f32 * 150.0,
            card_width,
            138.0,
        );
        draw_slot(ctx, card, slot, actions);
    }
}

fn draw_slot(ctx: &UiContext<'_>, card: Rect, slot: usize, actions: &mut Vec<UiAction>) {
    let preset = ctx.session.loadout_slot(slot);
    let stored = preset.is_some();
    draw_rectangle(
        card.x,
        card.y,
        card.w,
        card.h,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.92),
    );
    draw_rectangle_lines(
        card.x,
        card.y,
        card.w,
        card.h,
        1.0,
        visual_theme::with_alpha(
            if stored {
                visual_theme::cyan()
            } else {
                visual_theme::structure_light()
            },
            0.75,
        ),
    );
    draw_text(
        slot_title(slot, preset),
        card.x + 12.0,
        card.y + 22.0,
        15.0,
        visual_theme::text(),
    );
    let detail = preset.map_or_else(
        || "NO ARRANGEMENT SAVED // STORE CURRENT SHIP".to_owned(),
        |preset| {
            format!(
                "{} MODULE(S) // {}",
                preset.placements.len(),
                module_summary(ctx, preset)
            )
        },
    );
    draw_text(
        clipped(&detail, 48),
        card.x + 12.0,
        card.y + 45.0,
        10.0,
        if stored {
            visual_theme::cyan()
        } else {
            visual_theme::text_dim()
        },
    );
    if stored && ctx.session.loadout_matches_current(slot) {
        draw_text(
            "CURRENT SHIP ARRANGEMENT",
            card.x + 12.0,
            card.y + 64.0,
            10.0,
            visual_theme::safe(),
        );
    } else {
        draw_text(
            "PERMANENT MODULES ONLY // CARGO IS NOT REMEMBERED",
            card.x + 12.0,
            card.y + 64.0,
            9.0,
            visual_theme::text_dim(),
        );
    }

    if let Some((fuel_capacity, hull_capacity)) = ctx.session.loadout_capacity(slot, ctx.data) {
        draw_text(
            format!("SLOT CAPACITY  //  FUEL {fuel_capacity}  //  HULL {hull_capacity}"),
            card.x + 12.0,
            card.y + 82.0,
            9.0,
            visual_theme::amber(),
        );
    }

    let store = Rect::new(card.x + 12.0, card.bottom() - 34.0, 86.0, 24.0);
    if button(ctx, store, "STORE", true, ButtonTone::Secondary) {
        actions.push(UiAction::StoreLoadout(slot));
    }
    let apply = Rect::new(card.x + 108.0, card.bottom() - 34.0, 86.0, 24.0);
    let active = stored && ctx.session.loadout_matches_current(slot);
    if button(
        ctx,
        apply,
        if active { "ACTIVE" } else { "APPLY" },
        stored && !active,
        if active {
            ButtonTone::Secondary
        } else {
            ButtonTone::Primary
        },
    ) {
        actions.push(UiAction::ApplyLoadout(slot));
    }
}

fn slot_title(slot: usize, preset: Option<&LoadoutPreset>) -> String {
    format!(
        "SLOT {} // {}",
        slot + 1,
        if preset.is_some() { "STORED" } else { "EMPTY" }
    )
}

fn module_summary(ctx: &UiContext<'_>, preset: &LoadoutPreset) -> String {
    preset
        .placements
        .iter()
        .map(|item| {
            ctx.data
                .modules
                .get(&item.id)
                .map_or_else(|| item.id.clone(), |module| module.display_name.clone())
        })
        .collect::<Vec<_>>()
        .join(" / ")
}

fn capacity_label(ctx: &UiContext<'_>) -> String {
    format!(
        "LIVE CAPACITY  //  FUEL {}/{}  //  HULL {}/{}",
        ctx.session.economy.fuel,
        ctx.session.max_fuel(ctx.data),
        ctx.session.hull,
        ctx.session.max_hull_with_modules(ctx.data),
    )
}

fn loadout_button_label(stored: usize) -> String {
    if stored == 0 {
        "LOADOUTS".to_owned()
    } else {
        format!(
            "LOADOUTS {stored}/{SLOT_COUNT}",
            SLOT_COUNT = loadout::SLOT_COUNT
        )
    }
}
