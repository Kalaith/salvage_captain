//! Compact crew assignment control used during mission briefing.

use super::*;
use crate::state::crew::MAX_CREW_EXPERIENCE;
use crate::state::{CrewRole, GameSession};
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
    let training_rect = Rect::new(
        width - 350.0,
        crate::ui::port_panel::HEADER_HEIGHT + 108.0,
        100.0,
        28.0,
    );
    let assignment_rect = Rect::new(
        width - 126.0,
        crate::ui::port_panel::HEADER_HEIGHT + 108.0,
        100.0,
        28.0,
    );
    draw_text(
        &crew_expertise_status_label(ctx.session),
        assignment_rect.x,
        assignment_rect.y - 5.0,
        8.0,
        crew_color(ctx.session.crew_role()),
    );
    draw_assignment_button(ctx, assignment_rect, actions);
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
    draw_text(
        &format!("NEXT XP  //  {}", ctx.session.crew_training_label(ctx.data)),
        training_rect.x,
        training_rect.y - 5.0,
        8.0,
        crew_color(ctx.session.crew_role()),
    );
    if button(
        ctx,
        training_rect,
        &ctx.session.crew_training_label(ctx.data),
        ctx.session
            .crew_training_cost(ctx.data)
            .is_some_and(|cost| ctx.session.economy.credits >= cost),
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::TrainCrew);
    }
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

fn crew_expertise_status_label(session: &GameSession) -> String {
    format!(
        "{}  //  XP {}/{}",
        session.crew_expertise_label(),
        session.crew_experience(),
        MAX_CREW_EXPERIENCE
    )
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
