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
    panel_title(Rect::new(320.0, 176.0, 640.0, 356.0), state::pause::TITLE);
    draw_text(
        "Your current run is safe until you resume.",
        380.0,
        248.0,
        20.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        "Save only at port or after resolving a return.",
        380.0,
        280.0,
        16.0,
        dark::TEXT_DIM,
    );
    if button(
        ctx,
        Rect::new(380.0, 328.0, 190.0, 48.0),
        "RESUME",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::TogglePause);
    }
    if button(
        ctx,
        Rect::new(590.0, 328.0, 170.0, 48.0),
        "SAVE",
        ctx.resume_state == GameState::Port,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        ctx,
        Rect::new(780.0, 328.0, 130.0, 48.0),
        "LOAD",
        ctx.save_exists,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        ctx,
        Rect::new(380.0, 400.0, 230.0, 48.0),
        "NEW GAME",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::NewGame);
    }
    draw_text(
        "CHECKPOINT TOOLS",
        640.0,
        430.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        if ctx.save_exists {
            "SAVE SLOT READY // LOAD RESTORES THE LAST PORT"
        } else {
            "NO SAVE SLOT // SAVE IS AVAILABLE AT PORT"
        },
        640.0,
        454.0,
        11.0,
        visual_theme::text_dim(),
    );
}
