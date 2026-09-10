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
    draw_section_nav(ctx, site, actions);
    if let Some(section) = section {
        draw_text(
            format!(
                "HAZARDS: {}",
                if section.hazard_tags.is_empty() {
                    "NONE LOGGED".to_owned()
                } else {
                    section.hazard_tags.join(" / ").to_uppercase()
                }
            ),
            layout.viewport.x,
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
        layout.viewport.x,
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
    let mut x = 414.0;
    for section in &site.sections {
        let rect = Rect::new(x, 112.0, 150.0, 34.0);
        if button(
            ctx,
            rect,
            &section.display_name.to_uppercase(),
            true,
            if section.id == current {
                ButtonTone::Primary
            } else {
                ButtonTone::Secondary
            },
        ) {
            actions.push(UiAction::SelectSection(section.id.clone()));
        }
        x += 158.0;
    }
}

fn draw_target_selection(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    if !ctx.workspace_scanned || ctx.workspace_extraction_target.is_some() {
        return;
    }
    let Some(expedition) = &ctx.session.expedition else {
        return;
    };
    if !ctx.pointer.released {
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
    let can_scan = !ctx.workspace_scanned
        && ctx.workspace_scan_progress <= 0.0
        && ctx.workspace_elapsed >= 0.8
        && ctx.workspace_extraction_target.is_none();
    if button(
        ctx,
        Rect::new(
            layout.command.x + 16.0,
            layout.command.y + 38.0,
            150.0,
            48.0,
        ),
        if ctx.workspace_scan_progress > 0.0 {
            "SCANNING"
        } else {
            "SCAN"
        },
        can_scan,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Scan);
    }
    if button(
        ctx,
        Rect::new(
            layout.command.x + 178.0,
            layout.command.y + 38.0,
            166.0,
            48.0,
        ),
        "RETURN TO PACKING",
        ctx.workspace_extraction_target.is_none(),
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::ReturnFromWorkspace);
    }
    if ctx.workspace_scan_progress > 0.0 {
        draw_text(
            "Pulse crossing the hull...",
            layout.command.x + 16.0,
            layout.command.y + 100.0,
            12.0,
            visual_theme::cyan(),
        );
    } else if !ctx.workspace_scanned {
        draw_text(
            "SCAN AVAILABLE",
            layout.command.x + 180.0,
            layout.command.y + 28.0,
            12.0,
            visual_theme::cyan(),
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
        visual_theme::safe(),
    );
    draw_text(
        "The mount is now visibly empty.",
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
    if matches!(
        ctx.workspace_extraction_phase,
        Some(ExtractionPhase::Strain | ExtractionPhase::Separation)
    ) {
        for index in 0..4 {
            let t = index as f32 / 4.0;
            let x = lerp(start.x, end.x, t);
            let y = lerp(start.y, end.y, t)
                - (ctx.workspace_extraction_progress * 80.0 + index as f32).sin() * 8.0;
            draw_circle(x, y, 3.0, visual_theme::amber());
        }
    }
}
