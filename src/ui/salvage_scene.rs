//! The close salvage workspace: scan, select, extract, and return.

use super::extraction_panel;
use super::scan_overlay;
use super::scene_layout::{self, SalvageLayout};
use super::ship_visual;
use super::visual_theme;
use super::wreck_visual;
use super::*;
use crate::state::workspace::ExtractionPhase;
use macroquad_toolkit::math::lerp;

pub(crate) const SECTION_SHIFT_SECONDS: f32 = 0.75;
pub(crate) const SECTION_ARRIVAL_FLASH_SECONDS: f32 = 0.6;
pub(crate) const SECTION_SETTLED_PROMPT: &str = "Section settled. Tap SCAN to reveal this frame.";

#[cfg(test)]
mod tests;

pub(crate) fn section_switch_prompt(message: &str, moving_camera: bool) -> String {
    if moving_camera {
        format!("{message} Camera shift underway; wait for ARRIVAL, then tap SCAN.")
    } else {
        format!("{message} Tap SCAN to reveal this section.")
    }
}

pub(crate) fn section_arrival_ready(camera_shift: f32, workspace_elapsed: f32) -> bool {
    camera_shift >= 1.0 && workspace_elapsed >= 0.8
}

fn section_shift_ease(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    progress * progress * (3.0 - 2.0 * progress)
}

pub fn draw_salvage_workspace(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let layout = scene_layout::salvage_layout();
    let Some(expedition) = &ctx.session.expedition else {
        draw_text(
            "NO ACTIVE SALVAGE RUN",
            54.0,
            176.0,
            24.0,
            visual_theme::warning(),
        );
        return;
    };
    let Some(site) = ctx.data.sites.get(&expedition.site_id) else {
        return;
    };
    let section = ctx.session.workspace_section(ctx.data).ok();
    let condition = ctx
        .session
        .workspace_condition_status(ctx.data)
        .unwrap_or_else(|_| crate::state::workspace::WorkspaceConditionStatus::unknown());
    draw_section_nav(ctx, site, actions);
    if let Some(section) = section {
        let hazard_readout = if section.hazard_tags.is_empty() {
            "HAZARDS: NONE LOGGED".to_owned()
        } else {
            format!(
                "HAZARDS {:02} // {}",
                section.hazard_tags.len(),
                section
                    .hazard_tags
                    .iter()
                    .map(|tag| hazard_label(tag))
                    .collect::<Vec<_>>()
                    .join(" / ")
            )
        };
        draw_text(
            hazard_readout,
            layout.viewport.x + 30.0,
            layout.viewport.y + 42.0,
            12.0,
            if section.hazard_tags.is_empty() {
                visual_theme::text_dim()
            } else {
                visual_theme::warning()
            },
        );
    }
    draw_text(
        format!(
            "{}  /  {}",
            site.display_name.to_uppercase(),
            section.map_or("UNKNOWN SECTION", |value| value.display_name.as_str())
        ),
        layout.viewport.x + 30.0,
        layout.viewport.y + 24.0,
        15.0,
        visual_theme::text_dim(),
    );
    draw_debris(ctx.workspace_elapsed);
    wreck_visual::draw_wreck(
        layout,
        site,
        ctx.session,
        ctx.data,
        ctx.workspace_elapsed,
        ctx.workspace_scanned,
        ctx.workspace_selected_target,
        ctx.workspace_extraction_target,
        ctx.workspace_extraction_progress,
        section.map_or(&[], |value| value.candidate_targets.as_slice()),
        section.map_or(&[], |value| value.hazard_tags.as_slice()),
        condition,
    );
    ship_visual::draw_ship(
        layout.ship,
        ctx.session,
        ctx.data,
        ctx.workspace_elapsed,
        ctx.workspace_selected_target.is_some(),
    );
    if let Some(target_id) = ctx.workspace_extraction_target {
        draw_tractor_beam(ctx, layout, target_id);
    }
    scan_overlay::draw_scan_overlay(
        layout,
        ctx.workspace_elapsed,
        ctx.workspace_scan_progress,
        ctx.workspace_scanned,
        &expedition.revealed_targets,
    );
    draw_section_shift(ctx, layout);
    draw_target_selection(ctx, layout, actions);
    draw_command_panel(ctx, layout, actions);
    extraction_panel::draw_target_panel(ctx, layout, actions);
    draw_notice(ctx);
}

fn draw_section_nav(
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
        if let Some(status) = ctx
            .session
            .site_section_condition_status(&site.id, &section.id, ctx.data)
            .filter(|status| status.discovered)
        {
            draw_section_recovery(rect, status);
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

fn draw_section_recovery(rect: Rect, status: crate::state::workspace::WorkspaceConditionStatus) {
    draw_text(
        format!(
            "RECOV {}/{}",
            status.recovered_targets, status.total_targets
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

fn draw_section_shift(ctx: &UiContext<'_>, layout: SalvageLayout) {
    if ctx.workspace_camera_shift >= 1.0 {
        if ctx.workspace_arrival_flash <= 0.0 {
            return;
        }
        let intensity = ctx.workspace_arrival_flash.clamp(0.0, 1.0);
        draw_rectangle_lines(
            layout.wreck.x - 7.0,
            layout.wreck.y - 7.0,
            layout.wreck.w + 14.0,
            layout.wreck.h + 14.0,
            3.0,
            visual_theme::with_alpha(visual_theme::safe(), intensity),
        );
        draw_circle_lines(
            layout.wreck.center().x,
            layout.wreck.center().y,
            44.0 + (1.0 - intensity) * 26.0,
            2.0,
            visual_theme::with_alpha(visual_theme::safe(), intensity),
        );
        draw_text(
            if section_arrival_ready(ctx.workspace_camera_shift, ctx.workspace_elapsed) {
                "SECTION LOCKED // SCAN READY"
            } else {
                "SECTION LOCKED // HULL SETTLING"
            },
            layout.wreck.x + 18.0,
            516.0,
            12.0,
            visual_theme::safe(),
        );
        return;
    }
    let progress = ctx.workspace_camera_shift.clamp(0.0, 1.0);
    let sweep_x = layout.wreck.x - 42.0 + section_shift_ease(progress) * (layout.wreck.w + 84.0);
    draw_rectangle(
        layout.ship.x - 24.0,
        layout.ship.y - 26.0,
        layout.wreck.right() - layout.ship.x + 48.0,
        layout.wreck.bottom() - layout.ship.y + 44.0,
        visual_theme::with_alpha(visual_theme::space(), 0.22),
    );
    draw_line(
        sweep_x,
        layout.wreck.y - 12.0,
        sweep_x,
        layout.wreck.bottom() + 12.0,
        3.0,
        visual_theme::with_alpha(visual_theme::cyan(), 0.82),
    );
    draw_text(
        "CAMERA SHIFT // FOLLOWING WORKBOAT",
        layout.wreck.x + 18.0,
        516.0,
        12.0,
        visual_theme::cyan(),
    );
    draw_text(
        format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
        layout.wreck.right() - 116.0,
        516.0,
        11.0,
        visual_theme::text_dim(),
    );
}

fn draw_target_selection(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    if !ctx.workspace_scanned
        || ctx.workspace_extraction_target.is_some()
        || ctx.workspace_camera_shift < 1.0
    {
        return;
    }
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    if !ctx.interaction_enabled || !ctx.pointer.released {
        return;
    }
    for target_id in &expedition.revealed_targets {
        let Some(rect) = layout.target_rect(target_id) else {
            continue;
        };
        if ctx.pointer.released_on(rect) && !ctx.session.target_is_removed(target_id) {
            actions.push(UiAction::SelectTarget(target_id.clone()));
            break;
        }
    }
}

fn draw_command_panel(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    panel(layout.command, visual_theme::panel());
    draw_text(
        "OPERATOR CONSOLE",
        layout.command.x + 16.0,
        layout.command.y + 24.0,
        14.0,
        visual_theme::text_dim(),
    );
    if let Some(expedition) = &ctx.session.expedition {
        if let Some(objective) = ctx
            .session
            .contract_objective_status(&expedition.site_id, ctx.data)
        {
            let target_name = ctx
                .data
                .salvage_objects
                .get(&objective.target_id)
                .map_or(objective.target_id.clone(), |target| {
                    if target.workspace_name.is_empty() {
                        target.display_name.clone()
                    } else {
                        target.workspace_name.clone()
                    }
                })
                .to_uppercase();
            draw_text(
                format!(
                    "OBJ {} // {}",
                    objective.state.label(),
                    clipped(&target_name, 13)
                ),
                layout.command.x + 190.0,
                layout.command.y + 14.0,
                10.0,
                match objective.state {
                    crate::state::contracts::ContractObjectiveState::Failed => {
                        visual_theme::warning()
                    }
                    crate::state::contracts::ContractObjectiveState::Complete => {
                        visual_theme::safe()
                    }
                    crate::state::contracts::ContractObjectiveState::Open
                    | crate::state::contracts::ContractObjectiveState::Recovered => {
                        visual_theme::amber()
                    }
                },
            );
        }
    }
    let can_scan = !ctx.workspace_scanned
        && ctx.workspace_scan_progress <= 0.0
        && section_arrival_ready(ctx.workspace_camera_shift, ctx.workspace_elapsed)
        && ctx.workspace_extraction_target.is_none()
        && ctx
            .session
            .workspace_energy()
            .is_some_and(|(remaining, _)| remaining >= ctx.data.config.workspace_scan_energy_cost);
    let scan_power_available = ctx
        .session
        .workspace_energy()
        .is_some_and(|(remaining, _)| remaining >= ctx.data.config.workspace_scan_energy_cost);
    if button(
        ctx,
        Rect::new(
            layout.command.x + 16.0,
            layout.command.y + 38.0,
            150.0,
            48.0,
        ),
        if ctx.workspace_camera_shift < 1.0 {
            "SHIFTING"
        } else if ctx.workspace_scan_progress > 0.0 {
            "SCANNING"
        } else if !scan_power_available {
            "NO POWER"
        } else {
            "SCAN"
        },
        can_scan,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Scan);
    }
    if ctx.workspace_extraction_target.is_some() {
        if ctx.workspace_extraction_progress < 1.0
            && button(
                ctx,
                Rect::new(
                    layout.command.x + 178.0,
                    layout.command.y + 38.0,
                    166.0,
                    48.0,
                ),
                "CANCEL EXTRACTION",
                true,
                ButtonTone::Warning,
            )
        {
            actions.push(UiAction::CancelExtraction);
        }
        if ctx.workspace_extraction_progress >= 1.0
            && button(
                ctx,
                Rect::new(
                    layout.command.x + 178.0,
                    layout.command.y + 38.0,
                    166.0,
                    48.0,
                ),
                "RETURN TO PACKING",
                true,
                ButtonTone::Positive,
            )
        {
            actions.push(UiAction::ReturnFromWorkspace);
        }
    } else if button(
        ctx,
        Rect::new(
            layout.command.x + 178.0,
            layout.command.y + 38.0,
            166.0,
            48.0,
        ),
        "RETURN TO PACKING",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::ReturnFromWorkspace);
    }
    if ctx.workspace_camera_shift < 1.0 {
        draw_text(
            "Following the workboat...",
            layout.command.x + 16.0,
            layout.command.y + 100.0,
            12.0,
            visual_theme::cyan(),
        );
    } else if ctx.workspace_scan_progress > 0.0 {
        draw_text(
            "Pulse crossing the hull...",
            layout.command.x + 16.0,
            layout.command.y + 100.0,
            12.0,
            visual_theme::cyan(),
        );
    }
    if let Some(expedition) = &ctx.session.expedition {
        let recovery = ctx
            .session
            .site_recovery_status(&expedition.site_id, ctx.data);
        let scan_suffix = if !ctx.workspace_scanned
            && section_arrival_ready(ctx.workspace_camera_shift, ctx.workspace_elapsed)
            && ctx.workspace_scan_progress <= 0.0
        {
            " // SCAN READY"
        } else {
            ""
        };
        draw_text(
            format!(
                "RECOVERY {}/{}  //  LEFT {}{scan_suffix}",
                recovery.recovered_targets, recovery.total_targets, recovery.remaining_targets
            ),
            layout.command.x + 180.0,
            layout.command.y + 28.0,
            12.0,
            if scan_suffix.is_empty() {
                visual_theme::text_dim()
            } else {
                visual_theme::cyan()
            },
        );
        draw_text(
            format!(
                "FRAMES {}/{}  //  EXPLORED {:02}%",
                recovery.explored_sections, recovery.total_sections, recovery.exploration_percent
            ),
            layout.command.x + 180.0,
            layout.command.y + 44.0,
            10.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_notice(ctx: &UiContext<'_>) {
    if ctx.workspace_notice.is_empty() || ctx.workspace_notice_timer <= 0.0 {
        return;
    }
    let rect = Rect::new(430.0, 526.0, 504.0, 62.0);
    panel(rect, visual_theme::with_alpha(visual_theme::panel(), 0.96));
    draw_text(
        ctx.workspace_notice,
        rect.x + 16.0,
        rect.y + 25.0,
        15.0,
        if ctx.workspace_notice_warning {
            visual_theme::warning()
        } else {
            visual_theme::safe()
        },
    );
    let condition_line = ctx
        .session
        .workspace_condition_status(ctx.data)
        .map_or_else(
            |_| {
                if ctx.workspace_notice_warning {
                    "Hazard result is final; inspect the hull before the next pull.".to_owned()
                } else {
                    "The mount is now visibly empty.".to_owned()
                }
            },
            |condition| {
                format!(
                    "Section {:02}% // {}{}",
                    condition.section_condition,
                    condition.label(),
                    if ctx.workspace_notice_warning {
                        " // HAZARD FINAL"
                    } else {
                        ""
                    }
                )
            },
        );
    draw_text(
        condition_line,
        rect.x + 16.0,
        rect.y + 47.0,
        12.0,
        visual_theme::text_dim(),
    );
}

fn draw_debris(elapsed: f32) {
    for index in 0..8 {
        let x = 350.0 + index as f32 * 75.0 + (elapsed * (0.4 + index as f32 * 0.03)).sin() * 12.0;
        let y = 188.0 + (index * 53) as f32 + (elapsed * 0.3).cos() * 8.0;
        draw_rectangle(
            x,
            y,
            4.0 + (index % 3) as f32,
            3.0,
            visual_theme::with_alpha(visual_theme::structure_light(), 0.5),
        );
    }
}

fn draw_tractor_beam(ctx: &UiContext<'_>, layout: SalvageLayout, target_id: &str) {
    let Some(target_rect) = layout.target_rect(target_id) else {
        return;
    };
    let start = ship_visual::emitter_point(layout.ship);
    let mut end = target_rect.center();
    let progress = ctx.workspace_extraction_progress;
    if progress > 0.68 {
        let retrieval = ((progress - 0.68) / 0.32).clamp(0.0, 1.0);
        let control = vec2((start.x + end.x) * 0.5, end.y - 120.0);
        let point_a = vec2(
            lerp(end.x, control.x, retrieval),
            lerp(end.y, control.y, retrieval),
        );
        end = vec2(
            lerp(point_a.x, start.x, retrieval),
            lerp(point_a.y, start.y, retrieval),
        );
        draw_rectangle(
            end.x - 14.0,
            end.y - 10.0,
            28.0,
            20.0,
            visual_theme::amber(),
        );
    }
    let bend = vec2((start.x + end.x) * 0.5, (start.y + end.y) * 0.5 - 36.0);
    draw_line(
        start.x,
        start.y,
        bend.x,
        bend.y,
        14.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.85),
    );
    draw_line(
        bend.x,
        bend.y,
        end.x,
        end.y,
        14.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.85),
    );
    draw_line(start.x, start.y, bend.x, bend.y, 4.0, visual_theme::cyan());
    draw_line(bend.x, bend.y, end.x, end.y, 4.0, visual_theme::cyan());
    draw_extraction_effects(
        start,
        bend,
        end,
        target_rect,
        ctx.workspace_extraction_phase
            .unwrap_or(ExtractionPhase::Alignment),
        ctx.workspace_extraction_progress,
        ctx.workspace_elapsed,
    );
}

fn draw_extraction_effects(
    start: Vec2,
    bend: Vec2,
    end: Vec2,
    target_rect: Rect,
    phase: ExtractionPhase,
    progress: f32,
    elapsed: f32,
) {
    match phase {
        ExtractionPhase::Alignment => {
            let pulse = 18.0 + (elapsed * 4.0).sin().abs() * 10.0;
            draw_circle_lines(end.x, end.y, pulse, 2.0, visual_theme::cyan());
            draw_line(
                end.x - pulse - 8.0,
                end.y,
                end.x - pulse,
                end.y,
                2.0,
                visual_theme::cyan(),
            );
            draw_line(
                end.x + pulse,
                end.y,
                end.x + pulse + 8.0,
                end.y,
                2.0,
                visual_theme::cyan(),
            );
        }
        ExtractionPhase::Connection => {
            draw_circle_lines(end.x, end.y, 22.0, 2.0, visual_theme::cyan());
            for index in 1..4 {
                let point = beam_point(start, bend, end, index as f32 / 4.0);
                draw_circle(point.x, point.y, 4.0, visual_theme::cyan());
            }
        }
        ExtractionPhase::Strain => {
            draw_circle_lines(
                end.x,
                end.y,
                target_rect.w.min(target_rect.h) * 0.42,
                3.0,
                visual_theme::warning(),
            );
            for index in 0..8 {
                let angle = elapsed * 3.0 + index as f32 * 0.78;
                let inner = target_rect.w.min(target_rect.h) * 0.28;
                let outer = inner + 10.0 + (elapsed * 8.0 + index as f32).sin().abs() * 12.0;
                draw_line(
                    end.x + angle.cos() * inner,
                    end.y + angle.sin() * inner,
                    end.x + angle.cos() * outer,
                    end.y + angle.sin() * outer,
                    2.0,
                    visual_theme::amber(),
                );
            }
        }
        ExtractionPhase::Separation => {
            let burst = (elapsed * 4.0).sin().abs();
            for index in 0..6 {
                let angle = index as f32 * 1.05 + elapsed * 0.6;
                let inner = 12.0 + burst * 6.0;
                let outer = 28.0 + burst * 18.0 + index as f32 * 2.0;
                draw_line(
                    end.x + angle.cos() * inner,
                    end.y + angle.sin() * inner,
                    end.x + angle.cos() * outer,
                    end.y + angle.sin() * outer,
                    2.0,
                    visual_theme::warning(),
                );
                draw_circle(
                    end.x + angle.cos() * outer,
                    end.y + angle.sin() * outer,
                    2.5,
                    visual_theme::amber(),
                );
            }
            draw_line(
                target_rect.x + 12.0,
                target_rect.center().y,
                target_rect.right() - 12.0,
                target_rect.center().y,
                2.0,
                visual_theme::warning(),
            );
        }
        ExtractionPhase::Retrieval => {
            for index in 0..6 {
                let t = (elapsed * 1.8 + index as f32 * 0.17).fract();
                let point = beam_point(start, bend, end, t);
                draw_circle(
                    point.x,
                    point.y,
                    2.0 + (elapsed * 6.0 + index as f32).sin().abs() * 2.0,
                    visual_theme::amber(),
                );
            }
        }
        ExtractionPhase::Capture => {
            let pulse = 15.0 + (progress * 32.0).sin().abs() * 8.0;
            draw_circle_lines(start.x, start.y, pulse, 2.0, visual_theme::safe());
            draw_line(
                start.x - 18.0,
                start.y - 12.0,
                start.x + 18.0,
                start.y - 12.0,
                3.0,
                visual_theme::amber(),
            );
            draw_line(
                start.x - 18.0,
                start.y + 12.0,
                start.x + 18.0,
                start.y + 12.0,
                3.0,
                visual_theme::amber(),
            );
        }
    }
}

fn beam_point(start: Vec2, bend: Vec2, end: Vec2, progress: f32) -> Vec2 {
    if progress <= 0.5 {
        let t = progress * 2.0;
        vec2(lerp(start.x, bend.x, t), lerp(start.y, bend.y, t))
    } else {
        let t = (progress - 0.5) * 2.0;
        vec2(lerp(bend.x, end.x, t), lerp(bend.y, end.y, t))
    }
}
