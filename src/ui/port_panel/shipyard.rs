//! Tabbed preparation rail with a persistent departure control.

use super::*;
mod equipment;

pub(super) fn draw_shipyard(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    visual_theme::surface(console);
    let copy = &ctx.data.port_ui;
    visual_theme::body(
        &copy.title,
        Rect::new(console.x + 18.0, 72.0, 220.0, 34.0),
        28.0,
        visual_theme::text(),
    );
    text_at(
        ctx.session.salvage_standing().label(),
        Rect::new(console.right() - 164.0, 78.0, 146.0, 28.0),
        visual_theme::cyan(),
    );
    for (tab, label) in [
        (PortTab::Service, &copy.service),
        (PortTab::Equipment, &copy.equipment),
        (PortTab::Crew, &copy.crew),
    ] {
        let rect = tab_rect(tab);
        if button(ctx, rect, label, true, ButtonTone::Secondary) {
            actions.push(UiAction::SelectPortTab(tab));
        }
        if ctx.port_tab == tab {
            draw_rectangle(
                rect.x,
                rect.bottom() - 3.0,
                rect.w,
                3.0,
                visual_theme::amber(),
            );
        }
    }
    let frame = content_rect();
    match ctx.port_tab {
        PortTab::Service => crate::ui::service_panel::draw_port_services(ctx, frame, actions),
        PortTab::Equipment if ctx.port_loadouts_open => {
            crate::ui::loadout_panel::draw_port_loadouts(ctx, frame, actions)
        }
        PortTab::Equipment => equipment::draw(ctx, frame, actions),
        PortTab::Crew => crate::ui::crew_panel::draw_port_control(ctx, frame, actions),
    }
    draw_line(
        console.x + 18.0,
        644.0,
        console.right() - 18.0,
        644.0,
        1.0,
        visual_theme::structure_light(),
    );
    if button(
        ctx,
        departure_rect(),
        &copy.depart,
        true,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::GoToSites);
    }
}
