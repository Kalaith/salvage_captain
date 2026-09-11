//! Responsive Port composition: the hangar is the world and the shipyard is
//! an action rail over its starboard edge.

use super::*;
use crate::data::{ModuleData, ModuleEffect};
use crate::ui::ship_visual;
use crate::ui::visual_theme;

mod world;

#[cfg(test)]
mod refinery_tests;
#[cfg(test)]
mod tests;

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
    draw_cargo_hold(ctx, layout.world, layout.cargo_row, actions);
    draw_shipyard(ctx, layout.shipyard, actions);
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
        &clipped(&ticker, 96),
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

fn draw_cargo_hold(ctx: &UiContext<'_>, world: Rect, row: Rect, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        visual_theme::with_alpha(visual_theme::panel(), 0.86),
    );
    draw_rectangle(row.x, row.y, 4.0, row.h, visual_theme::amber());
    draw_line(
        row.x,
        row.y - 10.0,
        row.right(),
        row.y - 10.0,
        1.0,
        visual_theme::with_alpha(visual_theme::structure_light(), 0.5),
    );
    draw_text(
        "CARGO HOLD",
        row.x + 16.0,
        row.y + 18.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &format!(
            "{} / {} CELLS",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        row.x + 16.0,
        row.y + 39.0,
        17.0,
        visual_theme::amber(),
    );

    let button_width = 112.0_f32.min((row.w - 28.0).max(80.0));
    let button_rect = Rect::new(
        row.right() - button_width - 12.0,
        row.y + 9.0,
        button_width,
        30.0,
    );
    let meter_x = row.x + 176.0;
    let meter_width = (button_rect.x - meter_x - 18.0).max(110.0);
    draw_text(
        "PHYSICAL PACKING CAPACITY",
        meter_x,
        row.y + 16.0,
        9.0,
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(meter_x, row.y + 25.0, meter_width, 10.0),
        ctx.session.ship_layout.occupied_cells() as f32
            / (ctx.session.ship_layout.width * ctx.session.ship_layout.height) as f32,
        visual_theme::amber(),
        &format!(
            "{} / {}",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
    );
    if button(
        ctx,
        button_rect,
        if ctx.port_hold_expanded {
            "HIDE GRID"
        } else {
            "VIEW GRID"
        },
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePortHold);
    }

    if ctx.port_hold_expanded {
        let popup_width = 300.0_f32.min((world.w - 28.0).max(230.0));
        let popup_height = 168.0;
        let popup = Rect::new(
            row.x + 18.0,
            (row.y - popup_height - 14.0).max(world.y + 18.0),
            popup_width,
            popup_height,
        );
        panel(popup, visual_theme::with_alpha(visual_theme::panel(), 0.98));
        draw_text(
            "CARGO MAP  //  5 × 5",
            popup.x + 16.0,
            popup.y + 24.0,
            12.0,
            visual_theme::cyan(),
        );
        draw_ship_grid(
            ctx,
            Rect::new(popup.x + 16.0, popup.y + 34.0, 142.0, 100.0),
            false,
            actions,
        );
        draw_text(
            "Recovered hardware must fit before return.",
            popup.x + 176.0,
            popup.y + 74.0,
            11.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_shipyard(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        console.x - 8.0,
        console.y,
        8.0,
        console.h,
        visual_theme::with_alpha(visual_theme::structure_dark(), 0.65),
    );
    panel(
        console,
        visual_theme::with_alpha(visual_theme::panel(), 0.98),
    );
    draw_rectangle(
        console.x,
        console.y,
        console.w,
        46.0,
        visual_theme::structure_dark(),
    );
    draw_rectangle(console.x, console.y, 4.0, 46.0, visual_theme::amber());
    draw_text(
        "SHIPYARD",
        console.x + 18.0,
        console.y + 29.0,
        19.0,
        visual_theme::text(),
    );
    let offline = ctx.session.damaged_modules.len();
    let system_label = if offline == 0 {
        "SYSTEMS NOMINAL"
    } else {
        "SYSTEMS NEED SERVICE"
    };
    draw_text(
        system_label,
        console.right() - 142.0,
        console.y + 28.0,
        9.0,
        if offline == 0 {
            visual_theme::safe()
        } else {
            visual_theme::warning()
        },
    );
    let standing = ctx.session.salvage_standing();
    let standing_label = ctx.session.next_standing_threshold().map_or_else(
        || {
            format!(
                "STAND {} // REP {}",
                standing.label(),
                ctx.session.reputation
            )
        },
        |next| {
            format!(
                "STAND {} // REP {}/{}",
                standing.label(),
                ctx.session.reputation,
                next
            )
        },
    );
    draw_text(
        &clipped(&standing_label, 30),
        console.x + 124.0,
        console.y + 28.0,
        9.0,
        visual_theme::cyan(),
    );
    draw_tabs(console);

    let selected = Rect::new(console.x + 14.0, console.y + 102.0, console.w - 28.0, 116.0);
    draw_selected_module(ctx, selected, actions);
    let refinery_y = selected.bottom() + 20.0;
    draw_refinery_console(ctx, console, refinery_y, actions);
    draw_yard_stock(ctx, console, refinery_y + 68.0, actions);
    draw_services(ctx, console, actions);
}

fn draw_refinery_console(ctx: &UiContext<'_>, console: Rect, y: f32, actions: &mut Vec<UiAction>) {
    let rect = Rect::new(console.x + 14.0, y, console.w - 28.0, 58.0);
    panel(
        rect,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.9),
    );
    draw_text(
        "REFINERY  //  STOCK TO CASH",
        rect.x + 12.0,
        rect.y + 16.0,
        10.0,
        visual_theme::text_dim(),
    );
    let gap = 8.0;
    let button_width = (rect.w - 24.0 - gap) * 0.5;
    for (index, (resource, action)) in [
        (
            crate::engine::refinery::RefineryResource::Alloy,
            UiAction::RefineAlloy,
        ),
        (
            crate::engine::refinery::RefineryResource::Electronics,
            UiAction::RefineElectronics,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let quote = ctx.session.refinery_quote(resource, ctx.data);
        let button_rect = Rect::new(
            rect.x + 12.0 + index as f32 * (button_width + gap),
            rect.y + 24.0,
            button_width,
            26.0,
        );
        if button(
            ctx,
            button_rect,
            &refinery_button_label(quote),
            quote.can_refine(),
            ButtonTone::Positive,
        ) {
            actions.push(action);
        }
    }
}

fn refinery_button_label(quote: crate::engine::refinery::RefineryQuote) -> String {
    format!(
        "{} {}/{}  ->  ¢{}",
        quote.resource.label(),
        quote.available,
        quote.batch_size,
        quote.payout
    )
}

fn draw_tabs(console: Rect) {
    draw_rectangle(
        console.x + 14.0,
        console.y + 58.0,
        112.0,
        30.0,
        visual_theme::cyan_dim(),
    );
    draw_text(
        "EQUIPMENT",
        console.x + 27.0,
        console.y + 79.0,
        11.0,
        visual_theme::text(),
    );
    draw_text(
        "SERVICES",
        console.x + 144.0,
        console.y + 79.0,
        10.0,
        visual_theme::text_dim(),
    );
    draw_line(
        console.x + 14.0,
        console.y + 90.0,
        console.right() - 14.0,
        console.y + 90.0,
        1.0,
        visual_theme::structure(),
    );
}

fn draw_selected_module(ctx: &UiContext<'_>, card: Rect, actions: &mut Vec<UiAction>) {
    panel(card, visual_theme::panel_soft());
    let Some(module_id) = ctx.port_selected_module else {
        draw_text(
            "SELECT A SHIP MOUNT",
            card.x + 16.0,
            card.y + 36.0,
            16.0,
            visual_theme::amber(),
        );
        draw_text(
            "Tap the illuminated machinery in the hangar.",
            card.x + 16.0,
            card.y + 64.0,
            11.0,
            visual_theme::text_dim(),
        );
        return;
    };
    let Some(module) = ctx.data.modules.get(module_id) else {
        return;
    };
    let unlocked = ctx.session.module_is_unlocked(module_id, ctx.data);
    let display_name = if module.id == "engine_core" {
        "TRACTOR EMITTER"
    } else {
        module.display_name.as_str()
    };
    draw_text(
        &clipped(&display_name.to_uppercase(), 24),
        card.x + 16.0,
        card.y + 24.0,
        17.0,
        visual_theme::text(),
    );
    let mount_label = if module.id == "engine_core" {
        "MK I INDUSTRIAL  //  BASIC TRACTOR".to_owned()
    } else {
        format!(
            "MOUNT  //  {}",
            module.mount.replace('_', " ").to_uppercase()
        )
    };
    draw_text(
        &clipped(&mount_label, 42),
        card.x + 16.0,
        card.y + 44.0,
        10.0,
        visual_theme::cyan(),
    );
    draw_text(
        &clipped(&module.description, 48),
        card.x + 16.0,
        card.y + 65.0,
        11.0,
        visual_theme::text_dim(),
    );
    let detail = if unlocked {
        module_stock_detail(module)
    } else {
        format!("BLUEPRINT LOCKED // EARN ¢{}", module.unlock_credits)
    };
    draw_text(
        &clipped(&detail, 34),
        card.x + 16.0,
        card.y + 86.0,
        11.0,
        visual_theme::text(),
    );
    let installed = ctx
        .session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == module.id);
    let fits = unlocked
        && (installed
            || ctx
                .session
                .ship_layout
                .first_fit(&module.id, module.footprint, true)
                .is_some());
    if installed
        && button(
            ctx,
            Rect::new(card.right() - 112.0, card.y + 74.0, 96.0, 30.0),
            &format!("REMOVE ¢{}", module.remove_cost),
            true,
            ButtonTone::Warning,
        )
    {
        actions.push(UiAction::RemoveModule(module.id.clone()));
    }
    let status = if !unlocked {
        format!("LOCKED // EARN ¢{}", module.unlock_credits)
    } else if installed {
        "INSTALLED".to_owned()
    } else if fits {
        "PREVIEW ACTIVE".to_owned()
    } else {
        "NO FIT // PREVIEW".to_owned()
    };
    draw_text(
        &status,
        card.x + 16.0,
        card.bottom() - 10.0,
        9.0,
        if !unlocked {
            visual_theme::warning()
        } else if installed {
            visual_theme::safe()
        } else if fits {
            visual_theme::cyan()
        } else {
            visual_theme::warning()
        },
    );
}

fn draw_yard_stock(
    ctx: &UiContext<'_>,
    console: Rect,
    section_y: f32,
    actions: &mut Vec<UiAction>,
) {
    let unlocked_count = ctx.session.unlocked_module_count(ctx.data);
    let total_count = ctx.data.modules.iter().count();
    let next_unlock = ctx.session.next_module_unlock(ctx.data).map_or_else(
        || "ALL BLUEPRINTS ONLINE".to_owned(),
        |module| {
            format!(
                "NEXT {} @ ¢{}",
                module.display_name.to_uppercase(),
                module.unlock_credits
            )
        },
    );
    draw_text(
        format!(
            "YARD STOCK  //  BLUEPRINTS {}/{}",
            unlocked_count, total_count
        ),
        console.x + 14.0,
        section_y,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &clipped(&next_unlock, 24),
        console.right() - 174.0,
        section_y,
        9.0,
        visual_theme::amber(),
    );
    let mut stock: Vec<_> = ctx
        .data
        .modules
        .iter()
        .filter(|(_, module)| {
            !ctx.session
                .ship_layout
                .placements
                .iter()
                .any(|item| item.permanent && item.id == module.id)
        })
        .collect();
    stock.sort_by(|(left, _), (right, _)| left.cmp(right));

    let columns = if console.w >= 360.0 { 2 } else { 1 };
    let gap = 8.0;
    let inner_width = console.w - 28.0;
    let card_width = (inner_width - gap * (columns as f32 - 1.0)) / columns as f32;
    let service_top = console.bottom() - 84.0;
    let stock_top = section_y + 14.0;
    let rows = (stock.len() + columns - 1) / columns;
    if rows == 0 {
        return;
    }
    let available_height = (service_top - stock_top - 8.0).max(48.0);
    let card_height = ((available_height - gap * (rows.saturating_sub(1) as f32)) / rows as f32)
        .clamp(48.0, 70.0);
    for (index, (_, module)) in stock.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let card = Rect::new(
            console.x + 14.0 + column as f32 * (card_width + gap),
            stock_top + row as f32 * (card_height + gap),
            card_width,
            card_height,
        );
        draw_rectangle(
            card.x,
            card.y,
            card.w,
            card.h,
            visual_theme::with_alpha(visual_theme::panel_soft(), 0.68),
        );
        let unlocked = ctx.session.module_is_unlocked(&module.id, ctx.data);
        draw_rectangle(
            card.x,
            card.y,
            3.0,
            card.h,
            if unlocked {
                visual_theme::amber()
            } else {
                visual_theme::structure_light()
            },
        );
        if ctx.port_selected_module == Some(module.id.as_str()) {
            draw_rectangle_lines(card.x, card.y, card.w, card.h, 2.0, visual_theme::cyan());
        }
        let buy_width = 72.0_f32.min(card.w * 0.42);
        let buy_rect = Rect::new(
            card.right() - buy_width - 6.0,
            card.y + 5.0,
            buy_width,
            22.0,
        );
        let name_limit = if card.w < 160.0 { 14 } else { 19 };
        draw_text(
            &clipped(&module.display_name.to_uppercase(), name_limit),
            card.x + 10.0,
            card.y + 16.0,
            10.0,
            visual_theme::text(),
        );
        let detail = if unlocked {
            module_stock_detail(module)
        } else {
            format!("LOCKED // EARN ¢{}", module.unlock_credits)
        };
        draw_text(
            &clipped(&detail, name_limit + 5),
            card.x + 10.0,
            card.y + card.h - 9.0,
            9.0,
            visual_theme::text_dim(),
        );
        let fits = unlocked
            && ctx
                .session
                .ship_layout
                .first_fit(&module.id, module.footprint, true)
                .is_some();
        let affordable = ctx.session.economy.credits >= module.purchase_cost;
        let enabled = unlocked && fits && affordable;
        let label = if !unlocked {
            "LOCKED".to_owned()
        } else if !fits {
            "NO FIT".to_owned()
        } else if !affordable {
            "LOW CR".to_owned()
        } else {
            format!("BUY ¢{}", module.purchase_cost)
        };
        if button(ctx, buy_rect, &label, enabled, ButtonTone::Positive) {
            actions.push(UiAction::PurchaseModule(module.id.clone()));
        }
        let preview_zone = Rect::new(
            card.x,
            card.y,
            (buy_rect.x - card.x - 4.0).max(40.0),
            card.h,
        );
        if ctx.interaction_enabled && ctx.pointer.released_on(preview_zone) {
            actions.push(UiAction::SelectPortModule(module.id.clone()));
        }
    }
}

fn draw_services(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    let inner_x = console.x + 14.0;
    let inner_width = console.w - 28.0;
    let gap = 8.0;
    let small_width = (inner_width - gap) * 0.5;
    let y = console.bottom() - 78.0;
    if button(
        ctx,
        Rect::new(inner_x, y, small_width, 30.0),
        "REFUEL",
        true,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Refuel);
    }
    if button(
        ctx,
        Rect::new(inner_x + small_width + gap, y, small_width, 30.0),
        "REPAIR",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::Repair);
    }
    if button(
        ctx,
        Rect::new(inner_x, y + 38.0, inner_width, 32.0),
        "BROWSE WRECKS",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::GoToSites);
    }
}

fn module_stock_detail(module: &ModuleData) -> String {
    if let Some(capability) = &module.capability {
        let label = format!("CAP {}", capability.replace('_', " ").to_uppercase());
        return if module.drone_support > 0 {
            format!("{label} // DRONES +{}", module.drone_support)
        } else {
            label
        };
    }
    if module.external_capacity > 0 {
        return format!("CLAMP +{}", module.external_capacity);
    }
    match &module.effect {
        ModuleEffect::CargoSpace => "CARGO SPACE".to_owned(),
        ModuleEffect::FuelCapacity(value) => format!("FUEL CAP +{value}"),
        ModuleEffect::FuelEfficiency(value) => format!("FUEL EFF +{value}"),
        ModuleEffect::Hull(value) => format!("HULL +{value}"),
        ModuleEffect::Power(value) => format!("POWER +{value}"),
        ModuleEffect::Scanning(value) => format!("SCAN +{value}"),
        ModuleEffect::Shielding(value) => format!("SHIELD +{value}"),
    }
}
