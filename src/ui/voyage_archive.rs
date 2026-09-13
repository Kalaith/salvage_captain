//! Captain's journal: navigable records, lifetime figures, ledger and awards.

mod navigation;
mod records;
mod statistics;
pub use navigation::{ArchiveAction, ArchiveFilter, ArchiveState, ArchiveTab, ARCHIVE_PAGE_SIZE};

use super::*;

pub(super) fn archive_button_label(run_count: usize, open: bool) -> String {
    if open {
        "CLOSE".to_owned()
    } else {
        format!("LOG {run_count:02}")
    }
}

pub fn draw_voyage_archive(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.journal_ui;
    let state = ctx.voyage_archive;
    let top = if ctx.state == GameState::Results
        || ctx.state == GameState::Pause && ctx.resume_state == GameState::Results
    {
        68.0
    } else {
        port_panel::HEADER_HEIGHT
    };
    draw_rectangle(
        0.0,
        top,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - top,
        visual_theme::space(),
    );
    text(
        &copy.title,
        Rect::new(28.0, 78.0, 710.0, 43.0),
        34.0,
        visual_theme::text(),
    );
    text(
        &copy.subtitle,
        Rect::new(30.0, 120.0, 800.0, 28.0),
        18.0,
        visual_theme::cyan(),
    );
    if button(
        ctx,
        Rect::new(1032.0, 88.0, 220.0, 48.0),
        &copy.close,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleVoyageArchive);
    }
    for (index, tab) in ArchiveTab::ALL.into_iter().enumerate() {
        control(
            ctx,
            actions,
            Rect::new(28.0 + index as f32 * 310.0, 158.0, 294.0, 48.0),
            &copy.tabs[index],
            state.tab == tab,
            ArchiveAction::Tab(tab),
        );
    }
    match state.tab {
        ArchiveTab::Voyages => records::draw(ctx, actions),
        ArchiveTab::Career | ArchiveTab::Ledger => statistics::draw(ctx, actions),
        ArchiveTab::Awards => statistics::draw_awards(ctx),
    }
    text(
        &copy.hints[state.tab as usize],
        Rect::new(30.0, 680.0, 1220.0, 30.0),
        18.0,
        visual_theme::text(),
    );
}

fn control(
    ctx: &UiContext<'_>,
    actions: &mut Vec<UiAction>,
    rect: Rect,
    label: &str,
    selected: bool,
    action: ArchiveAction,
) {
    if button(ctx, rect, label, true, ButtonTone::Secondary) {
        actions.push(UiAction::Archive(action));
    }
    if selected {
        draw_rectangle(
            rect.x,
            rect.bottom() - 4.0,
            rect.w,
            4.0,
            visual_theme::cyan(),
        );
    }
}

fn text(value: &str, rect: Rect, size: f32, color: Color) {
    visual_theme::body(value, rect, size, color);
}

fn metric(rect: Rect, label: &str, value: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::panel());
    text(
        label,
        Rect::new(rect.x + 14.0, rect.y + 8.0, rect.w - 28.0, 27.0),
        18.0,
        visual_theme::text(),
    );
    text(
        value,
        Rect::new(rect.x + 14.0, rect.y + 36.0, rect.w - 28.0, rect.h - 40.0),
        25.0,
        visual_theme::cyan(),
    );
}

fn metric_grid(area: Rect, labels: &[String], values: &[String], columns: usize) {
    let rows = labels.len().div_ceil(columns);
    let width = (area.w - (columns - 1) as f32 * 12.0) / columns as f32;
    let height = (area.h - (rows - 1) as f32 * 10.0) / rows as f32;
    for (index, (label, value)) in labels.iter().zip(values).enumerate() {
        metric(
            Rect::new(
                area.x + (index % columns) as f32 * (width + 12.0),
                area.y + (index / columns) as f32 * (height + 10.0),
                width,
                height,
            ),
            label,
            value,
        );
    }
}

fn money(value: i64) -> String {
    format!("{value} CR")
}
