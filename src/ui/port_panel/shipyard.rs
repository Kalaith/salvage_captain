//! Shipyard module selection, stock, and service panels.

use super::*;

pub(super) fn draw_shipyard(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
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
    let wear = ctx.session.ship_wear();
    let system_label = if offline == 0 {
        if wear == 0 {
            "SYSTEMS NOMINAL"
        } else {
            "SYSTEMS WORN"
        }
    } else {
        "SYSTEMS NEED SERVICE"
    };
    draw_text(
        system_label,
        console.right() - 142.0,
        console.y + 28.0,
        9.0,
        if offline > 0 {
            visual_theme::warning()
        } else if wear > 0 {
            visual_theme::amber()
        } else {
            visual_theme::safe()
        },
    );
    maintenance::draw_wear_meter(console, wear);
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
        clipped(&standing_label, 30),
        console.x + 124.0,
        console.y + 28.0,
        9.0,
        visual_theme::cyan(),
    );
    draw_tabs(console);

    let selected = Rect::new(console.x + 14.0, console.y + 102.0, console.w - 28.0, 116.0);
    draw_selected_module(ctx, selected, actions);
    let refinery_y = selected.bottom() + 20.0;
    refinery::draw_refinery_console(ctx, console, refinery_y, actions);
    draw_yard_stock(ctx, console, refinery_y + 68.0, actions);
    draw_services(ctx, console, actions);
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
        clipped(&display_name.to_uppercase(), 24),
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
        clipped(&mount_label, 42),
        card.x + 16.0,
        card.y + 44.0,
        10.0,
        visual_theme::cyan(),
    );
    draw_text(
        clipped(&module.description, 48),
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
        clipped(&detail, 34),
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
    let damaged = ctx
        .session
        .damaged_modules
        .iter()
        .any(|damaged_id| damaged_id == &module.id);
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
    let status = selected_module_status(unlocked, installed, fits, damaged, module.unlock_credits);
    draw_text(
        &status,
        card.x + 16.0,
        card.bottom() - 10.0,
        9.0,
        if !unlocked || damaged {
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

fn selected_module_status(
    unlocked: bool,
    installed: bool,
    fits: bool,
    damaged: bool,
    unlock_credits: i64,
) -> String {
    if !unlocked {
        format!("LOCKED // EARN ¢{unlock_credits}")
    } else if installed && damaged {
        "INSTALLED // OFFLINE // SERVICE DUE".to_owned()
    } else if installed {
        "INSTALLED".to_owned()
    } else if fits {
        "PREVIEW ACTIVE".to_owned()
    } else {
        "NO FIT // PREVIEW".to_owned()
    }
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
        clipped(&next_unlock, 24),
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
    stock.sort_by_key(|(left, _)| *left);

    let columns = if console.w >= 360.0 { 2 } else { 1 };
    let gap = 8.0;
    let inner_width = console.w - 28.0;
    let card_width = (inner_width - gap * (columns as f32 - 1.0)) / columns as f32;
    let service_top = console.bottom() - 84.0;
    let stock_top = section_y + 14.0;
    let rows = stock.len().div_ceil(columns);
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
            clipped(&module.display_name.to_uppercase(), name_limit),
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
            clipped(&detail, name_limit + 5),
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
    let quote = ctx.session.repair_quote(ctx.data);
    let completion_label = maintenance_completion_label(ctx.message);
    let status_label = if quote.is_due() {
        maintenance_status_label(
            quote.missing_hull,
            quote.offline_modules,
            quote.ship_wear,
            quote.total_cost,
            ctx.session.economy.credits,
        )
    } else {
        completion_label
            .unwrap_or("SYSTEMS NOMINAL // NO SERVICE DUE")
            .to_owned()
    };
    draw_text(
        clipped(&status_label, 48),
        inner_x,
        y - 12.0,
        10.0,
        if quote.is_due() {
            visual_theme::warning()
        } else {
            visual_theme::safe()
        },
    );
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
        &repair_button_label(quote.total_cost),
        quote.is_due() && ctx.session.economy.credits >= quote.total_cost,
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
