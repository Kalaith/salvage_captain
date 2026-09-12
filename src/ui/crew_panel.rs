//! Crew assignment, readiness and training in a dedicated port view.

use super::*;
use crate::state::crew::MAX_CREW_EXPERIENCE;
use crate::ui::port_panel::text_at;

pub fn draw_port_control(ctx: &UiContext<'_>, frame: Rect, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.port_ui;
    let role = ctx.session.crew_role();
    panel(
        Rect::new(frame.x, frame.y, frame.w, 160.0),
        visual_theme::panel_soft(),
    );
    visual_theme::body(
        role.label(),
        Rect::new(frame.x + 14.0, frame.y + 12.0, frame.w - 28.0, 34.0),
        28.0,
        visual_theme::text(),
    );
    text_at(
        role.description(),
        Rect::new(frame.x + 14.0, frame.y + 54.0, frame.w - 28.0, 54.0),
        visual_theme::text_dim(),
    );
    if button(
        ctx,
        Rect::new(frame.x + 14.0, frame.y + 108.0, frame.w - 28.0, 44.0),
        &copy.assignment,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleCrew);
    }
    text_at(
        &copy.crew_hint,
        Rect::new(frame.x, 340.0, frame.w, 52.0),
        visual_theme::text_dim(),
    );
    text_at(
        &format!("{} {}%", copy.readiness, ctx.session.crew_readiness()),
        Rect::new(frame.x, 406.0, 240.0, 30.0),
        visual_theme::cyan(),
    );
    let fatigue = ctx.session.crew_fatigue();
    if button(
        ctx,
        Rect::new(frame.right() - 168.0, 398.0, 168.0, 44.0),
        if fatigue > 0 {
            &copy.rest
        } else {
            &copy.rested
        },
        fatigue > 0,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::RestCrew);
    }
    text_at(
        &format!(
            "{} / XP {}/{}",
            ctx.session.crew_expertise_label(),
            ctx.session.crew_experience(),
            MAX_CREW_EXPERIENCE
        ),
        Rect::new(frame.x, 462.0, frame.w, 30.0),
        visual_theme::amber(),
    );
    if button(
        ctx,
        Rect::new(frame.x, 502.0, frame.w, 44.0),
        &ctx.session.crew_training_label(ctx.data),
        ctx.session
            .crew_training_cost(ctx.data)
            .is_some_and(|cost| ctx.session.economy.credits >= cost),
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TrainCrew);
    }
    text_at(
        &copy.training_hint,
        Rect::new(frame.x, 564.0, frame.w, 70.0),
        visual_theme::text_dim(),
    );
}
