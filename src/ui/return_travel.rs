//! Automatic return leg that spends the reserved safe-return fuel.

use super::scene_layout;
use super::ship_visual;
use super::visual_theme;
use super::*;
use crate::state::{ReturnPolicy, VoyageRecord};
use macroquad_toolkit::math::pulse_range;

pub(crate) const RETURN_TRAVEL_DURATION_SECONDS: f32 = 3.0;

pub fn draw_return_travel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let view = scene_layout::travel_view();
    panel(view, visual_theme::panel_soft());
    let progress = (ctx.return_elapsed / RETURN_TRAVEL_DURATION_SECONDS).clamp(0.0, 1.0);
    let record = ctx.session.last_voyage();
    let site_name = record
        .and_then(|record| ctx.data.sites.get(&record.site_id))
        .map_or("UNKNOWN WRECK", |site| site.display_name.as_str());
    let live_returned_value: i64 = ctx
        .session
        .returned
        .iter()
        .filter_map(|item| {
            ctx.data.salvage_objects.get(&item.object_id).map(|object| {
                ctx.session
                    .market_quote(&item.object_id, ctx.data)
                    .map_or(object.sale_value, |quote| quote.sale_value)
            })
        })
        .sum();
    let returned_value = return_manifest_value(record, live_returned_value);
    let returned_count = record.map_or(ctx.session.returned.len(), |record| {
        record.recovered_count as usize
    });
    let return_fuel = ctx.data.config.safe_return_buffer.max(0);
    let fuel_before = ctx.session.economy.fuel + return_fuel;

    draw_text("AUTOMATIC RETURN", 54.0, 176.0, 16.0, visual_theme::cyan());
    draw_text(
        "THE YARD IS WAITING",
        54.0,
        214.0,
        30.0,
        visual_theme::text(),
    );
    draw_text(
        format!("WORKBOAT OUTBOUND // {site_name}"),
        56.0,
        239.0,
        15.0,
        visual_theme::text_dim(),
    );
    draw_text(
        "The return burn is committed. Cargo stays aboard until the yard debrief.",
        54.0,
        276.0,
        15.0,
        visual_theme::text_dim(),
    );
    if let Some(record) = record {
        draw_text(
            return_flight_report_label(record, ctx.session.last_return_policy),
            54.0,
            308.0,
            12.0,
            if record.risk_outcome == RiskOutcome::OrdinaryReturn {
                visual_theme::safe()
            } else {
                visual_theme::warning()
            },
        );
    }
    draw_return_route(progress, ctx.return_elapsed);
    let ship_rect = scene_layout::travel_ship_rect(1.0 - progress);
    draw_return_wake(ship_rect, ctx.return_elapsed, progress);
    ship_visual::draw_ship(ship_rect, ctx.session, ctx.data, ctx.return_elapsed, false);

    let brief = Rect::new(438.0, 478.0, 404.0, 166.0);
    panel(brief, visual_theme::with_alpha(visual_theme::panel(), 0.94));
    draw_text(
        "RETURN MANIFEST",
        brief.x + 20.0,
        brief.y + 28.0,
        17.0,
        visual_theme::text(),
    );
    draw_text(
        format!("FROM WRECK  {}", site_name),
        brief.x + 20.0,
        brief.y + 54.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!("FUEL BEFORE  {fuel_before}"),
        brief.x + 20.0,
        brief.y + 78.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "FUEL AFTER   {}  //  BURN {}",
            ctx.session.economy.fuel, return_fuel
        ),
        brief.x + 196.0,
        brief.y + 78.0,
        14.0,
        visual_theme::cyan(),
    );
    draw_text(
        format!(
            "HAUL         {:02} OBJECTS  //  VALUE ¢{}",
            returned_count, returned_value
        ),
        brief.x + 20.0,
        brief.y + 102.0,
        13.0,
        visual_theme::amber(),
    );
    draw_text(
        format!(
            "STATUS       {}  //  ETA {}",
            return_phase(progress),
            return_eta(progress)
        ),
        brief.x + 20.0,
        brief.y + 126.0,
        13.0,
        return_phase_color(progress),
    );
    if let Some(record) = record {
        draw_text(
            format!(
                "CLEARANCE    {} FRAME(S)  //  BOUNTY ¢{}",
                record.cleared_sections.len(),
                record.clearance_payout
            ),
            brief.x + 20.0,
            brief.y + 150.0,
            11.0,
            if record.clearance_payout > 0 {
                visual_theme::safe()
            } else {
                visual_theme::text_dim()
            },
        );
    }
    visual_theme::draw_meter(
        Rect::new(brief.x + 196.0, brief.y + 114.0, 184.0, 22.0),
        progress,
        return_phase_color(progress),
        &format!("DOCKING  {:02}%", (progress * 100.0) as i32),
    );
    if button(
        ctx,
        Rect::new(490.0, 658.0, 300.0, 38.0),
        "DOCK NOW",
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::ContinueReturn);
    }
}

fn return_manifest_value(record: Option<&VoyageRecord>, live_value: i64) -> i64 {
    record.map_or(live_value, |record| record.recovered_value)
}

fn return_flight_report_label(record: &VoyageRecord, return_policy: ReturnPolicy) -> String {
    format!(
        "FLIGHT REPORT  //  {}  //  {}  //  PLAN {}  //  POLICY {}  //  DRONE {}  //  SCAN {}",
        risk_label(record.risk_outcome),
        if record.contract_accepted {
            "CONTRACT ACTIVE"
        } else {
            "PRIVATE HAUL"
        },
        record.voyage_plan.label(),
        return_policy.short_label(),
        record.drone_directive.short_label(),
        record.scan_profile.short_label()
    )
}

fn draw_return_route(progress: f32, elapsed: f32) {
    let route_y = 386.0;
    let start_x = 96.0;
    let end_x = 890.0;
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
        let x = 114.0 + index as f32 * 78.0;
        let reached = index as f32 / 9.0 >= 1.0 - progress;
        draw_circle(
            x,
            route_y,
            4.0,
            if reached {
                visual_theme::safe()
            } else {
                visual_theme::amber()
            },
        );
    }
    draw_rectangle(
        start_x - 10.0,
        route_y - 38.0,
        48.0,
        34.0,
        visual_theme::with_alpha(visual_theme::safe(), 0.2),
    );
    draw_rectangle_lines(
        start_x - 10.0,
        route_y - 38.0,
        48.0,
        34.0,
        2.0,
        visual_theme::safe(),
    );
    draw_text(
        "YARD",
        start_x - 4.0,
        route_y - 16.0,
        10.0,
        visual_theme::safe(),
    );
    draw_rectangle(
        end_x - 14.0,
        route_y - 42.0,
        58.0,
        42.0,
        visual_theme::with_alpha(visual_theme::amber(), 0.18),
    );
    draw_rectangle_lines(
        end_x - 14.0,
        route_y - 42.0,
        58.0,
        42.0,
        2.0,
        visual_theme::amber(),
    );
    draw_text(
        "WRECK",
        end_x - 10.0,
        route_y + 18.0,
        10.0,
        visual_theme::amber(),
    );
    draw_text(
        format!("RETURN VECTOR  //  {}% CLEARED", (progress * 100.0) as i32),
        96.0,
        route_y + 56.0,
        11.0,
        visual_theme::text_dim(),
    );
    let pulse = pulse_range(2.0, 0.55, 1.0) * (0.82 + 0.18 * (elapsed * 2.0).sin().abs());
    draw_circle(
        98.0 + (1.0 - progress) * 790.0,
        route_y,
        12.0,
        visual_theme::with_alpha(visual_theme::cyan(), 0.12 * pulse),
    );
}

fn draw_return_wake(ship: Rect, elapsed: f32, progress: f32) {
    let count = if progress >= 1.0 { 0 } else { 4 };
    for index in 0..count {
        let offset = index as f32 * 18.0 + (elapsed * 3.0 + index as f32).sin() * 3.0;
        let y = ship.y + ship.h * (0.38 + index as f32 * 0.08);
        draw_line(
            ship.right() + 10.0 + offset,
            y,
            ship.right() + 34.0 + offset,
            y + (index as f32 - 2.0) * 2.0,
            2.0,
            visual_theme::with_alpha(visual_theme::cyan(), 0.62 - index as f32 * 0.08),
        );
    }
}

fn return_phase(progress: f32) -> &'static str {
    match progress.clamp(0.0, 1.0) {
        value if value < 0.2 => "DEPARTING WRECK",
        value if value < 0.8 => "RETURN CRUISE",
        value if value < 1.0 => "YARD APPROACH",
        _ => "DOCKED",
    }
}

fn return_eta(progress: f32) -> String {
    let seconds = ((1.0 - progress.clamp(0.0, 1.0)) * RETURN_TRAVEL_DURATION_SECONDS).ceil() as i32;
    if seconds == 0 {
        "NOW".to_owned()
    } else {
        format!("{seconds}s")
    }
}

fn return_phase_color(progress: f32) -> Color {
    if progress >= 1.0 {
        visual_theme::safe()
    } else if progress >= 0.8 {
        visual_theme::amber()
    } else {
        visual_theme::cyan()
    }
}
