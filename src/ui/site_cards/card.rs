//! Quiet cards keep only the information needed to compare wrecks.

use super::*;

pub(super) fn draw(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
    rect: Rect,
    selected: bool,
) {
    let copy = &ctx.data.selection_ui;
    visual_theme::surface(rect);
    let accent = if selected {
        visual_theme::cyan()
    } else {
        visual_theme::structure()
    };
    draw_rectangle(rect.x, rect.bottom() - 3.0, rect.w, 3.0, accent);
    if selected {
        draw_rectangle(rect.x, rect.y, rect.w, 3.0, accent);
    }
    draw_illustration(site, rect);
    text(
        &site.display_name,
        rect.x + 18.0,
        rect.y + 150.0,
        266.0,
        35.0,
        28.0,
        visual_theme::text(),
    );
    text(
        &format!(
            "{} {}%",
            copy.danger,
            details::site_departure_danger(site, ctx.session, ctx.data, ctx.voyage_plan)
        ),
        rect.x + 18.0,
        rect.y + 192.0,
        178.0,
        30.0,
        23.0,
        danger_color(details::site_departure_danger(
            site,
            ctx.session,
            ctx.data,
            ctx.voyage_plan,
        )),
    );
    text(
        &format!("{} {}", copy.fuel, fuel_required(ctx, site)),
        rect.x + 208.0,
        rect.y + 192.0,
        180.0,
        30.0,
        23.0,
        if ctx.session.economy.fuel < fuel_required(ctx, site) {
            visual_theme::warning()
        } else {
            visual_theme::text()
        },
    );
    text(
        &site.known_reward,
        rect.x + 18.0,
        rect.y + 228.0,
        366.0,
        33.0,
        21.0,
        visual_theme::text_dim(),
    );
    // The complete card is one touch target; release semantics match toolkit buttons.
    if ctx.interaction_enabled && ctx.pointer.released_on(rect) {
        actions.push(UiAction::WreckSelection(SelectionAction::Select(
            site.id.clone(),
        )));
    }
    text(
        if selected {
            &copy.selected
        } else {
            &copy.select
        },
        rect.x + 274.0,
        rect.y + 158.0,
        118.0,
        25.0,
        18.0,
        if selected {
            accent
        } else {
            visual_theme::text_dim()
        },
    );
}

fn draw_illustration(site: &crate::data::SiteData, rect: Rect) {
    // A dark field, distant stars and loose fragments separate hull depth from the UI.
    for index in 0..18 {
        let x = rect.x + 14.0 + ((index * 83 + 17) % 370) as f32;
        let y = rect.y + 12.0 + ((index * 31) % 124) as f32;
        draw_circle(x, y, 1.0, visual_theme::structure_light());
    }
    let hull = Rect::new(rect.x + 36.0, rect.y + 18.0, 328.0, 122.0);
    wreck_silhouette::draw_wreck(hull, &site.visual_theme);
    draw_rectangle(
        rect.x + 22.0,
        rect.y + 122.0,
        12.0,
        7.0,
        visual_theme::structure(),
    );
}
