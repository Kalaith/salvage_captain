//! Port composition: the ship is the hero and the yard is a focused action rail.

use super::*;
use crate::data::{ModuleData, ModuleEffect};
use crate::ui::ship_visual;
use crate::ui::visual_theme;

const BAY: Rect = Rect::new(24.0, 100.0, 798.0, 550.0);
const CONSOLE: Rect = Rect::new(846.0, 100.0, 410.0, 550.0);

pub fn draw_port(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_hangar_bay(ctx, actions);
    draw_shipyard(ctx, actions);
}

fn draw_hangar_bay(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel(
        BAY,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.86),
    );
    draw_rectangle(BAY.x, BAY.y, BAY.w, 48.0, visual_theme::structure_dark());
    draw_text(
        "HANGAR BAY  //  SC-07",
        BAY.x + 18.0,
        BAY.y + 31.0,
        20.0,
        visual_theme::text(),
    );
    draw_text(
        "PATCHED WORKBOAT",
        BAY.right() - 172.0,
        BAY.y + 30.0,
        12.0,
        visual_theme::text_dim(),
    );

    draw_hangar_structure(BAY);
    let ship = Rect::new(BAY.x + 116.0, BAY.y + 104.0, 556.0, 272.0);
    ship_visual::draw_ship_with_selection(
        ship,
        ctx.session,
        ctx.data,
        0.0,
        false,
        ctx.port_selected_module,
    );
    draw_mount_interactions(ctx, ship, actions);
    draw_emitter_interaction(ctx, ship, actions);
    draw_cargo_hold(ctx, actions);
}

fn draw_hangar_structure(bay: Rect) {
    let top = bay.y + 48.0;
    let floor = bay.bottom() - 142.0;
    let structure = visual_theme::with_alpha(visual_theme::structure_light(), 0.18);
    let deep = visual_theme::with_alpha(visual_theme::structure_dark(), 0.9);
    draw_rectangle(bay.x + 22.0, top + 26.0, bay.w - 44.0, 10.0, deep);
    draw_line(
        bay.x + 28.0,
        top + 31.0,
        bay.right() - 28.0,
        top + 31.0,
        2.0,
        structure,
    );
    draw_line(
        bay.x + 52.0,
        top + 70.0,
        bay.right() - 38.0,
        top + 70.0,
        2.0,
        structure,
    );
    draw_line(
        bay.x + 52.0,
        top + 77.0,
        bay.right() - 38.0,
        top + 77.0,
        1.0,
        structure,
    );
    for index in 0..5 {
        let x = bay.x + 58.0 + index as f32 * 166.0;
        draw_line(x, top + 25.0, x + 30.0, floor - 12.0, 2.0, deep);
        draw_line(x + 13.0, top + 25.0, x + 42.0, floor - 12.0, 1.0, structure);
    }
    for index in 0..4 {
        let x = bay.x + 82.0 + index as f32 * 188.0;
        let length = 28.0 + (index % 2) as f32 * 18.0;
        draw_line(x, top + 88.0, x, top + 88.0 + length, 2.0, structure);
        draw_circle(x, top + 88.0 + length, 3.0, visual_theme::amber());
    }
    draw_rectangle(
        bay.x + 74.0,
        top + 44.0,
        112.0,
        5.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.72),
    );
    draw_rectangle(
        bay.right() - 188.0,
        top + 44.0,
        112.0,
        5.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.72),
    );
    draw_line(
        bay.x + 20.0,
        floor,
        bay.right() - 20.0,
        floor,
        2.0,
        structure,
    );
    for index in 0..9 {
        let x = bay.x + 54.0 + index as f32 * 82.0;
        draw_line(x, floor + 28.0, x + 32.0, floor + 28.0, 3.0, deep);
        draw_line(
            x + 40.0,
            floor + 28.0,
            x + 52.0,
            floor + 28.0,
            3.0,
            structure,
        );
    }
    draw_service_cart(bay.x + 46.0, floor - 54.0);
    draw_line(
        bay.x + 118.0,
        floor - 2.0,
        bay.x + 118.0,
        floor - 28.0,
        2.0,
        visual_theme::amber(),
    );
    draw_line(
        bay.right() - 104.0,
        floor - 2.0,
        bay.right() - 104.0,
        floor - 28.0,
        2.0,
        visual_theme::amber(),
    );
}

fn draw_service_cart(x: f32, y: f32) {
    draw_rectangle(
        x,
        y,
        70.0,
        28.0,
        visual_theme::with_alpha(visual_theme::structure_dark(), 0.92),
    );
    draw_rectangle_lines(x, y, 70.0, 28.0, 1.0, visual_theme::structure_light());
    draw_rectangle(x + 8.0, y + 7.0, 22.0, 5.0, visual_theme::amber());
    draw_rectangle(x + 38.0, y + 7.0, 20.0, 5.0, visual_theme::cyan_dim());
    draw_circle(x + 12.0, y + 31.0, 4.0, visual_theme::structure_light());
    draw_circle(x + 58.0, y + 31.0, 4.0, visual_theme::structure_light());
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
        if ctx.pointer.released_on(hit) {
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
    if ctx.pointer.released_on(emitter) {
        actions.push(UiAction::SelectPortModule("engine_core".to_owned()));
    }
}

fn draw_cargo_hold(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let hold_height = if ctx.port_hold_expanded { 164.0 } else { 104.0 };
    let hold = Rect::new(
        BAY.x + 28.0,
        BAY.bottom() - hold_height - 20.0,
        BAY.w - 56.0,
        hold_height,
    );
    if ctx.port_hold_expanded {
        panel(hold, visual_theme::with_alpha(visual_theme::panel(), 0.96));
    }
    draw_line(
        hold.x,
        hold.y - 14.0,
        hold.right(),
        hold.y - 14.0,
        1.0,
        visual_theme::structure_light(),
    );
    draw_text(
        "CARGO HOLD",
        hold.x,
        hold.y + 12.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!(
            "{} / {}",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        hold.x,
        hold.y + 43.0,
        28.0,
        visual_theme::amber(),
    );
    draw_text(
        "PHYSICAL PACKING CAPACITY",
        hold.x,
        hold.y + 67.0,
        11.0,
        visual_theme::text_dim(),
    );
    let button_rect = Rect::new(hold.right() - 174.0, hold.y + 12.0, 174.0, 44.0);
    if button(
        ctx,
        button_rect,
        if ctx.port_hold_expanded {
            "HIDE HOLD"
        } else {
            "VIEW HOLD"
        },
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePortHold);
    }
    if ctx.port_hold_expanded {
        let grid = Rect::new(hold.x + 300.0, hold.y + 14.0, 160.0, 100.0);
        draw_ship_grid(ctx, grid, false, actions);
        draw_text(
            "5 × 5",
            hold.x + 228.0,
            hold.y + 50.0,
            15.0,
            visual_theme::cyan(),
        );
        draw_text(
            "HOLD MAP",
            hold.x + 216.0,
            hold.y + 72.0,
            10.0,
            visual_theme::text_dim(),
        );
    } else {
        draw_text(
            "Tap VIEW HOLD to inspect the grid.",
            hold.x + 202.0,
            hold.y + 36.0,
            13.0,
            visual_theme::text(),
        );
        draw_text(
            "Recovered hardware must fit before return.",
            hold.x + 202.0,
            hold.y + 57.0,
            11.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_shipyard(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel(CONSOLE, visual_theme::panel());
    draw_rectangle(
        CONSOLE.x,
        CONSOLE.y,
        CONSOLE.w,
        48.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "SHIPYARD",
        CONSOLE.x + 18.0,
        CONSOLE.y + 31.0,
        20.0,
        visual_theme::text(),
    );
    let offline = ctx.session.damaged_modules.len();
    draw_text(
        if offline == 0 {
            "SYSTEMS NOMINAL"
        } else {
            "SYSTEMS NEED SERVICE"
        },
        CONSOLE.right() - 154.0,
        CONSOLE.y + 30.0,
        11.0,
        if offline == 0 {
            visual_theme::safe()
        } else {
            visual_theme::warning()
        },
    );
    draw_tabs(CONSOLE);
    draw_selected_module(ctx, actions);
    draw_yard_stock(ctx, actions);
    draw_services(ctx, actions);
}

fn draw_tabs(console: Rect) {
    draw_rectangle(
        console.x + 16.0,
        console.y + 62.0,
        118.0,
        30.0,
        visual_theme::cyan_dim(),
    );
    draw_text(
        "EQUIPMENT",
        console.x + 29.0,
        console.y + 83.0,
        12.0,
        visual_theme::text(),
    );
    draw_text(
        "SERVICES",
        console.x + 156.0,
        console.y + 83.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_line(
        console.x + 16.0,
        console.y + 94.0,
        console.right() - 16.0,
        console.y + 94.0,
        1.0,
        visual_theme::structure(),
    );
}

fn draw_selected_module(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let card = Rect::new(CONSOLE.x + 16.0, CONSOLE.y + 108.0, CONSOLE.w - 32.0, 122.0);
    panel(card, visual_theme::panel_soft());
    let Some(module_id) = ctx.port_selected_module else {
        draw_text(
            "SELECT A SHIP MOUNT",
            card.x + 16.0,
            card.y + 36.0,
            17.0,
            visual_theme::amber(),
        );
        draw_text(
            "Tap the illuminated machinery in the hangar.",
            card.x + 16.0,
            card.y + 64.0,
            12.0,
            visual_theme::text_dim(),
        );
        return;
    };
    let Some(module) = ctx.data.modules.get(module_id) else {
        return;
    };
    let display_name = if module.id == "engine_core" {
        "TRACTOR EMITTER"
    } else {
        module.display_name.as_str()
    };
    draw_text(
        &display_name.to_uppercase(),
        card.x + 16.0,
        card.y + 25.0,
        18.0,
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
        &mount_label,
        card.x + 16.0,
        card.y + 45.0,
        11.0,
        visual_theme::cyan(),
    );
    draw_text(
        &clipped(&module.description, 49),
        card.x + 16.0,
        card.y + 66.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        &module_stock_detail(module),
        card.x + 16.0,
        card.y + 86.0,
        12.0,
        visual_theme::text(),
    );
    let installed = ctx
        .session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == module.id);
    if installed
        && button(
            ctx,
            Rect::new(card.right() - 112.0, card.y + 68.0, 94.0, 40.0),
            &format!("REMOVE ¢{}", module.remove_cost),
            true,
            ButtonTone::Warning,
        )
    {
        actions.push(UiAction::RemoveModule(module.id.clone()));
    }
}

fn draw_yard_stock(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_text(
        "YARD STOCK",
        CONSOLE.x + 16.0,
        CONSOLE.y + 252.0,
        14.0,
        visual_theme::text_dim(),
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
    for (index, (_, module)) in stock.iter().enumerate() {
        let row = Rect::new(
            CONSOLE.x + 16.0,
            CONSOLE.y + 262.0 + index as f32 * 28.0,
            CONSOLE.w - 32.0,
            25.0,
        );
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            visual_theme::with_alpha(visual_theme::panel_soft(), 0.66),
        );
        draw_rectangle(row.x, row.y, 3.0, row.h, visual_theme::amber());
        draw_text(
            &module.display_name.to_uppercase(),
            row.x + 11.0,
            row.y + 16.0,
            12.0,
            visual_theme::text(),
        );
        draw_text(
            &module_stock_detail(module),
            row.x + 142.0,
            row.y + 16.0,
            10.0,
            visual_theme::text_dim(),
        );
        let fits = ctx
            .session
            .ship_layout
            .first_fit(&module.id, module.footprint, true)
            .is_some();
        let affordable = ctx.session.economy.credits >= module.purchase_cost;
        let enabled = fits && affordable;
        let label = if !fits {
            "NO FIT".to_owned()
        } else if !affordable {
            "LOW CR".to_owned()
        } else {
            format!("BUY ¢{}", module.purchase_cost)
        };
        if button(
            ctx,
            Rect::new(row.right() - 74.0, row.y + 1.0, 66.0, 23.0),
            &label,
            enabled,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::PurchaseModule(module.id.clone()));
        }
    }
}

fn draw_services(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let y = CONSOLE.bottom() - 88.0;
    if button(
        ctx,
        Rect::new(CONSOLE.x + 16.0, y, 116.0, 36.0),
        "REFUEL",
        true,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Refuel);
    }
    if button(
        ctx,
        Rect::new(CONSOLE.x + 142.0, y, 116.0, 36.0),
        "REPAIR",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::Repair);
    }
    if button(
        ctx,
        Rect::new(CONSOLE.x + 16.0, y + 46.0, CONSOLE.w - 32.0, 42.0),
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
