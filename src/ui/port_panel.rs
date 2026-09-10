//! Port panel: repairs, refuelling, save controls, and installed ship status.

use super::*;

pub fn draw_port(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel_title(Rect::new(24.0, 112.0, 430.0, 494.0), "SHIP LAYOUT");
    draw_ship_grid(ctx, Rect::new(58.0, 180.0, 362.0, 362.0), false, actions);
    draw_text(
        "Installed modules occupy the same cells as salvage.",
        50.0,
        582.0,
        14.0,
        dark::TEXT_DIM,
    );

    panel_title(Rect::new(480.0, 112.0, 776.0, 494.0), state::port::TITLE);
    draw_text(
        "The port is the only safe checkpoint. Spend credits to prepare the next run.",
        512.0,
        164.0,
        17.0,
        dark::TEXT,
    );
    let stats = ctx.session.module_stats(ctx.data);
    draw_text(
        format!(
            "Fuel capacity  {}/{}",
            ctx.session.economy.fuel,
            ctx.session.max_fuel(ctx.data)
        ),
        512.0,
        208.0,
        19.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        format!(
            "Hull integrity  {}/{}     Risk protection  {}",
            ctx.session.hull,
            ctx.session.max_hull_with_modules(ctx.data),
            stats.shielding + stats.scanning / 2
        ),
        512.0,
        238.0,
        17.0,
        dark::TEXT_DIM,
    );
    draw_text("Installed:", 512.0, 286.0, 17.0, dark::TEXT_BRIGHT);
    for (index, item) in ctx
        .session
        .ship_layout
        .placements
        .iter()
        .filter(|item| item.permanent)
        .enumerate()
    {
        let module = ctx
            .data
            .modules
            .get(&item.id)
            .map(|module| (module.display_name.as_str(), module.remove_cost));
        let name = module.map_or(item.id.as_str(), |(name, _)| name);
        let remove_label = module.map_or_else(
            || "REMOVE".to_owned(),
            |(_, cost)| format!("REMOVE {}", cost),
        );
        let row_y = 314.0 + index as f32 * 30.0;
        draw_text(format!("- {}", name), 528.0, row_y, 16.0, dark::TEXT);
        if button(
            ctx,
            Rect::new(1098.0, row_y - 21.0, 130.0, 26.0),
            &remove_label,
            true,
            ButtonTone::Warning,
        ) {
            actions.push(UiAction::RemoveModule(item.id.clone()));
        }
    }
    let button_y = 470.0;
    if button(
        ctx,
        Rect::new(512.0, button_y, 168.0, 44.0),
        "REFUEL",
        true,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Refuel);
    }
    if button(
        ctx,
        Rect::new(690.0, button_y, 168.0, 44.0),
        "REPAIR",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::Repair);
    }
    if button(
        ctx,
        Rect::new(868.0, button_y, 190.0, 44.0),
        "BROWSE WRECKS",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::GoToSites);
    }
    if button(
        ctx,
        Rect::new(1068.0, button_y, 160.0, 44.0),
        "NEW GAME",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::NewGame);
    }
    if button(
        ctx,
        Rect::new(512.0, 528.0, 168.0, 40.0),
        "SAVE",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        ctx,
        Rect::new(690.0, 528.0, 168.0, 40.0),
        "LOAD",
        ctx.save_exists,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Load);
    }
    let save_label = if ctx.save_exists {
        "SAVE SLOT READY"
    } else {
        "NO SAVE SLOT"
    };
    draw_text(
        format!(
            "{}  -  assets {}  -  {}",
            save_label,
            ctx.loaded_assets,
            ctx.save_slots.join(", ")
        ),
        880.0,
        554.0,
        14.0,
        dark::TEXT_DIM,
    );
    if ctx.session.milestone_reached {
        badge(
            Rect::new(880.0, 510.0, 348.0, 30.0),
            "MILESTONE: NEXT BUILD UNLOCKED",
            Color::new(0.18, 0.34, 0.22, 1.0),
        );
    }
}
