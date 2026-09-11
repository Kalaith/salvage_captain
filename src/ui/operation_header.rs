//! Compact resource badges shared by travel and salvage workspace headers.

use super::visual_theme;
use super::*;

pub(super) fn draw_operation_badges(ctx: &UiContext<'_>) {
    let site_label = ctx
        .session
        .expedition
        .as_ref()
        .and_then(|expedition| ctx.data.sites.get(&expedition.site_id))
        .map_or("NO DESTINATION".to_owned(), |site| {
            let section = ctx
                .session
                .expedition
                .as_ref()
                .map(|expedition| expedition.workspace_section.replace('_', " "))
                .unwrap_or_default();
            if section.is_empty() {
                site.display_name.to_uppercase()
            } else {
                format!(
                    "{} / {}",
                    site.display_name.to_uppercase(),
                    section.to_uppercase()
                )
            }
        });
    badge(
        Rect::new(286.0, 20.0, 206.0, 46.0),
        &clipped(&site_label, 20),
        visual_theme::with_alpha(visual_theme::amber(), 0.22),
    );
    let plan_label = ctx
        .session
        .expedition
        .as_ref()
        .map_or("PLAN --".to_owned(), |expedition| {
            format!("PLAN {}", expedition.voyage_plan.label())
        });
    badge(
        Rect::new(500.0, 20.0, 104.0, 46.0),
        &plan_label,
        visual_theme::with_alpha(visual_theme::cyan(), 0.2),
    );
    badge(
        Rect::new(612.0, 20.0, 90.0, 46.0),
        &format!("FUEL {}", ctx.session.economy.fuel),
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.75),
    );
    badge(
        Rect::new(710.0, 20.0, 90.0, 46.0),
        &hull_badge_label(ctx.session.hull, ctx.session.ship_wear()),
        visual_theme::with_alpha(visual_theme::warning(), 0.26),
    );
    badge(
        Rect::new(808.0, 20.0, 90.0, 46.0),
        &ctx.session.workspace_energy().map_or_else(
            || "POWER --".to_owned(),
            |(remaining, capacity)| format!("POWER {remaining}/{capacity}"),
        ),
        power_badge_color(ctx.session.workspace_energy()),
    );
    draw_text(
        &field_power_cell_label(ctx.session.field_power_cells),
        818.0,
        60.0,
        8.0,
        if ctx.session.field_power_cells > 0 {
            visual_theme::amber()
        } else {
            visual_theme::text_dim()
        },
    );
    badge(
        Rect::new(906.0, 20.0, 78.0, 46.0),
        &format!("CARGO {}", expedition_cargo_count(ctx)),
        visual_theme::with_alpha(visual_theme::safe(), 0.22),
    );
}

pub(super) fn hull_badge_label(hull: i32, ship_wear: u8) -> String {
    format!("HULL {hull} // W{ship_wear}")
}

pub(super) fn log_button_label(entry_count: usize) -> String {
    format!("LOG {:02}", entry_count)
}

pub(super) fn field_power_cell_label(cells: u8) -> String {
    format!("CELL {cells}")
}

#[cfg(test)]
mod tests;

pub(super) fn power_badge_color(reserve: Option<(i32, i32)>) -> Color {
    let Some((remaining, capacity)) = reserve else {
        return visual_theme::with_alpha(visual_theme::cyan_dim(), 0.7);
    };
    if remaining <= 0 || remaining * 3 <= capacity {
        visual_theme::with_alpha(visual_theme::warning(), 0.34)
    } else {
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.7)
    }
}

fn expedition_cargo_count(ctx: &UiContext<'_>) -> usize {
    ctx.session.expedition.as_ref().map_or(0, |expedition| {
        expedition
            .cargo
            .iter()
            .filter(|cargo| matches!(cargo.status, CargoStatus::Pending | CargoStatus::Packed))
            .count()
    })
}
