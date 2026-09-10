//! Hangar command deck: the captain reads the vessel before choosing a wreck.

use super::*;
use crate::data::{ModuleData, ModuleEffect};
use crate::ui::ship_visual;
use crate::ui::visual_theme;

pub fn draw_port(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_hangar_bay(ctx, actions);
    draw_yard_console(ctx, actions);
}

fn draw_hangar_bay(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let bay = Rect::new(24.0, 112.0, 694.0, 494.0);
    panel(bay, visual_theme::panel_soft());
    draw_rectangle(bay.x, bay.y, bay.w, 42.0, visual_theme::structure_dark());
    draw_text(
        "HANGAR BAY  //  SC-07",
        bay.x + 18.0,
        bay.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "PATCHED INDUSTRIAL WORKBOAT",
        bay.x + 430.0,
        bay.y + 27.0,
        11.0,
        visual_theme::text_dim(),
    );

    draw_bay_structure(bay);
    ship_visual::draw_ship(
        Rect::new(bay.x + 82.0, bay.y + 96.0, 500.0, 220.0),
        ctx.session,
        ctx.data,
        0.0,
        false,
    );
    draw_text(
        "PHYSICAL LOADOUT",
        bay.x + 24.0,
        bay.y + 354.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!(
            "HULL  {}/{}",
            ctx.session.hull,
            ctx.session.max_hull_with_modules(ctx.data)
        ),
        bay.x + 24.0,
        bay.y + 382.0,
        15.0,
        visual_theme::text(),
    );
    visual_theme::draw_meter(
        Rect::new(bay.x + 24.0, bay.y + 392.0, 190.0, 18.0),
        ctx.session.hull as f32 / ctx.session.max_hull_with_modules(ctx.data).max(1) as f32,
        visual_theme::safe(),
        "",
    );
    draw_text(
        format!(
            "FUEL  {}/{}",
            ctx.session.economy.fuel,
            ctx.session.max_fuel(ctx.data)
        ),
        bay.x + 238.0,
        bay.y + 382.0,
        15.0,
        visual_theme::text(),
    );
    visual_theme::draw_meter(
        Rect::new(bay.x + 238.0, bay.y + 392.0, 190.0, 18.0),
        ctx.session.economy.fuel as f32 / ctx.session.max_fuel(ctx.data).max(1) as f32,
        visual_theme::cyan(),
        "",
    );
    draw_text(
        "CARGO CELLS",
        bay.x + 454.0,
        bay.y + 382.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!(
            "{}/{}",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        bay.x + 454.0,
        bay.y + 407.0,
        22.0,
        visual_theme::amber(),
    );

    draw_ship_grid(
        ctx,
        Rect::new(bay.x + 24.0, bay.y + 414.0, 240.0, 36.0),
        false,
        actions,
    );
    draw_text(
        "GRID IS THE HOLD",
        bay.x + 288.0,
        bay.y + 438.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        "Every recovered object must fit",
        bay.x + 288.0,
        bay.y + 461.0,
        13.0,
        visual_theme::text(),
    );
    draw_text(
        "before the vessel can come home.",
        bay.x + 288.0,
        bay.y + 482.0,
        13.0,
        visual_theme::text(),
    );
}

fn draw_bay_structure(bay: Rect) {
    for index in 0..5 {
        let x = bay.x + 34.0 + index as f32 * 152.0;
        draw_line(
            x,
            bay.y + 58.0,
            x + 28.0,
            bay.y + 326.0,
            2.0,
            visual_theme::structure_dark(),
        );
        draw_line(
            x + 12.0,
            bay.y + 58.0,
            x + 40.0,
            bay.y + 326.0,
            1.0,
            visual_theme::structure_light(),
        );
    }
    for index in 0..4 {
        let y = bay.y + 116.0 + index as f32 * 62.0;
        draw_line(
            bay.x + 18.0,
            y,
            bay.right() - 18.0,
            y + 12.0,
            1.0,
            visual_theme::structure_light(),
        );
    }
    draw_rectangle(bay.x + 52.0, bay.y + 70.0, 90.0, 4.0, visual_theme::amber());
    draw_rectangle(
        bay.right() - 142.0,
        bay.y + 70.0,
        90.0,
        4.0,
        visual_theme::amber(),
    );
}

fn draw_yard_console(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let console = Rect::new(738.0, 112.0, 518.0, 494.0);
    panel(console, visual_theme::panel());
    draw_rectangle(
        console.x,
        console.y,
        console.w,
        42.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "YARD CONSOLE",
        console.x + 18.0,
        console.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "SAFE CHECKPOINT",
        console.right() - 142.0,
        console.y + 27.0,
        11.0,
        visual_theme::safe(),
    );
    draw_resource_strip(ctx, console);
    draw_module_manifest(ctx, console, actions);
    draw_console_actions(ctx, console, actions);
}

fn draw_resource_strip(ctx: &UiContext<'_>, console: Rect) {
    let resources = [
        (
            "CREDITS",
            format!("¢{}", ctx.session.economy.credits),
            visual_theme::safe(),
        ),
        (
            "ALLOY",
            ctx.session.economy.alloy.to_string(),
            visual_theme::amber(),
        ),
        (
            "ELECTRONICS",
            ctx.session.economy.electronics.to_string(),
            visual_theme::cyan(),
        ),
        (
            "CLAMPS",
            format!(
                "{}/{}",
                ctx.session.external_cargo_count(ctx.data, None),
                ctx.session.external_capacity(ctx.data)
            ),
            visual_theme::amber(),
        ),
    ];
    for (index, (label, value, color)) in resources.into_iter().enumerate() {
        let x = console.x + 18.0 + index as f32 * 120.0;
        draw_text(label, x, console.y + 72.0, 10.0, visual_theme::text_dim());
        draw_text(value, x, console.y + 96.0, 21.0, color);
    }
    draw_line(
        console.x + 18.0,
        console.y + 110.0,
        console.right() - 18.0,
        console.y + 110.0,
        1.0,
        visual_theme::structure(),
    );
}

fn draw_module_manifest(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    draw_text(
        "INSTALLED SYSTEMS",
        console.x + 18.0,
        console.y + 136.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        "YARD STOCK",
        console.x + 270.0,
        console.y + 136.0,
        13.0,
        visual_theme::text_dim(),
    );
    let modules: Vec<_> = ctx
        .session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
        .collect();
    if modules.is_empty() {
        draw_text(
            "NO PERMANENT MODULES",
            console.x + 18.0,
            console.y + 170.0,
            14.0,
            visual_theme::warning(),
        );
    }
    for (index, item) in modules.iter().enumerate() {
        let row = Rect::new(
            console.x + 16.0,
            console.y + 148.0 + index as f32 * 32.0,
            240.0,
            26.0,
        );
        draw_rectangle(row.x, row.y, row.w, row.h, visual_theme::panel_soft());
        draw_rectangle(row.x, row.y, 4.0, row.h, visual_theme::cyan_dim());
        let module = ctx.data.modules.get(&item.id);
        let name = module.map_or(item.id.as_str(), |module| module.display_name.as_str());
        let capability = module.and_then(|module| module.capability.as_deref());
        draw_text(
            &name.to_uppercase(),
            row.x + 14.0,
            row.y + 18.0,
            12.0,
            visual_theme::text(),
        );
        if let Some(capability) = capability {
            draw_text(
                &capability.replace('_', " ").to_uppercase(),
                row.x + 114.0,
                row.y + 17.0,
                9.0,
                visual_theme::text_dim(),
            );
        }
        let cost = module.map_or(0, |module| module.remove_cost);
        if button(
            ctx,
            Rect::new(row.right() - 84.0, row.y + 2.0, 74.0, 22.0),
            &format!("REMOVE ¢{}", cost),
            true,
            ButtonTone::Warning,
        ) {
            actions.push(UiAction::RemoveModule(item.id.clone()));
        }
    }
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
            console.x + 270.0,
            console.y + 148.0 + index as f32 * 32.0,
            232.0,
            26.0,
        );
        draw_rectangle(row.x, row.y, row.w, row.h, visual_theme::panel_soft());
        draw_rectangle(row.x, row.y, 3.0, row.h, visual_theme::amber());
        draw_text(
            &module.display_name.to_uppercase(),
            row.x + 10.0,
            row.y + 17.0,
            11.0,
            visual_theme::text(),
        );
        draw_text(
            &module_stock_detail(module),
            row.x + 10.0,
            row.y + 25.0,
            8.0,
            visual_theme::cyan_dim(),
        );
        let fits = ctx
            .session
            .ship_layout
            .first_fit(&module.id, module.footprint, true)
            .is_some();
        let affordable = ctx.session.economy.credits >= module.purchase_cost;
        let enabled = affordable && fits;
        let buy_label = if !fits {
            "NO FIT".to_owned()
        } else if !affordable {
            format!("NEED ¢{}", module.purchase_cost)
        } else {
            format!("BUY ¢{}", module.purchase_cost)
        };
        if button(
            ctx,
            Rect::new(row.right() - 78.0, row.y + 2.0, 70.0, 22.0),
            &buy_label,
            enabled,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::PurchaseModule(module.id.clone()));
        }
    }
}

fn module_stock_detail(module: &ModuleData) -> String {
    if let Some(capability) = &module.capability {
        return format!("CAP {}", capability.replace('_', " ").to_uppercase());
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

fn draw_console_actions(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    let y = console.y + 344.0;
    draw_text(
        "NEXT RUN",
        console.x + 18.0,
        y - 16.0,
        13.0,
        visual_theme::text_dim(),
    );
    let buttons = [
        (
            Rect::new(console.x + 18.0, y, 150.0, 44.0),
            "REFUEL",
            ButtonTone::Primary,
            UiAction::Refuel,
        ),
        (
            Rect::new(console.x + 178.0, y, 150.0, 44.0),
            "REPAIR",
            ButtonTone::Warning,
            UiAction::Repair,
        ),
        (
            Rect::new(console.x + 338.0, y, 160.0, 44.0),
            "BROWSE WRECKS",
            ButtonTone::Positive,
            UiAction::GoToSites,
        ),
        (
            Rect::new(console.x + 18.0, y + 54.0, 150.0, 38.0),
            "SAVE",
            ButtonTone::Secondary,
            UiAction::Save,
        ),
        (
            Rect::new(console.x + 178.0, y + 54.0, 150.0, 38.0),
            "LOAD",
            ButtonTone::Secondary,
            UiAction::Load,
        ),
        (
            Rect::new(console.x + 338.0, y + 54.0, 160.0, 38.0),
            "NEW GAME",
            ButtonTone::Secondary,
            UiAction::NewGame,
        ),
    ];
    for (rect, label, tone, action) in buttons {
        let enabled = !matches!(action, UiAction::Load) || ctx.save_exists;
        if button(ctx, rect, label, enabled, tone) {
            actions.push(action);
        }
    }
    draw_text(
        if ctx.save_exists {
            "SAVE SLOT READY"
        } else {
            "NO SAVE SLOT"
        },
        console.x + 18.0,
        console.y + 468.0,
        12.0,
        visual_theme::text_dim(),
    );
}
