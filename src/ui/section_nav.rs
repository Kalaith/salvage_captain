//! Connected wreck-section navigation and compact recovery markers.

use super::{button, clipped, hazard_label, visual_theme, UiAction, UiContext};
use crate::state::workspace::WorkspaceConditionStatus;
use macroquad::prelude::*;
use macroquad_toolkit::ui::ButtonTone;

#[cfg(test)]
mod tests;

pub fn draw_section_nav(
    ctx: &UiContext<'_>,
    site: &crate::data::SiteData,
    actions: &mut Vec<UiAction>,
) {
    let current = ctx
        .session
        .expedition
        .as_ref()
        .map(|expedition| expedition.workspace_section.as_str())
        .unwrap_or_default();
    let current_section = ctx.session.workspace_section(ctx.data).ok();
    let extraction_active = ctx.workspace_extraction_target.is_some();
    let shifting = ctx.workspace_camera_shift < 1.0;
    let mut x = 408.0;
    for section in &site.sections {
        let rect = Rect::new(x, 92.0, 150.0, 34.0);
        let can_visit = section.id == current
            || current_section.is_some_and(|current| {
                current
                    .connected_sections
                    .iter()
                    .any(|neighbor| neighbor == &section.id)
            });
        let capability_ready = section
            .required_capability
            .as_deref()
            .is_none_or(|capability| ctx.session.has_capability(capability, ctx.data));
        let can_visit = !extraction_active && !shifting && can_visit && capability_ready;
        let visited = ctx
            .session
            .site_progress
            .get(&site.id)
            .is_some_and(|progress| {
                progress
                    .discovered_sections
                    .iter()
                    .any(|section_id| section_id == &section.id)
            });
        let label = if extraction_active {
            if section.id == current {
                "WORKING".to_owned()
            } else {
                "BUSY".to_owned()
            }
        } else if shifting {
            if section.id == current {
                "SHIFTING".to_owned()
            } else {
                "WAIT".to_owned()
            }
        } else if !capability_ready {
            format!(
                "NEEDS {}",
                clipped(
                    &hazard_label(section.required_capability.as_deref().unwrap_or_default()),
                    12
                )
            )
        } else if can_visit {
            section.display_name.to_uppercase()
        } else {
            format!("LOCKED // {}", clipped(&section.display_name, 11))
        };
        if button(
            ctx,
            rect,
            &label,
            can_visit,
            if section.id == current {
                ButtonTone::Primary
            } else {
                ButtonTone::Secondary
            },
        ) {
            actions.push(UiAction::SelectSection(section.id.clone()));
        }
        draw_hazard_badge(rect, section.hazard_tags.len());
        if site.contract_target.as_deref().is_some_and(|target_id| {
            section
                .candidate_targets
                .iter()
                .any(|candidate| candidate == target_id)
        }) {
            draw_text(
                "OBJ",
                rect.right() - 35.0,
                rect.y - 4.0,
                9.0,
                visual_theme::amber(),
            );
        }
        let stabilized_count = stabilized_target_count(
            &section.candidate_targets,
            ctx.session
                .expedition
                .as_ref()
                .map(|expedition| expedition.stabilized_targets.as_slice()),
        );
        if let Some(status) = ctx
            .session
            .site_section_condition_status(&site.id, &section.id, ctx.data)
            .filter(|status| status.discovered)
        {
            draw_section_recovery(rect, status, stabilized_count);
        } else if visited {
            draw_text(
                "VISITED",
                rect.x + rect.w - 56.0,
                rect.y - 4.0,
                9.0,
                visual_theme::safe(),
            );
        }
        x += 158.0;
    }
}

fn draw_section_recovery(rect: Rect, status: WorkspaceConditionStatus, stabilized_count: usize) {
    let lock_suffix = if stabilized_count == 0 {
        String::new()
    } else {
        format!(" // LOCK {}", stabilized_count)
    };
    draw_text(
        format!(
            "RECOV {}/{}{}",
            status.recovered_targets, status.total_targets, lock_suffix
        ),
        rect.x + 4.0,
        rect.y - 4.0,
        9.0,
        match status.label() {
            "CRITICAL" => visual_theme::warning(),
            "STRESSED" => visual_theme::amber(),
            _ => visual_theme::safe(),
        },
    );
}

fn stabilized_target_count(candidates: &[String], stabilized_targets: Option<&[String]>) -> usize {
    stabilized_targets.map_or(0, |stabilized| {
        stabilized
            .iter()
            .filter(|target_id| candidates.iter().any(|candidate| candidate == *target_id))
            .count()
    })
}

fn draw_hazard_badge(rect: Rect, count: usize) {
    if count == 0 {
        return;
    }
    let center = vec2(rect.right() - 13.0, rect.y + 10.0);
    draw_circle(center.x, center.y, 8.0, visual_theme::warning());
    draw_text(
        format!("{:02}", count.min(99)),
        center.x - 6.0,
        center.y + 3.0,
        8.0,
        visual_theme::panel(),
    );
}
