//! Automatic transit scene with a skippable arrival briefing.

use super::scene_layout;
use super::ship_visual;
use super::visual_theme;
use super::*;

pub fn draw_travel(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
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
    panel(Rect::new(958.0, 180.0, 254.0, 230.0), visual_theme::panel());
    draw_text("ARRIVAL BRIEF", 978.0, 212.0, 17.0, visual_theme::text());
    draw_text(
        format!("DESTINATION  {}", site.display_name),
        978.0,
        250.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!("FUEL BEFORE  {}", from_fuel),
        978.0,
        278.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("FUEL AFTER   {}", ctx.session.economy.fuel),
        978.0,
        304.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("DANGER       {:02}%", site.danger),
        978.0,
        330.0,
        14.0,
        danger_color(site.danger),
    );
    draw_text(
        format!("CLASS        {}", site.wreck_class),
        978.0,
        356.0,
        13.0,
        visual_theme::text_dim(),
    );
    visual_theme::draw_meter(
        Rect::new(978.0, 374.0, 214.0, 24.0),
        progress,
        visual_theme::cyan(),
        &format!("ARRIVAL  {:02}%", (progress * 100.0) as i32),
    );
    let label = if progress >= 1.0 {
        "CONTINUE"
    } else {
        "ARRIVE"
    };
    if button(
        ctx,
        Rect::new(978.0, 454.0, 214.0, 48.0),
        label,
        true,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::ContinueTravel);
    }
    draw_text(
        "Tap ARRIVE whenever you are ready.",
        54.0,
        552.0,
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
