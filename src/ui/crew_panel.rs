//! Compact crew assignment control used during mission briefing.

use super::*;
use crate::state::CrewRole;
use crate::ui::visual_theme;

pub fn draw_briefing_control(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let rect = Rect::new(1008.0, 158.0, 216.0, 30.0);
    draw_assignment_button(ctx, rect, actions);
    draw_text(
        &clipped(&ctx.session.crew_briefing_label(ctx.data), 34),
        rect.x,
        rect.y - 5.0,
        9.0,
        crew_color(ctx.session.crew_role()),
    );
}

pub fn draw_port_control(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    draw_assignment_button(
        ctx,
        Rect::new(
            width - 126.0,
            crate::ui::port_panel::HEADER_HEIGHT + 108.0,
            100.0,
            28.0,
        ),
        actions,
    );
    draw_rest_control(
        ctx,
        Rect::new(
            width - 238.0,
            crate::ui::port_panel::HEADER_HEIGHT + 108.0,
            100.0,
            28.0,
        ),
        actions,
    );
}

fn draw_assignment_button(ctx: &UiContext<'_>, rect: Rect, actions: &mut Vec<UiAction>) {
    if button(
        ctx,
        rect,
        &crew_button_label(ctx.session.crew_role()),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleCrew);
    }
}

fn draw_rest_control(ctx: &UiContext<'_>, rect: Rect, actions: &mut Vec<UiAction>) {
    let fatigue = ctx.session.crew_fatigue();
    let label = rest_button_label(ctx.session.crew_readiness());
    if button(
        ctx,
        rect,
        &label,
        fatigue > 0,
        if fatigue > 0 {
            ButtonTone::Warning
        } else {
            ButtonTone::Secondary
        },
    ) {
        actions.push(UiAction::RestCrew);
    }
}

fn crew_button_label(role: crate::state::CrewRole) -> String {
    format!("CREW  //  {}", role.short_label())
}

fn rest_button_label(readiness: u8) -> String {
    if readiness >= 100 {
        "RESTED".to_owned()
    } else {
        format!("REST  //  {readiness}%")
    }
}

fn crew_color(role: CrewRole) -> Color {
    if role == CrewRole::Deckhand {
        visual_theme::text_dim()
    } else {
        visual_theme::cyan()
    }
}

#[cfg(test)]
mod tests;
