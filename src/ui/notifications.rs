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
    panel_title(Rect::new(370.0, 214.0, 540.0, 250.0), state::pause::TITLE);
    draw_text(
        "Your current run is safe until you resume.",
        430.0,
        286.0,
        20.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        "Save only at port or after resolving a return.",
        430.0,
        318.0,
        16.0,
        dark::TEXT_DIM,
    );
    if button(
        ctx,
        Rect::new(430.0, 360.0, 190.0, 48.0),
        "RESUME",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::TogglePause);
    }
    if button(
        ctx,
        Rect::new(640.0, 360.0, 190.0, 48.0),
        "SAVE",
        ctx.save_exists || ctx.state == GameState::Port,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Save);
    }
}
