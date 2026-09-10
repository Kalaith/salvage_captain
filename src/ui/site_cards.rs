//! Mission briefing: three wrecks presented as physical salvage jobs.

use super::*;
use crate::ui::visual_theme;

pub fn draw_site_selection(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let frame = Rect::new(0.0, 84.0, 1280.0, 636.0);
    panel(frame, visual_theme::panel_soft());
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        42.0,
        visual_theme::structure_dark(),
    );
    draw_text(
        "MISSION BRIEFING  //  AVAILABLE WRECKS",
        frame.x + 18.0,
        frame.y + 28.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "READ THE HULL. PRICE THE RETURN.",
        frame.right() - 262.0,
        frame.y + 27.0,
        11.0,
        visual_theme::amber(),
    );
    draw_text(
        "Fuel cost includes navigation discount; required fuel includes the safe-return buffer.",
        frame.x + 22.0,
        frame.y + 66.0,
        14.0,
        visual_theme::text_dim(),
    );

    for (index, site) in ctx.data.ordered_sites().into_iter().enumerate() {
        let x = 42.0 + index as f32 * 398.0;
        draw_site_card(ctx, actions, site, Rect::new(x, 194.0, 378.0, 370.0));
    }
    draw_text("A site choice is a risk choice: danger is previewed, but the exact setback is seeded at departure.", 46.0, 590.0, 14.0, visual_theme::text_dim());
}

fn draw_site_card(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    site: &crate::data::SiteData,
    rect: Rect,
) {
    let accent = visual_theme::site_accent(&site.visual_theme);
    panel(rect, visual_theme::panel());
    draw_rectangle(rect.x, rect.y, 6.0, rect.h, accent);
    draw_wreck_brief(
        rect.x + 18.0,
        rect.y + 16.0,
        rect.w - 36.0,
        92.0,
        &site.visual_theme,
        site.condition,
    );
    draw_text(
        &site.display_name.to_uppercase(),
        rect.x + 18.0,
        rect.y + 136.0,
        22.0,
        visual_theme::text(),
    );
    draw_text(
        &site.wreck_class.to_uppercase(),
        rect.x + 18.0,
        rect.y + 158.0,
        11.0,
        accent,
    );
    draw_text(
        clipped(&site.description, 47),
        rect.x + 18.0,
        rect.y + 184.0,
        13.0,
        visual_theme::text_dim(),
    );
    draw_text(
        format!("DANGER  {:02}%", site.danger),
        rect.x + 18.0,
        rect.y + 218.0,
        17.0,
        danger_color(site.danger),
    );
    let cost = ctx
        .session
        .effective_fuel_cost(&site.id, ctx.data)
        .unwrap_or(site.fuel_cost);
    let required = ctx
        .session
        .departure_fuel_required(&site.id, ctx.data)
        .unwrap_or(cost);
    draw_text(
        format!("FUEL  {} TRIP  /  {} REQUIRED", cost, required),
        rect.x + 18.0,
        rect.y + 244.0,
        13.0,
        visual_theme::text(),
    );
    let progress = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(site.condition, |value| value.condition);
    let visits = ctx
        .session
        .site_progress
        .get(&site.id)
        .map_or(0, |value| value.visits);
    let (recovered, total_targets) = ctx.session.site_recovery_summary(&site.id, ctx.data);
    draw_text(
        format!(
            "COND {}%  //  VISITS {}  //  RECOVERED {}/{}",
            progress, visits, recovered, total_targets
        ),
        rect.x + 18.0,
        rect.y + 266.0,
        11.0,
        visual_theme::text_dim(),
    );
    let contract_target = site
        .contract_target
        .as_deref()
        .and_then(|target_id| ctx.data.salvage_objects.get(target_id))
        .map_or_else(
            || site.contract_target.as_deref().unwrap_or("NONE").to_owned(),
            |target| target.display_name.clone(),
        );
    let contract_complete = ctx
        .session
        .site_progress
        .get(&site.id)
        .is_some_and(|value| value.contract_completed);
    draw_text(
        format!("KNOWN RETURN  {}", site.known_reward),
        rect.x + 18.0,
        rect.y + 280.0,
        12.0,
        visual_theme::text(),
    );
    draw_text(
        format!(
            "CONTRACT  {}  //  {}  //  +{} CR",
            if contract_complete {
                "COMPLETE"
            } else {
                "RECOVER"
            },
            contract_target.to_uppercase(),
            site.contract_reward
        ),
        rect.x + 18.0,
        rect.y + 298.0,
        10.0,
        if contract_complete {
            visual_theme::safe()
        } else {
            accent
        },
    );
    let section_count = site.sections.len();
    let gated_sections = site
        .sections
        .iter()
        .filter(|section| section.required_capability.is_some())
        .count();
    let stats = ctx.session.module_stats(ctx.data);
    let offline = ctx.session.damaged_modules.len();
    draw_text(
        format!(
            "SEC {}  //  GATE {}  //  OFF {}  //  SCAN+{}  HULL+{}  DRONE+{}",
            section_count, gated_sections, offline, stats.scanning, stats.hull, stats.drone_support
        ),
        rect.x + 18.0,
        rect.y + 308.0,
        11.0,
        visual_theme::text_dim(),
    );
    let can_depart = ctx.session.can_depart(&site.id, ctx.data);
    if button(
        ctx,
        Rect::new(rect.x + 18.0, rect.bottom() - 42.0, rect.w - 36.0, 34.0),
        if can_depart {
            "DEPART FOR WRECK"
        } else {
            "NOT ENOUGH FUEL"
        },
        can_depart,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::Depart(site.id.clone()));
    }
}

fn draw_wreck_brief(x: f32, y: f32, width: f32, height: f32, theme: &str, condition: i32) {
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
