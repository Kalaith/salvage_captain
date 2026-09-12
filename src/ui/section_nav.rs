//! Touch navigation between connected wreck sections.

use super::*;

pub fn draw_section_nav(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    actions: &mut Vec<UiAction>,
) {
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    let current = ctx.session.workspace_section(ctx.data).ok();
    let width = (760.0 / site.sections.len().max(1) as f32).min(250.0);
    for (index, section) in site.sections.iter().enumerate() {
        let rect = Rect::new(24.0 + index as f32 * (width + 10.0), 98.0, width, 44.0);
        let connected = section.id == expedition.workspace_section
            || current.is_some_and(|value| value.connected_sections.contains(&section.id));
        let capable = section
            .required_capability
            .as_deref()
            .is_none_or(|capability| ctx.session.has_capability(capability, ctx.data));
        let enabled = connected
            && capable
            && ctx.workspace_extraction_target.is_none()
            && ctx.workspace_camera_shift >= 1.0
            && ctx.workspace_scan_progress <= 0.0;
        if button(
            ctx,
            rect,
            &section.display_name,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::SelectSection(section.id.clone()));
        }
        if section.id == expedition.workspace_section {
            draw_rectangle(
                rect.x,
                rect.bottom() + 2.0,
                rect.w,
                3.0,
                visual_theme::cyan(),
            );
        }
    }
    if ctx.session.crew_fatigue_danger_delta() > 0 {
        visual_theme::body(
            &ctx.data.salvage_ui.crew_tired,
            Rect::new(1020.0, 108.0, 220.0, 28.0),
            20.0,
            visual_theme::warning(),
        );
    }
}
