//! Touch-friendly settings panel opened from the pause menu.

use super::*;

pub fn draw_settings(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        visual_theme::with_alpha(visual_theme::space(), 0.86),
    );
    panel_title(Rect::new(300.0, 112.0, 680.0, 470.0), "SETTINGS");
    draw_text(
        "Preferences are saved separately from your salvage checkpoint.",
        356.0,
        198.0,
        17.0,
        visual_theme::text_dim(),
    );

    draw_setting_row(
        ctx,
        actions,
        246.0,
        "FULLSCREEN",
        if ctx.fullscreen { "ON" } else { "OFF" },
        UiAction::ToggleFullscreen,
    );
    draw_setting_row(
        ctx,
        actions,
        322.0,
        "REDUCED MOTION",
        if ctx.reduced_motion { "ON" } else { "OFF" },
        UiAction::ToggleReducedMotion,
    );
    draw_text(
        "Reduced motion keeps the readouts stable while preserving gameplay feedback.",
        356.0,
        422.0,
        13.0,
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(356.0, 482.0, 276.0, 52.0),
        "BACK TO MENU",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CloseSettings);
    }
    if button(
        ctx,
        Rect::new(648.0, 482.0, 276.0, 52.0),
        "BACK TO GAME",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::TogglePause);
    }
}

fn draw_setting_row(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    y: f32,
    label: &str,
    value: &str,
    action: UiAction,
) {
    draw_text(label, 356.0, y + 31.0, 18.0, visual_theme::text());
    if button(
        ctx,
        Rect::new(736.0, y, 188.0, 52.0),
        value,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(action);
    }
}
