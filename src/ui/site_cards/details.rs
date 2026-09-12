//! Site market, danger, and wreck-brief presentation helpers.

use super::*;

pub(super) fn site_market_outlook_label(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
) -> String {
    let Some(quote) = site
        .candidate_salvage
        .iter()
        .filter_map(|object_id| session.market_quote(object_id, data))
        .max_by_key(|quote| (quote.signed_multiplier(), quote.sale_value))
    else {
        return "DEMAND UNKNOWN".to_owned();
    };
    format!(
        "MKT {} {:+}%",
        quote.band.label(),
        quote.signed_multiplier()
    )
}

pub(super) fn site_departure_danger(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
    voyage_plan: crate::engine::VoyagePlan,
) -> i32 {
    let route_danger = crate::engine::danger_after_intel(
        site.danger,
        session.reconnaissance_level(&site.id),
        &data.config.reconnaissance,
    )
    .saturating_sub(session.route_familiarity_danger_reduction(&site.id));
    session.maintenance_adjusted_danger(
        session.crew_adjusted_danger(
            voyage_plan.adjust_danger(route_danger, &data.config.voyage_plan),
        ),
        data,
    )
}

pub(super) fn site_danger_label(
    site: &crate::data::SiteData,
    session: &GameSession,
    data: &GameData,
    voyage_plan: crate::engine::VoyagePlan,
) -> String {
    let level = session.reconnaissance_level(&site.id);
    let familiarity = session.route_familiarity(&site.id);
    let danger = site_departure_danger(site, session, data, voyage_plan);
    let plan_delta = voyage_plan.danger_delta(&data.config.voyage_plan);
    let crew_delta = session.crew_danger_delta();
    let maintenance_delta = session.maintenance_danger_delta(data);
    if level == 0
        && familiarity == 0
        && plan_delta == 0
        && crew_delta == 0
        && maintenance_delta == 0
    {
        format!("DANGER  {:02}%", site.danger)
    } else {
        let mut adjustments = Vec::new();
        if level > 0 {
            adjustments.push(format!(
                "INTEL -{}",
                level as i32 * data.config.reconnaissance.danger_reduction_per_level
            ));
        }
        if familiarity > 0 {
            adjustments.push(format!(
                "ROUTE -{}",
                session.route_familiarity_danger_reduction(&site.id)
            ));
        }
        if plan_delta != 0 {
            adjustments.push(format!("PLAN {plan_delta:+}"));
        }
        if crew_delta != 0 {
            adjustments.push(format!("CREW {crew_delta:+}"));
        }
        if maintenance_delta != 0 {
            adjustments.push(format!("WEAR {maintenance_delta:+}"));
        }
        format!(
            "DANGER  {:02}% -> {:02}%  //  {}",
            site.danger,
            danger,
            adjustments.join("  //  ")
        )
    }
}

pub(super) fn draw_wreck_brief(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    theme: &str,
    condition: i32,
) {
    let accent = visual_theme::site_accent(theme);
    draw_rectangle(x, y, width, height, visual_theme::space());
    draw_rectangle_lines(x, y, width, height, 2.0, accent);
    draw_line(
        x + 26.0,
        y + height * 0.28,
        x + width - 32.0,
        y + height * 0.48,
        5.0,
        visual_theme::structure_light(),
    );
    draw_line(
        x + 36.0,
        y + height * 0.67,
        x + width - 56.0,
        y + height * 0.58,
        3.0,
        visual_theme::structure(),
    );
    draw_line(
        x + width * 0.35,
        y + 12.0,
        x + width * 0.44,
        y + height - 12.0,
        2.0,
        accent,
    );
    draw_line(
        x + width * 0.7,
        y + 16.0,
        x + width * 0.62,
        y + height - 16.0,
        2.0,
        accent,
    );
    for index in 0..5 {
        let light_x = x + 28.0 + index as f32 * (width - 70.0) / 4.0;
        draw_circle(light_x, y + height * 0.31, 3.0, accent);
    }
    draw_profile_hint(x, y, width, height, theme);
    draw_rectangle(
        x + 18.0,
        y + height - 25.0,
        102.0,
        14.0,
        visual_theme::with_alpha(accent, 0.18),
    );
    draw_text(
        format!("COND {}%", condition),
        x + 25.0,
        y + height - 14.0,
        10.0,
        accent,
    );
    draw_text(
        "VISUAL SCAN",
        x + width - 102.0,
        y + height - 14.0,
        10.0,
        visual_theme::text_dim(),
    );
}

pub(super) fn draw_profile_hint(x: f32, y: f32, width: f32, height: f32, theme: &str) {
    match theme {
        "military" => {
            for index in 0..2 {
                let start_x = x + width * (0.16 + index as f32 * 0.42);
                draw_line(
                    start_x,
                    y + height * 0.52,
                    start_x + width * 0.18,
                    y + height * 0.84,
                    5.0,
                    visual_theme::with_alpha(visual_theme::warning(), 0.72),
                );
            }
        }
        "research" => {
            draw_rectangle_lines(
                x + width * 0.12,
                y + height * 0.55,
                width * 0.28,
                height * 0.2,
                2.0,
                visual_theme::with_alpha(visual_theme::cyan(), 0.72),
            );
            draw_circle_lines(
                x + width * 0.74,
                y + height * 0.6,
                height * 0.16,
                2.0,
                visual_theme::with_alpha(visual_theme::cyan(), 0.72),
            );
        }
        _ => {
            for index in 0..3 {
                let crate_x = x + width * (0.14 + index as f32 * 0.25);
                draw_rectangle(
                    crate_x,
                    y + height * 0.6,
                    width * 0.16,
                    height * 0.16,
                    visual_theme::with_alpha(visual_theme::amber(), 0.5),
                );
            }
        }
    }
}
