//! Illustrated wreck choices with one shared preparation inspector.

use super::*;
mod board;
mod card;
pub use board::{BoardAction, BOARD_PAGE_SIZE};
mod details;
mod preparation;
mod selection;
pub use selection::{SelectionAction, WreckSelection};

pub fn draw_site_selection(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.selection_ui;
    let blocked = ctx
        .session
        .discovery_block_reason(crate::data::discovery::LeadKind::Local, ctx.data);
    visual_theme::body(
        if ctx.message == crate::game::prompts::state_prompt(GameState::SiteSelection) {
            blocked
                .as_deref()
                .unwrap_or(&ctx.data.discovery.copy.readiness_hint)
        } else {
            ctx.message
        },
        Rect::new(28.0, 86.0, 1224.0, 36.0),
        21.0,
        visual_theme::text_dim(),
    );
    board::draw_toolbar(ctx, actions);
    let selected = ctx.wreck_selection.selected_on_board(ctx.session, ctx.data);
    if selected.is_none() {
        board::draw_empty(ctx);
        return;
    }
    for (index, site) in ctx
        .wreck_selection
        .visible_sites(ctx.session, ctx.data)
        .into_iter()
        .enumerate()
    {
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
        if ctx.wreck_selection.details_open || ctx.wreck_selection.archived {
            details::draw(ctx, site);
        } else {
            preparation::draw(ctx, actions, site);
        }
        if ctx.wreck_selection.archived {
            text(
                &ctx.data.discovery.copy.depleted_departure,
                948.0,
                486.0,
                288.0,
                90.0,
                27.0,
                visual_theme::text_dim(),
            );
            return;
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
    Rect::new(24.0 + index as f32 * 416.0, 184.0, 400.0, 214.0)
}

pub fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    crate::ui::header::draw_standard_header(
        ctx,
        actions,
        "SITES // SC-07",
        "ROUTE PLANNING",
        crate::ui::header::HeaderNavigation {
            label: "PORT",
            action: UiAction::GoToPort,
            enabled: true,
            pause_enabled: true,
        },
    );
}

fn text(value: &str, x: f32, y: f32, width: f32, height: f32, size: f32, color: Color) {
    visual_theme::body(value, Rect::new(x, y, width, height), size, color);
}

fn fuel_required(ctx: &UiContext<'_>, site: &crate::data::SiteData) -> i32 {
    ctx.session
        .departure_fuel_required_with_plan(&site.id, ctx.data, ctx.voyage_plan)
        .unwrap_or(site.fuel_cost)
}
