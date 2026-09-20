//! Main-menu presentation reached from the pause menu.

use super::*;

pub fn draw_main_menu(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        visual_theme::with_alpha(visual_theme::space(), 0.44),
    );
    crate::ui::header::draw_menu_header();
    panel_title(Rect::new(300.0, 104.0, 680.0, 462.0), "COMMAND DECK");
    visual_theme::body(
        "A patched vessel. A cold wreck. One more run.",
        Rect::new(356.0, 178.0, 568.0, 36.0),
        20.0,
        visual_theme::text(),
    );
    visual_theme::body(
        "Choose an operation.",
        Rect::new(356.0, 216.0, 568.0, 28.0),
        16.0,
        visual_theme::text_dim(),
    );

    let continue_enabled = ctx.resume_state != GameState::MainMenu;
    if button(
        ctx,
        Rect::new(356.0, 268.0, 568.0, 52.0),
        "CONTINUE RUN",
        continue_enabled,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::ContinueGame);
    }
    if button(
        ctx,
        Rect::new(356.0, 338.0, 276.0, 52.0),
        "NEW GAME",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::NewGame);
    }
    if button(
        ctx,
        Rect::new(648.0, 338.0, 276.0, 52.0),
        "LOAD",
        ctx.save_exists,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        ctx,
        Rect::new(356.0, 420.0, 568.0, 52.0),
        "EXIT GAME",
        true,
        ButtonTone::Danger,
    ) {
        actions.push(UiAction::ExitGame);
    }
    visual_theme::body(
        if ctx.save_exists {
            "SAVE SLOT READY"
        } else {
            "NO SAVE SLOT YET"
        },
        Rect::new(356.0, 496.0, 568.0, 26.0),
        12.0,
        visual_theme::text_dim(),
    );
}
