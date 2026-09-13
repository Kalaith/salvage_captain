//! Selectable voyage list and complete, grouped return records.

use super::*;
use crate::state::VoyageRecord;

pub(super) fn draw(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.journal_ui;
    let state = ctx.voyage_archive;
    for (index, filter) in ArchiveFilter::ALL.into_iter().enumerate() {
        control(
            ctx,
            actions,
            Rect::new(28.0 + index as f32 * 180.0, 220.0, 168.0, 44.0),
            &copy.filters[index],
            state.filter == filter,
            ArchiveAction::Filter(filter),
        );
    }
    let total = state.matching_indices(&ctx.session.voyage_log).len();
    let start = state.page_start(total);
    let selected = state.selected_index(&ctx.session.voyage_log);
    if selected.is_none() {
        draw_empty(ctx);
        return;
    }
    text(
        &format!(
            "{} {} {} {}",
            copy.page,
            start / ARCHIVE_PAGE_SIZE + 1,
            copy.of,
            total.div_ceil(ARCHIVE_PAGE_SIZE)
        ),
        Rect::new(838.0, 228.0, 404.0, 30.0),
        21.0,
        visual_theme::text(),
    );
    for (row, index) in state
        .visible_indices(&ctx.session.voyage_log)
        .into_iter()
        .enumerate()
    {
        draw_row(ctx, actions, row, index, selected == Some(index));
    }
    for (index, (label, enabled, action)) in [
        (&copy.newer, start > 0, ArchiveAction::Newer),
        (
            &copy.older,
            start + ARCHIVE_PAGE_SIZE < total,
            ArchiveAction::Older,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            ctx,
            Rect::new(28.0 + index as f32 * 188.0, 624.0, 176.0, 44.0),
            label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::Archive(action));
        }
    }
    if let Some(index) = selected {
        draw_record(ctx, actions, index);
    }
}

fn draw_empty(ctx: &UiContext<'_>) {
    let copy = &ctx.data.journal_ui;
    let empty = ctx.session.voyage_log.is_empty();
    visual_theme::surface(Rect::new(28.0, 284.0, 1224.0, 376.0));
    text(
        if empty {
            &copy.empty_title
        } else {
            &copy.filtered_title
        },
        Rect::new(66.0, 364.0, 1120.0, 50.0),
        34.0,
        visual_theme::text(),
    );
    text(
        if empty {
            &copy.empty_hint
        } else {
            &copy.filtered_hint
        },
        Rect::new(66.0, 432.0, 950.0, 105.0),
        24.0,
        visual_theme::text(),
    );
}

fn site_name<'a>(ctx: &'a UiContext<'_>, record: &'a VoyageRecord) -> &'a str {
    ctx.data
        .sites
        .get(&record.site_id)
        .map_or(record.site_id.as_str(), |site| site.display_name.as_str())
}

fn draw_row(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    row: usize,
    index: usize,
    selected: bool,
) {
    let record = &ctx.session.voyage_log[index];
    let rect = Rect::new(28.0, 278.0 + row as f32 * 68.0, 364.0, 60.0);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            visual_theme::structure_dark()
        } else {
            visual_theme::panel()
        },
    );
    draw_rectangle(
        rect.x,
        rect.y,
        4.0,
        rect.h,
        if selected {
            visual_theme::cyan()
        } else {
            visual_theme::structure()
        },
    );
    text(
        &format!("{:02}  {}", index + 1, site_name(ctx, record)),
        Rect::new(rect.x + 14.0, rect.y + 5.0, rect.w - 24.0, 29.0),
        22.0,
        visual_theme::text(),
    );
    text(
        &format!(
            "{} / {}",
            money(record.recovered_value),
            risk_label(record.risk_outcome)
        ),
        Rect::new(rect.x + 14.0, rect.y + 34.0, rect.w - 24.0, 23.0),
        17.0,
        visual_theme::text(),
    );
    if ctx.interaction_enabled && ctx.pointer.released_on(rect) {
        actions.push(UiAction::Archive(ArchiveAction::Select(index)));
    }
}

fn draw_record(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, index: usize) {
    let copy = &ctx.data.journal_ui;
    let record = &ctx.session.voyage_log[index];
    text(
        &format!("{} {:02} / {}", copy.run, index + 1, site_name(ctx, record)),
        Rect::new(416.0, 278.0, 820.0, 36.0),
        28.0,
        visual_theme::text(),
    );
    text(
        risk_label(record.risk_outcome),
        Rect::new(416.0, 315.0, 820.0, 26.0),
        20.0,
        if record.risk_outcome == RiskOutcome::OrdinaryReturn {
            visual_theme::safe()
        } else {
            visual_theme::warning()
        },
    );
    for (page, label) in copy.voyage_pages.iter().enumerate() {
        control(
            ctx,
            actions,
            Rect::new(416.0 + page as f32 * 418.0, 348.0, 402.0, 44.0),
            label,
            ctx.voyage_archive.page == page,
            ArchiveAction::Page(page),
        );
    }
    if ctx.voyage_archive.page == 0 {
        draw_recovery(ctx, record);
    } else {
        draw_preparation(ctx, record);
    }
}

fn draw_recovery(ctx: &UiContext<'_>, record: &VoyageRecord) {
    let copy = &ctx.data.journal_ui;
    let contract = if !record.contract_accepted {
        0
    } else if record.contract_completed {
        1
    } else if record.contract_failed {
        2
    } else {
        3
    };
    let values = vec![
        record.recovered_count.to_string(),
        money(record.recovered_value),
        record.recovered_alloy.to_string(),
        record.recovered_electronics.to_string(),
        record.external_load.to_string(),
        record.return_fuel.to_string(),
        copy.contracts[contract].clone(),
        record.cleared_sections.len().to_string(),
        money(record.clearance_payout),
        format!("{}%", record.condition_after),
        record.market_cycle.to_string(),
        record.danger_score.to_string(),
    ];
    metric_grid(
        Rect::new(416.0, 404.0, 836.0, 264.0),
        &copy.recovery,
        &values,
        4,
    );
}

fn draw_preparation(ctx: &UiContext<'_>, record: &VoyageRecord) {
    let copy = &ctx.data.journal_ui;
    let values = vec![
        record.voyage_plan.label().to_owned(),
        record.return_policy.short_label().to_owned(),
        record.scan_profile.short_label().to_owned(),
        record.drone_directive.short_label().to_owned(),
        record.reconnaissance_level.to_string(),
        copy.coverage[usize::from(record.insured)].clone(),
        money(record.insurance_premium),
        money(record.insurance_payout),
    ];
    metric_grid(
        Rect::new(416.0, 404.0, 836.0, 174.0),
        &copy.preparation,
        &values,
        4,
    );
    let names: Vec<_> = record
        .cleared_sections
        .iter()
        .map(|id| {
            ctx.data
                .sites
                .get(&record.site_id)
                .and_then(|site| site.sections.iter().find(|section| section.id == *id))
                .map_or_else(
                    || id.replace('_', " "),
                    |section| section.display_name.clone(),
                )
        })
        .collect();
    text(
        &copy.sections,
        Rect::new(430.0, 590.0, 808.0, 27.0),
        18.0,
        visual_theme::text(),
    );
    let section_names = names.join(" / ");
    text(
        if names.is_empty() {
            &copy.none
        } else {
            &section_names
        },
        Rect::new(430.0, 620.0, 808.0, 50.0),
        22.0,
        visual_theme::cyan(),
    );
}
