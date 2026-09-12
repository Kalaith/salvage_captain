//! Responsive Port composition: the hangar is the world and the shipyard is
//! an action rail over its starboard edge.

use super::*;
use crate::data::{ModuleData, ModuleEffect};
use crate::ui::ship_visual;
use crate::ui::visual_theme;

mod cargo_hold;
mod maintenance;
mod refinery;
mod shipyard;
mod world;

pub(super) use maintenance::{
    maintenance_completion_label, maintenance_status_label, repair_button_label,
};

pub const HEADER_HEIGHT: f32 = 56.0;

#[derive(Debug, Clone, Copy)]
struct PortLayout {
    world: Rect,
    ship: Rect,
    cargo_row: Rect,
    shipyard: Rect,
}

pub fn draw_port(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let layout = port_layout(ctx);
    world::draw_hangar_world(layout.world, layout.cargo_row);
    draw_market_ticker(ctx, layout.world);
    ship_visual::draw_ship_with_selection(
        layout.ship,
        ctx.session,
        ctx.data,
        0.0,
        false,
        ctx.port_selected_module,
    );
    draw_mount_interactions(ctx, layout.ship, actions);
    draw_emitter_interaction(ctx, layout.ship, actions);
    cargo_hold::draw_cargo_hold(ctx, layout.world, layout.cargo_row, actions);
    shipyard::draw_shipyard(ctx, layout.shipyard, actions);
}

fn draw_market_ticker(ctx: &UiContext<'_>, world: Rect) {
    let Some((object, quote)) = ctx
        .data
        .salvage_objects
        .iter()
        .filter_map(|(_, object)| {
            ctx.session
                .market_quote(&object.id, ctx.data)
                .map(|quote| (object, quote))
        })
        .max_by_key(|(_, quote)| (quote.signed_multiplier(), quote.sale_value))
    else {
        return;
    };
    let ticker = format!(
        "{}  //  BUYERS FAVOR {} {} {:+}%  //  PRICES LOCK AT RETURN",
        ctx.session.market_cycle_label(),
        object.market_group.to_uppercase(),
        quote.band.label(),
        quote.signed_multiplier()
    );
    let panel_rect = Rect::new(world.x + 24.0, world.y + 42.0, world.w - 48.0, 22.0);
    panel(
        panel_rect,
        visual_theme::with_alpha(visual_theme::panel(), 0.84),
    );
    draw_text(
        clipped(&ticker, 96),
        panel_rect.x + 12.0,
        panel_rect.y + 15.0,
        9.0,
        visual_theme::cyan(),
    );
}

fn port_layout(ctx: &UiContext<'_>) -> PortLayout {
    let width = ctx.viewport_width.max(1.0);
    let height = ctx.viewport_height.max(1.0);
    let sidebar_width = (width * 0.34).clamp(320.0, 620.0).min(width * 0.48);
    let world_width = width - sidebar_width;
    let world = Rect::new(0.0, HEADER_HEIGHT, world_width, height - HEADER_HEIGHT);
    let shipyard = Rect::new(
        world_width,
        HEADER_HEIGHT,
        sidebar_width,
        height - HEADER_HEIGHT,
    );
    let cargo_row = Rect::new(
        world.x + 28.0,
        world.bottom() - 66.0,
        (world.w - 56.0).max(220.0),
        48.0,
    );
    let ship_zone = Rect::new(
        world.x + 22.0,
        world.y + 62.0,
        (world.w - 44.0).max(300.0),
        (cargo_row.y - world.y - 78.0).max(260.0),
    );
    let ship_width = (world.w * 0.86)
        .clamp(420.0, 1_060.0)
        .min((world.w - 42.0).max(300.0))
        .max(300.0);
    let ship_height = (ship_width * 0.49).clamp(280.0, 520.0);
    let ship_x = ship_zone.x + (ship_zone.w - ship_width).max(0.0) * 0.46;
    let ship_y = ship_zone.y + (ship_zone.h - ship_height).max(0.0) * 0.42;
    let ship = Rect::new(ship_x, ship_y, ship_width, ship_height);
    PortLayout {
        world,
        ship,
        cargo_row,
        shipyard,
    }
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
