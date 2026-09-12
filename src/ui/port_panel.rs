//! Full-width hangar with persistent dock controls and on-demand yard drawers.

use super::*;
use crate::data::ModuleData;
use crate::ui::ship_visual;
use crate::ui::visual_theme;

mod cargo_hold;
mod chrome;
pub(crate) mod refinery;
mod shipyard;
mod world;

pub const HEADER_HEIGHT: f32 = 68.0;
pub use chrome::draw_header;

#[derive(Debug, Clone, Copy)]
struct PortLayout {
    world: Rect,
    ship: Rect,
    cargo_row: Rect,
    shipyard: Rect,
}

pub fn draw_port(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let layout = port_layout();
    world::draw_hangar_world(layout.world, &ctx.data.port_ui);
    ship_visual::draw_ship_with_selection(
        layout.ship,
        ctx.session,
        ctx.data,
        0.0,
        false,
        if ctx.port_tab == PortTab::Equipment {
            ctx.port_selected_module
        } else {
            None
        },
    );
    let mut mount_ctx = *ctx;
    if ctx.port_hold_expanded
        || (ctx.port_tab != PortTab::Hangar && ctx.pointer.position.x >= layout.shipyard.x)
    {
        mount_ctx.interaction_enabled = false;
        mount_ctx.pointer = ctx.pointer.suppressed();
    }
    draw_mount_interactions(&mount_ctx, layout.ship, actions);
    draw_emitter_interaction(&mount_ctx, layout.ship, actions);
    if ctx.port_tab != PortTab::Hangar {
        shipyard::draw_shipyard(ctx, layout.shipyard, actions);
    }
    cargo_hold::draw_cargo_hold(ctx, layout.cargo_row, actions);
    chrome::draw_dock(ctx, actions);
    chrome::draw_footer(ctx);
}

pub(super) fn market_ticker(ctx: &UiContext<'_>) -> String {
    let copy = &ctx.data.port_ui;
    if let Some((object, quote)) = ctx
        .data
        .salvage_objects
        .iter()
        .filter_map(|(_, object)| {
            ctx.session
                .market_quote(&object.id, ctx.data)
                .map(|quote| (object, quote))
        })
        .max_by_key(|(_, quote)| (quote.signed_multiplier(), quote.sale_value))
    {
        return copy
            .market
            .replace("{cycle}", &ctx.session.market_cycle_label())
            .replace("{group}", &object.market_group)
            .replace("{change}", &format!("{:+}", quote.signed_multiplier()));
    }
    ctx.session.market_cycle_label()
}

fn port_layout() -> PortLayout {
    PortLayout {
        world: Rect::new(0.0, HEADER_HEIGHT, 1280.0, 616.0),
        ship: Rect::new(46.0, 252.0, 916.0, 410.0),
        cargo_row: cargo_rect(),
        shipyard: Rect::new(820.0, 80.0, 460.0, 516.0),
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PortTab {
    #[default]
    Hangar,
    Service,
    Equipment,
    Crew,
}

impl PortTab {
    pub const ALL: [Self; 3] = [Self::Service, Self::Equipment, Self::Crew];
}

pub fn tab_rect(tab: PortTab) -> Rect {
    let index = match tab {
        PortTab::Hangar => return close_rect(),
        PortTab::Service => 0,
        PortTab::Equipment => 1,
        PortTab::Crew => 2,
    };
    Rect::new(354.0 + index as f32 * 182.0, 604.0, 168.0, 64.0)
}

pub fn departure_rect() -> Rect {
    Rect::new(932.0, 598.0, 320.0, 76.0)
}
pub fn cargo_rect() -> Rect {
    Rect::new(28.0, 604.0, 272.0, 64.0)
}
pub fn close_rect() -> Rect {
    Rect::new(1150.0, 80.0, 112.0, 44.0)
}
pub fn content_rect() -> Rect {
    Rect::new(838.0, 128.0, 424.0, 460.0)
}
pub fn stock_card_rect(index: usize) -> Rect {
    Rect::new(
        838.0 + (index % 2) as f32 * 216.0,
        372.0 + (index / 2) as f32 * 74.0,
        208.0,
        66.0,
    )
}
pub fn stock_page(current: usize, next: bool, count: usize) -> usize {
    let last = count.div_ceil(4).saturating_sub(1);
    if next {
        current.saturating_add(1).min(last)
    } else {
        current.saturating_sub(1).min(last)
    }
}

pub(super) fn text_at(text: &str, rect: Rect, color: Color) {
    visual_theme::body(text, rect, 20.0, color);
}

fn draw_mount_interactions(ctx: &UiContext<'_>, ship: Rect, actions: &mut Vec<UiAction>) {
    for placement in ctx
        .session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
    {
        let Some(module) = ctx.data.modules.get(&placement.id) else {
            continue;
        };
        let hit = ship_visual::module_mount_rect(ship, module);
        if ctx.pointer.hovering_over(hit) {
            draw_rectangle_lines(
                hit.x,
                hit.y,
                hit.w,
                hit.h,
                2.0,
                visual_theme::with_alpha(visual_theme::amber(), 0.8),
            );
        }
        if ctx.interaction_enabled && ctx.pointer.released_on(hit) {
            actions.push(UiAction::SelectPortModule(placement.id.clone()));
        }
    }
}

fn draw_emitter_interaction(ctx: &UiContext<'_>, ship: Rect, actions: &mut Vec<UiAction>) {
    let engine_installed = ctx
        .session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == "engine_core");
    if !engine_installed {
        return;
    }
    let emitter = ship_visual::tractor_emitter_rect(ship);
    if ctx.pointer.hovering_over(emitter) {
        draw_rectangle_lines(
            emitter.x,
            emitter.y,
            emitter.w,
            emitter.h,
            2.0,
            visual_theme::with_alpha(visual_theme::cyan(), 0.9),
        );
    }
    if ctx.interaction_enabled && ctx.pointer.released_on(emitter) {
        actions.push(UiAction::SelectPortModule("engine_core".to_owned()));
    }
}
