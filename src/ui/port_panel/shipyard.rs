//! Yard drawers leave the action dock and the workboat visible.

use super::*;
mod equipment;

pub(super) fn draw_shipyard(ctx: &UiContext<'_>, console: Rect, actions: &mut Vec<UiAction>) {
    visual_theme::surface(console);
    let copy = &ctx.data.port_ui;
    let title = match ctx.port_tab {
        PortTab::Hangar => return,
        PortTab::Service => &copy.service,
        PortTab::Equipment => &copy.equipment,
        PortTab::Crew => &copy.crew,
    };
    visual_theme::body(
        title,
        Rect::new(console.x + 18.0, console.y + 10.0, 260.0, 30.0),
        26.0,
        visual_theme::text(),
    );
    if button(ctx, close_rect(), &copy.close, true, ButtonTone::Secondary) {
        actions.push(UiAction::SelectPortTab(PortTab::Hangar));
    }
    let frame = content_rect();
    match ctx.port_tab {
        PortTab::Hangar => {}
        PortTab::Service => crate::ui::service_panel::draw_port_services(ctx, frame, actions),
        PortTab::Equipment if ctx.port_loadouts_open => {
            crate::ui::loadout_panel::draw_port_loadouts(ctx, frame, actions)
        }
        PortTab::Equipment => equipment::draw(ctx, frame, actions),
        PortTab::Crew => crate::ui::crew_panel::draw_port_control(ctx, frame, actions),
    }
}
