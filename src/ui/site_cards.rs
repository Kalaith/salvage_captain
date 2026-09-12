//! Illustrated wreck choices with one shared preparation inspector.

use super::*;
mod card;
mod details;
mod preparation;
mod selection;
pub use selection::{SelectionAction, WreckSelection};

pub fn draw_site_selection(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.selection_ui;
    visual_theme::body(
        if ctx.message == crate::game::prompts::state_prompt(GameState::SiteSelection) {
            &copy.instruction
        } else {
            ctx.message
        },
        Rect::new(28.0, 86.0, 1224.0, 36.0),
        21.0,
        visual_theme::text_dim(),
    );
    let selected = ctx.wreck_selection.selected(ctx.data);
    for (index, site) in ctx.data.ordered_sites().into_iter().enumerate() {
        card::draw(
            ctx,
            actions,
            site,
            card_rect(index),
            selected.is_some_and(|s| s.id == site.id),
        );
    }
    if let Some(site) = selected {
        visual_theme::surface(Rect::new(24.0, 416.0, 1232.0, 280.0));
        if ctx.wreck_selection.details_open {
            details::draw(ctx, site);
        } else {
            preparation::draw(ctx, actions, site);
        }
        preparation::draw_departure(ctx, actions, site);
        let label = if ctx.wreck_selection.details_open {
            &copy.back
        } else {
            &copy.details
        };
        if button(
            ctx,
            Rect::new(696.0, 428.0, 220.0, 42.0),
            label,
            true,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::WreckSelection(SelectionAction::ToggleDetails));
        }
    }
}

pub fn card_rect(index: usize) -> Rect {
    Rect::new(24.0 + index as f32 * 416.0, 126.0, 400.0, 272.0)
}

pub fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.selection_ui;
    visual_theme::surface(Rect::new(0.0, 0.0, 1280.0, 76.0));
    visual_theme::body(
        &copy.title,
        Rect::new(28.0, 20.0, 365.0, 40.0),
        30.0,
        visual_theme::text(),
    );
    for (index, label) in [
        format!("{} {}", copy.credits, ctx.session.economy.credits),
        format!("{} {}", ctx.data.transit_ui.fuel, ctx.session.economy.fuel),
        format!("{} {}", copy.hull, ctx.session.hull),
        format!(
            "{} {}/{}",
            copy.cargo,
            ctx.session.internal_cargo_count(ctx.data, None),
            ctx.session.internal_cargo_capacity()
        ),
    ]
    .iter()
    .enumerate()
    {
        visual_theme::body(
            label,
            Rect::new(420.0 + index as f32 * 142.0, 26.0, 138.0, 32.0),
            22.0,
            visual_theme::text(),
        );
    }
    for (rect, label, action) in [
        (
            Rect::new(1000.0, 16.0, 108.0, 46.0),
            &copy.port,
            UiAction::GoToPort,
        ),
        (
            Rect::new(1120.0, 16.0, 136.0, 46.0),
            &copy.pause,
            UiAction::TogglePause,
        ),
    ] {
        if button(ctx, rect, label, true, ButtonTone::Secondary) {
            actions.push(action);
        }
    }
}

fn text(value: &str, x: f32, y: f32, width: f32, height: f32, size: f32, color: Color) {
    visual_theme::body(value, Rect::new(x, y, width, height), size, color);
}

fn fuel_required(ctx: &UiContext<'_>, site: &crate::data::SiteData) -> i32 {
    ctx.session
        .departure_fuel_required_with_plan(&site.id, ctx.data, ctx.voyage_plan)
        .unwrap_or(site.fuel_cost)
}
