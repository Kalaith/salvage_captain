//! Compact crew assignment control used during mission briefing.

use super::*;
use crate::state::CrewRole;
use crate::ui::visual_theme;

pub fn draw_briefing_control(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let rect = Rect::new(1008.0, 158.0, 216.0, 30.0);
    if button(
        ctx,
        rect,
        &format!("CREW  //  {}", ctx.session.crew_role().short_label()),
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CycleCrew);
    }
    draw_text(
        &clipped(&ctx.session.crew_briefing_label(ctx.data), 34),
        rect.x,
        rect.y - 5.0,
        9.0,
        crew_color(ctx.session.crew_role()),
    );
}

fn crew_color(role: CrewRole) -> Color {
    if role == CrewRole::Deckhand {
        visual_theme::text_dim()
    } else {
        visual_theme::cyan()
    }
}
