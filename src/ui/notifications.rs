//! Pause overlay and quiet interaction messaging.

use super::*;

pub fn draw_pause(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.01, 0.02, 0.03, 0.82),
    );
    panel_title(Rect::new(240.0, 104.0, 800.0, 520.0), state::pause::TITLE);
    draw_text(
        "Your current run is safe until you resume.",
        300.0,
        190.0,
        20.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        "Save only at port or after resolving a return.",
        300.0,
        222.0,
        16.0,
        dark::TEXT_DIM,
    );
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
        "SAVE",
        ctx.resume_state == GameState::Port,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        ctx,
        Rect::new(864.0, 262.0, 116.0, 52.0),
        "LOAD",
        ctx.save_exists,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        ctx,
        Rect::new(300.0, 344.0, 214.0, 52.0),
        "MAIN MENU",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::BackToMainMenu);
    }
    if button(
        ctx,
        Rect::new(526.0, 344.0, 214.0, 52.0),
        "NEW GAME",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::NewGame);
    }
    if button(
        ctx,
        Rect::new(752.0, 344.0, 228.0, 52.0),
        "EXIT GAME",
        true,
        ButtonTone::Danger,
    ) {
        actions.push(UiAction::ExitGame);
    }
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
