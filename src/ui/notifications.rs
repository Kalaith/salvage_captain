//! Pause overlay and quiet interaction messaging.

use super::*;

pub fn draw_pause(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        visual_theme::with_alpha(visual_theme::space(), 0.82),
    );
    panel_title(
        Rect::new(240.0, 104.0, 800.0, 520.0),
        crate::state::pause::TITLE,
    );
    draw_text(
        "Your current run is safe until you resume.",
        300.0,
        190.0,
        20.0,
        visual_theme::text(),
    );
    draw_text(
        "Save only at port or after resolving a return.",
        300.0,
        222.0,
        16.0,
        visual_theme::text_dim(),
    );
    draw_pause_controls(ctx, actions);
    draw_pause_checkpoint(ctx);
}

pub fn draw_help(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        visual_theme::with_alpha(visual_theme::space(), 0.88),
    );
    let frame = Rect::new(222.0, 96.0, 836.0, 528.0);
    panel_title(frame, "HELP // TOUCH PATH");
    let screen = if ctx.state == GameState::Pause {
        ctx.resume_state
    } else {
        ctx.state
    };
    let copy = match screen {
        GameState::Port => &ctx.data.salvage_ui.help.port,
        GameState::SiteSelection => &ctx.data.salvage_ui.help.selection,
        GameState::Travel | GameState::ReturnTravel => &ctx.data.salvage_ui.help.travel,
        GameState::SalvageWorkspace => &ctx.data.salvage_ui.help.salvage,
        GameState::CargoInventory => &ctx.data.salvage_ui.help.inventory,
        GameState::Results => &ctx.data.salvage_ui.help.results,
        GameState::MainMenu | GameState::Pause => &ctx.data.salvage_ui.help.port,
    };
    visual_theme::body(
        "CURRENT PHASE",
        Rect::new(frame.x + 42.0, frame.y + 76.0, 250.0, 30.0),
        18.0,
        visual_theme::amber(),
    );
    visual_theme::body(
        copy,
        Rect::new(frame.x + 42.0, frame.y + 122.0, frame.w - 84.0, 142.0),
        24.0,
        visual_theme::text(),
    );
    visual_theme::body(
        "Help stays available from PAUSE > HELP. Close it to return to the exact phase you left.",
        Rect::new(frame.x + 42.0, frame.y + 300.0, frame.w - 84.0, 62.0),
        18.0,
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(frame.x + 42.0, frame.bottom() - 76.0, 300.0, 48.0),
        "CLOSE HELP",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::CloseHelp);
    }
}

fn draw_pause_controls(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    if button(
        ctx,
        Rect::new(300.0, 262.0, 170.0, 52.0),
        "RESUME",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::TogglePause);
    }
    if button(
        ctx,
        Rect::new(488.0, 262.0, 170.0, 52.0),
        "SETTINGS",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::OpenSettings);
    }
    if button(
        ctx,
        Rect::new(676.0, 262.0, 170.0, 52.0),
        "HELP",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleHelp);
    }
    if button(
        ctx,
        Rect::new(864.0, 262.0, 116.0, 52.0),
        "SAVE",
        ctx.resume_state == GameState::Port,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        ctx,
        Rect::new(300.0, 344.0, 116.0, 52.0),
        "LOAD",
        ctx.save_exists,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        ctx,
        Rect::new(434.0, 344.0, 214.0, 52.0),
        "MAIN MENU",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BackToMainMenu);
    }
    if button(
        ctx,
        Rect::new(662.0, 344.0, 146.0, 52.0),
        "NEW GAME",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::NewGame);
    }
    if button(
        ctx,
        Rect::new(822.0, 344.0, 158.0, 52.0),
        "EXIT GAME",
        true,
        ButtonTone::Danger,
    ) {
        actions.push(UiAction::ExitGame);
    }
}

fn draw_pause_checkpoint(ctx: &UiContext<'_>) {
    draw_text(
        "CHECKPOINT TOOLS",
        300.0,
        440.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        if ctx.save_exists {
            "SAVE SLOT READY // LOAD RESTORES THE LAST PORT"
        } else {
            "NO SAVE SLOT // SAVE IS AVAILABLE AT PORT"
        },
        300.0,
        466.0,
        11.0,
        visual_theme::text_dim(),
    );
}
