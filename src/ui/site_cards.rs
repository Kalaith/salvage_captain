//! Site cards expose travel cost, condition, reward hints, and danger.

use super::*;

pub fn draw_site_selection(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel_title(
        Rect::new(24.0, 112.0, 1232.0, 494.0),
        state::site_selection::TITLE,
    );
    draw_text(
        "Fuel cost includes the nav module discount. Required fuel includes a safe-return buffer.",
        54.0,
        162.0,
        16.0,
        dark::TEXT_DIM,
    );
    for (index, site) in ctx.data.ordered_sites().into_iter().enumerate() {
        let x = 48.0 + index as f32 * 397.0;
        let rect = Rect::new(x, 192.0, 370.0, 360.0);
        let accent = danger_color(site.danger);
        panel(rect, Color::new(0.09, 0.12, 0.16, 1.0));
        draw_rectangle(x, rect.y, 6.0, rect.h, accent);
        draw_text(&site.display_name, x + 22.0, 232.0, 25.0, dark::TEXT_BRIGHT);
        draw_text(&site.category, x + 22.0, 258.0, 15.0, accent);
        draw_text(
            clipped(&site.description, 44),
            x + 22.0,
            294.0,
            15.0,
            dark::TEXT,
        );
        draw_text(
            format!("DANGER  {:02}%", site.danger),
            x + 22.0,
            342.0,
            18.0,
            accent,
        );
        let cost = ctx
            .session
            .effective_fuel_cost(&site.id, ctx.data)
            .unwrap_or(site.fuel_cost);
        let required = ctx
            .session
            .departure_fuel_required(&site.id, ctx.data)
            .unwrap_or(cost);
        draw_text(
            format!("FUEL  {} trip  /  {} required", cost, required),
            x + 22.0,
            372.0,
            16.0,
            dark::TEXT_BRIGHT,
        );
        let progress = ctx
            .session
            .site_progress
            .get(&site.id)
            .map_or(site.condition, |value| value.condition);
        let visits = ctx
            .session
            .site_progress
            .get(&site.id)
            .map_or(0, |value| value.visits);
        draw_text(
            format!("CONDITION  {}%   VISITS {}", progress, visits),
            x + 22.0,
            400.0,
            15.0,
            dark::TEXT_DIM,
        );
        draw_text(
            format!("KNOWN: {}", site.known_reward),
            x + 22.0,
            430.0,
            14.0,
            dark::TEXT,
        );
        let can_depart = ctx.session.can_depart(&site.id, ctx.data);
        if button(
            ctx,
            Rect::new(x + 22.0, 468.0, 326.0, 48.0),
            if can_depart {
                "DEPART"
            } else {
                "NOT ENOUGH FUEL"
            },
            can_depart,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::Depart(site.id.clone()));
        }
    }
    draw_text(
        "A site choice is a risk choice: danger is previewed, but the exact setback is seeded at departure.",
        54.0,
        584.0,
        15.0,
        dark::TEXT_DIM,
    );
}
