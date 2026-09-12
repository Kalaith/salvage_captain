//! Command panel and extraction feedback for the salvage workspace.

use super::*;

pub(super) fn draw_command_panel(
    ctx: &UiContext<'_>,
    layout: SalvageLayout,
    actions: &mut Vec<UiAction>,
) {
    visual_theme::surface(layout.command);
    let copy = &ctx.data.salvage_ui;
    visual_theme::body(
        &copy.field,
        Rect::new(layout.command.x + 16.0, layout.command.y + 8.0, 220.0, 24.0),
        18.0,
        visual_theme::text_dim(),
    );
    let can_scan = !ctx.workspace_scanned
        && ctx.workspace_scan_progress <= 0.0
        && section_arrival_ready(ctx.workspace_camera_shift, ctx.workspace_elapsed)
        && ctx.workspace_extraction_target.is_none()
        && ctx
            .session
            .workspace_energy()
            .is_some_and(|(remaining, _)| remaining >= ctx.data.config.workspace_scan_energy_cost);
    power_cycle::draw_command_button(ctx, layout, actions, can_scan);
    let pulling =
        ctx.workspace_extraction_target.is_some() && ctx.workspace_extraction_progress < 1.0;
    if button(
        ctx,
        Rect::new(
            layout.command.x + 178.0,
            layout.command.y + 38.0,
            180.0,
            48.0,
        ),
        if pulling {
            &copy.cancel
        } else {
            &copy.return_to_hold
        },
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(if pulling {
            UiAction::CancelExtraction
        } else {
            UiAction::ReturnFromWorkspace
        });
    }
    if ctx.session.module_stats(ctx.data).drone_support > 0 {
        drone_command::draw_operator_control(ctx, layout, actions);
    } else if let Some(label) = power_cycle::status_label_with_cells(
        ctx.session.can_use_field_power_cell(),
        ctx.session.field_power_cells,
        ctx.session.can_power_cycle_workspace(ctx.data),
        ctx.session
            .expedition
            .as_ref()
            .map_or(0, |expedition| expedition.power_cycles_used),
    ) {
        visual_theme::body(
            &label.replace(" // ", " / "),
            Rect::new(
                layout.command.x + 16.0,
                layout.command.y + 98.0,
                340.0,
                30.0,
            ),
            18.0,
            visual_theme::text_dim(),
        );
    }
}

pub(super) fn draw_notice(ctx: &UiContext<'_>) {
    if ctx.workspace_notice.is_empty() || ctx.workspace_notice_timer <= 0.0 {
        return;
    }
    let rect = Rect::new(430.0, 458.0, 560.0, 74.0);
    visual_theme::surface(rect);
    visual_theme::body(
        ctx.workspace_notice,
        Rect::new(rect.x + 16.0, rect.y + 10.0, rect.w - 32.0, 54.0),
        20.0,
        if ctx.workspace_notice_warning {
            visual_theme::warning()
        } else {
            visual_theme::text()
        },
    );
}

pub(super) fn draw_debris(elapsed: f32) {
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

pub(super) fn draw_tractor_beam(ctx: &UiContext<'_>, layout: SalvageLayout, target_id: &str) {
    let Some(target_rect) = layout.target_rect(target_id) else {
        return;
    };
    let mode = ctx
        .data
        .salvage_objects
        .get(target_id)
        .map(TransferMode::from_target)
        .unwrap_or(TransferMode::InternalCargo);
    let start = transfer_hardware::transfer_point(layout.ship, mode);
    let mut end = target_rect.center();
    let active = ctx.workspace_extraction_target.is_some();
    let beam_light = visual_theme::cyan();
    for layer in (1..=4).rev() {
        draw_circle(
            end.x,
            end.y,
            20.0 + layer as f32 * 15.0,
            visual_theme::with_alpha(beam_light, if active { 0.055 } else { 0.025 }),
        );
    }
    if !active {
        draw_triangle(
            start,
            end + vec2(0.0, -20.0),
            end + vec2(0.0, 20.0),
            visual_theme::with_alpha(beam_light, 0.065),
        );
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            2.0,
            visual_theme::with_alpha(beam_light, 0.65),
        );
        return;
    }
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
    let bend_offset = match mode {
        TransferMode::InternalCargo => 36.0,
        TransferMode::ExternalClamp => 72.0,
        TransferMode::Tow => -82.0,
    };
    let bend = vec2(
        (start.x + end.x) * 0.5,
        (start.y + end.y) * 0.5 - bend_offset,
    );
    let beam_color = match mode {
        TransferMode::InternalCargo => visual_theme::cyan(),
        TransferMode::ExternalClamp => visual_theme::amber(),
        TransferMode::Tow => visual_theme::warning(),
    };
    draw_line(
        start.x,
        start.y,
        bend.x,
        bend.y,
        14.0,
        visual_theme::with_alpha(beam_color, 0.45),
    );
    draw_line(
        bend.x,
        bend.y,
        end.x,
        end.y,
        14.0,
        visual_theme::with_alpha(beam_color, 0.45),
    );
    draw_line(start.x, start.y, bend.x, bend.y, 4.0, beam_color);
    draw_line(bend.x, bend.y, end.x, end.y, 4.0, beam_color);
    draw_extraction_effects(ExtractionEffectView {
        start,
        bend,
        end,
        target_rect,
        mode,
        phase: ctx
            .workspace_extraction_phase
            .unwrap_or(ExtractionPhase::Alignment),
        progress: ctx.workspace_extraction_progress,
        elapsed: ctx.workspace_elapsed,
    });
}

struct ExtractionEffectView {
    start: Vec2,
    bend: Vec2,
    end: Vec2,
    target_rect: Rect,
    mode: TransferMode,
    phase: ExtractionPhase,
    progress: f32,
    elapsed: f32,
}

fn draw_extraction_effects(view: ExtractionEffectView) {
    let ExtractionEffectView {
        start,
        bend,
        end,
        target_rect,
        mode,
        phase,
        progress,
        elapsed,
    } = view;
    let accent = match mode {
        TransferMode::InternalCargo => visual_theme::cyan(),
        TransferMode::ExternalClamp => visual_theme::amber(),
        TransferMode::Tow => visual_theme::warning(),
    };
    match phase {
        ExtractionPhase::Alignment => {
            let pulse = 18.0 + (elapsed * 4.0).sin().abs() * 10.0;
            draw_circle_lines(end.x, end.y, pulse, 2.0, accent);
            draw_line(
                end.x - pulse - 8.0,
                end.y,
                end.x - pulse,
                end.y,
                2.0,
                accent,
            );
            draw_line(
                end.x + pulse,
                end.y,
                end.x + pulse + 8.0,
                end.y,
                2.0,
                accent,
            );
        }
        ExtractionPhase::Connection => {
            draw_circle_lines(end.x, end.y, 22.0, 2.0, accent);
            for index in 1..4 {
                let point = beam_point(start, bend, end, index as f32 / 4.0);
                draw_circle(point.x, point.y, 4.0, accent);
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
