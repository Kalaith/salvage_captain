//! Automatic transit scene with a skippable arrival briefing.

use super::scene_layout;
use super::ship_visual;
use super::visual_theme;
use super::*;

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
        draw_text(
            format!(
                "CONTRACT  //  {} {}  //  +{} CR",
                if contract_complete {
                    "COMPLETE"
                } else {
                    "RECOVER"
                },
                target_name.to_uppercase(),
                site.contract_reward
            ),
            54.0,
            308.0,
            12.0,
            visual_theme::site_accent(&site.visual_theme),
        );
        draw_text(
            clipped(&site.contract_brief, 66),
            54.0,
            330.0,
            13.0,
            visual_theme::text_dim(),
        );
    }
    draw_line(92.0, 390.0, 880.0, 390.0, 2.0, visual_theme::cyan_dim());
    for index in 0..10 {
        let x = 112.0 + index as f32 * 78.0;
        draw_circle(
            x,
            390.0,
            if index as f32 / 9.0 <= progress {
                4.0
            } else {
                2.0
            },
            if index as f32 / 9.0 <= progress {
                visual_theme::cyan()
            } else {
                visual_theme::structure_light()
            },
        );
    }
    draw_wreck_marker(862.0, 330.0, site.visual_theme.as_str(), progress);
    ship_visual::draw_ship(
        scene_layout::travel_ship_rect(progress),
        ctx.session,
        ctx.data,
        ctx.travel_elapsed,
        false,
    );
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
        visual_theme::cyan(),
        &format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
    );
    draw_text(
        if progress >= 1.0 {
            "Tap CONTINUE in the top HUD to enter the wreck workspace."
        } else {
            "Tap ARRIVE in the top HUD whenever you are ready."
        },
        54.0,
        650.0,
        14.0,
        visual_theme::text_dim(),
    );
}

fn draw_wreck_marker(x: f32, y: f32, theme: &str, progress: f32) {
    let accent = visual_theme::site_accent(theme);
    draw_rectangle(x, y, 120.0, 76.0, visual_theme::structure_dark());
    draw_rectangle_lines(x, y, 120.0, 76.0, 3.0, accent);
    draw_line(x + 18.0, y + 20.0, x + 95.0, y + 58.0, 3.0, accent);
    draw_line(x + 82.0, y + 16.0, x + 18.0, y + 62.0, 3.0, accent);
    draw_text(
        if progress >= 1.0 { "WRECK" } else { "TARGET" },
        x + 30.0,
        y + 100.0,
        13.0,
        visual_theme::text_dim(),
    );
}
