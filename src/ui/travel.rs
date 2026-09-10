//! Automatic transit scene with a skippable arrival briefing.

use super::scene_layout;
use super::ship_visual;
use super::visual_theme;
use super::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TravelPhase {
    Departure,
    Cruise,
    FinalApproach,
    Docked,
}

pub fn draw_travel(ctx: &UiContext<'_>, _actions: &mut Vec<UiAction>) {
    let view = scene_layout::travel_view();
    panel(view, visual_theme::panel_soft());
    let Some(expedition) = &ctx.session.expedition else {
        draw_text(
            "NO ACTIVE TRANSIT",
            54.0,
            178.0,
            24.0,
            visual_theme::warning(),
        );
        return;
    };
    let Some(site) = ctx.data.sites.get(&expedition.site_id) else {
        return;
    };
    let progress = (ctx.travel_elapsed / 4.0).clamp(0.0, 1.0);
    let from_fuel = ctx.session.economy.fuel
        + ctx
            .session
            .effective_fuel_cost(&site.id, ctx.data)
            .unwrap_or(site.fuel_cost);
    draw_text("AUTOMATIC TRANSIT", 54.0, 176.0, 16.0, visual_theme::cyan());
    draw_text(
        &site.display_name.to_uppercase(),
        54.0,
        214.0,
        30.0,
        visual_theme::text(),
    );
    draw_text(
        &site.wreck_class,
        56.0,
        239.0,
        15.0,
        visual_theme::text_dim(),
    );
    draw_text(
        clipped(&site.arrival_text, 66),
        54.0,
        276.0,
        15.0,
        visual_theme::text_dim(),
    );
    if let Some(contract_target) = &site.contract_target {
        let target_name = ctx
            .data
            .salvage_objects
            .get(contract_target)
            .map_or(contract_target.as_str(), |target| {
                target.display_name.as_str()
            });
        let contract_complete = ctx
            .session
            .site_progress
            .get(&site.id)
            .is_some_and(|progress| progress.contract_completed);
        let contract_failed = ctx
            .session
            .site_progress
            .get(&site.id)
            .is_some_and(|progress| progress.contract_failed);
        draw_text(
            format!(
                "CONTRACT  //  {} {}  //  +{} CR",
                contract_status_label(contract_complete, contract_failed),
                target_name.to_uppercase(),
                site.contract_reward
            ),
            54.0,
            308.0,
            12.0,
            if contract_complete {
                visual_theme::safe()
            } else if contract_failed {
                visual_theme::warning()
            } else {
                visual_theme::site_accent(&site.visual_theme)
            },
        );
        draw_text(
            clipped(&site.contract_brief, 66),
            54.0,
            330.0,
            13.0,
            visual_theme::text_dim(),
        );
    }
    draw_text(
        format!(
            "FRAME PLAN  {} SECTIONS  //  {} HAZARD SIGNALS",
            site.sections.len(),
            site_hazard_count(site)
        ),
        54.0,
        352.0,
        12.0,
        visual_theme::site_accent(&site.visual_theme),
    );
    let phase = travel_phase(progress);
    draw_transit_route(
        progress,
        ctx.travel_elapsed,
        site.visual_theme.as_str(),
        phase,
    );
    draw_wreck_marker(
        862.0,
        330.0,
        site.visual_theme.as_str(),
        progress,
        ctx.travel_elapsed,
        phase,
    );
    let ship_rect = scene_layout::travel_ship_rect(progress);
    draw_transit_wake(
        ship_rect,
        ctx.travel_elapsed,
        site.visual_theme.as_str(),
        phase,
    );
    ship_visual::draw_ship(ship_rect, ctx.session, ctx.data, ctx.travel_elapsed, false);
    let brief = Rect::new(450.0, 500.0, 380.0, 142.0);
    panel(brief, visual_theme::with_alpha(visual_theme::panel(), 0.94));
    draw_text(
        "ARRIVAL BRIEF",
        brief.x + 20.0,
        brief.y + 28.0,
        17.0,
        visual_theme::text(),
    );
    draw_text(
        format!("DESTINATION  {}", site.display_name),
        brief.x + 20.0,
        brief.y + 54.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!("FUEL BEFORE  {}", from_fuel),
        brief.x + 20.0,
        brief.y + 76.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "FUEL AFTER {}  //  DANGER {:02}%",
            ctx.session.economy.fuel, site.danger
        ),
        brief.x + 160.0,
        brief.y + 76.0,
        14.0,
        danger_color(site.danger),
    );
    draw_text(
        format!("CLASS        {}", site.wreck_class),
        brief.x + 20.0,
        brief.y + 98.0,
        13.0,
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(brief.x + 160.0, brief.y + 88.0, 200.0, 24.0),
        progress,
        travel_phase_color(phase),
        &format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
    );
    draw_text(
        format!(
            "STATUS       {}  //  ETA {}",
            travel_phase_label(phase),
            travel_eta_label(progress)
        ),
        brief.x + 20.0,
        brief.y + 120.0,
        13.0,
        travel_phase_color(phase),
    );
    draw_text(
        travel_instruction(phase),
        54.0,
        650.0,
        14.0,
        visual_theme::text_dim(),
    );
}

fn travel_phase(progress: f32) -> TravelPhase {
    match progress.clamp(0.0, 1.0) {
        value if value < 0.18 => TravelPhase::Departure,
        value if value < 0.72 => TravelPhase::Cruise,
        value if value < 1.0 => TravelPhase::FinalApproach,
        _ => TravelPhase::Docked,
    }
}

fn travel_phase_label(phase: TravelPhase) -> &'static str {
    match phase {
        TravelPhase::Departure => "DEPARTURE",
        TravelPhase::Cruise => "CRUISE",
        TravelPhase::FinalApproach => "FINAL APPROACH",
        TravelPhase::Docked => "DOCKED",
    }
}

fn travel_eta_label(progress: f32) -> String {
    let seconds = ((1.0 - progress.clamp(0.0, 1.0)) * 4.0).ceil() as i32;
    if seconds == 0 {
        "NOW".to_owned()
    } else {
        format!("{seconds}s")
    }
}

fn wake_segment_count(phase: TravelPhase) -> usize {
    match phase {
        TravelPhase::Departure => 5,
        TravelPhase::Cruise => 4,
        TravelPhase::FinalApproach => 2,
        TravelPhase::Docked => 0,
    }
}

fn site_hazard_count(site: &crate::data::SiteData) -> usize {
    site.sections
        .iter()
        .map(|section| section.hazard_tags.len())
        .sum()
}

fn travel_phase_color(phase: TravelPhase) -> Color {
    match phase {
        TravelPhase::Departure | TravelPhase::Cruise => visual_theme::cyan(),
        TravelPhase::FinalApproach => visual_theme::amber(),
        TravelPhase::Docked => visual_theme::safe(),
    }
}

fn travel_instruction(phase: TravelPhase) -> &'static str {
    match phase {
        TravelPhase::Departure => "Transit cleared. Tap ARRIVE whenever you are ready.",
        TravelPhase::Cruise => "Route stable. Tap ARRIVE to skip ahead to the wreck.",
        TravelPhase::FinalApproach => {
            "Final approach locked. Tap ARRIVE to enter the wreck workspace."
        }
        TravelPhase::Docked => "Tap CONTINUE in the top HUD to enter the wreck workspace.",
    }
}

fn draw_transit_wake(ship: Rect, elapsed: f32, theme: &str, phase: TravelPhase) {
    let count = wake_segment_count(phase);
    if count == 0 {
        return;
    }
    let accent = visual_theme::site_accent(theme);
    for index in 0..count {
        let offset = index as f32 * 18.0 + (elapsed * 3.0 + index as f32).sin() * 3.0;
        let y = ship.y + ship.h * (0.38 + index as f32 * 0.08);
        draw_line(
            ship.x - 10.0 - offset,
            y,
            ship.x - 34.0 - offset,
            y + (index as f32 - 2.0) * 2.0,
            2.0,
            visual_theme::with_alpha(accent, 0.65 - index as f32 * 0.08),
        );
    }
}

fn draw_transit_route(progress: f32, elapsed: f32, theme: &str, phase: TravelPhase) {
    let start_x = 92.0;
    let end_x = 880.0;
    let route_y = 390.0;
    let accent = visual_theme::site_accent(theme);
    draw_line(
        start_x,
        route_y - 18.0,
        end_x,
        route_y - 18.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.32),
    );
    draw_line(
        start_x,
        route_y + 18.0,
        end_x,
        route_y + 18.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.32),
    );
    draw_line(
        start_x,
        route_y,
        end_x,
        route_y,
        2.0,
        visual_theme::cyan_dim(),
    );
    for index in 0..10 {
        let marker_progress = index as f32 / 9.0;
        let x = 112.0 + index as f32 * 78.0;
        let reached = marker_progress <= progress;
        draw_circle(
            x,
            route_y,
            if reached { 4.0 } else { 2.0 },
            if reached {
                accent
            } else {
                visual_theme::structure_light()
            },
        );
    }
    if phase != TravelPhase::Docked {
        let signal_progress = (elapsed * 0.32).fract();
        let signal_x = start_x + (end_x - start_x) * signal_progress;
        draw_circle(
            signal_x,
            route_y,
            5.0,
            visual_theme::with_alpha(visual_theme::text(), 0.85),
        );
        draw_circle_lines(
            signal_x,
            route_y,
            12.0 + (elapsed * 5.0).sin().abs() * 5.0,
            2.0,
            visual_theme::with_alpha(accent, 0.72),
        );
    }
    if matches!(phase, TravelPhase::FinalApproach | TravelPhase::Docked) {
        draw_text(
            "DOCKING CORRIDOR",
            716.0,
            route_y - 28.0,
            10.0,
            travel_phase_color(phase),
        );
    }
}

fn draw_wreck_marker(x: f32, y: f32, theme: &str, progress: f32, elapsed: f32, phase: TravelPhase) {
    let accent = visual_theme::site_accent(theme);
    let center = vec2(x + 60.0, y + 38.0);
    if matches!(phase, TravelPhase::FinalApproach | TravelPhase::Docked) {
        let pulse = if phase == TravelPhase::Docked {
            42.0
        } else {
            30.0 + (elapsed * 3.0).sin().abs() * 10.0
        };
        draw_circle_lines(
            center.x,
            center.y,
            pulse,
            2.0,
            visual_theme::with_alpha(accent, 0.7),
        );
        draw_line(
            center.x - pulse - 12.0,
            center.y,
            center.x - pulse,
            center.y,
            2.0,
            accent,
        );
        draw_line(
            center.x + pulse,
            center.y,
            center.x + pulse + 12.0,
            center.y,
            2.0,
            accent,
        );
    }
    draw_rectangle(x, y, 120.0, 76.0, visual_theme::structure_dark());
    draw_rectangle_lines(x, y, 120.0, 76.0, 3.0, accent);
    draw_line(x + 18.0, y + 20.0, x + 95.0, y + 58.0, 3.0, accent);
    draw_line(x + 82.0, y + 16.0, x + 18.0, y + 62.0, 3.0, accent);
    draw_profile_signature(x, y, theme, elapsed);
    draw_text(
        if progress >= 1.0 {
            "DOCKED"
        } else if phase == TravelPhase::FinalApproach {
            "APPROACH"
        } else {
            "TARGET"
        },
        x + 30.0,
        y + 100.0,
        13.0,
        visual_theme::text_dim(),
    );
}

fn draw_profile_signature(x: f32, y: f32, theme: &str, elapsed: f32) {
    match theme {
        "military" => {
            draw_line(
                x + 20.0,
                y + 18.0,
                x + 48.0,
                y + 58.0,
                4.0,
                visual_theme::warning(),
            );
            draw_line(
                x + 100.0,
                y + 18.0,
                x + 72.0,
                y + 58.0,
                4.0,
                visual_theme::warning(),
            );
            draw_circle(x + 60.0, y + 38.0, 5.0, visual_theme::warning());
        }
        "research" => {
            draw_circle_lines(
                x + 60.0,
                y + 38.0,
                19.0 + (elapsed * 2.0).sin().abs() * 5.0,
                2.0,
                visual_theme::cyan(),
            );
            draw_circle(x + 60.0, y + 38.0, 4.0, visual_theme::cyan());
        }
        _ => {
            for index in 0..3 {
                draw_rectangle(
                    x + 18.0 + index as f32 * 27.0,
                    y + 51.0,
                    18.0,
                    10.0,
                    visual_theme::with_alpha(visual_theme::amber(), 0.75),
                );
            }
        }
    }
}
