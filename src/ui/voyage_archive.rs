//! Port-accessible archive of completed salvage runs.

use super::*;
use crate::state::VoyageRecord;
use crate::ui::port_panel::HEADER_HEIGHT;

#[cfg(test)]
mod tests;

pub const ARCHIVE_PAGE_SIZE: usize = 5;

pub(super) fn archive_button_label(run_count: usize, open: bool) -> String {
    if open {
        "CLOSE".to_owned()
    } else {
        format!("LOG {run_count:02}")
    }
}

pub fn draw_voyage_archive(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width.max(1.0);
    let height = ctx.viewport_height.max(1.0);
    draw_rectangle(
        0.0,
        HEADER_HEIGHT,
        width,
        (height - HEADER_HEIGHT).max(0.0),
        visual_theme::with_alpha(visual_theme::space(), 0.99),
    );
    let frame = Rect::new(
        42.0,
        HEADER_HEIGHT + 18.0,
        (width - 84.0).max(480.0),
        (height - HEADER_HEIGHT - 36.0).max(430.0),
    );
    panel(frame, visual_theme::with_alpha(visual_theme::panel(), 0.98));
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        52.0,
        visual_theme::structure_dark(),
    );
    draw_rectangle(frame.x, frame.y, 5.0, 52.0, visual_theme::amber());
    draw_text(
        "CAPTAIN'S LOGBOOK  //  VOYAGE ARCHIVE",
        frame.x + 20.0,
        frame.y + 32.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        &archive_header(ctx.session.voyage_log.len()),
        frame.right() - 218.0,
        frame.y + 31.0,
        10.0,
        visual_theme::amber(),
    );

    let unlocked = ctx.session.unlocked_module_count(ctx.data);
    let total = ctx.data.modules.iter().count();
    draw_text(
        &archive_summary(&ctx.session.voyage_log, unlocked, total),
        frame.x + 20.0,
        frame.y + 82.0,
        11.0,
        visual_theme::cyan(),
    );
    draw_line(
        frame.x + 20.0,
        frame.y + 96.0,
        frame.right() - 20.0,
        frame.y + 96.0,
        1.0,
        visual_theme::structure_light(),
    );

    if ctx.session.voyage_log.is_empty() {
        draw_text(
            "NO RETURN RECORDS",
            frame.x + 24.0,
            frame.y + 172.0,
            22.0,
            visual_theme::amber(),
        );
        draw_text(
            "Complete a salvage run to start the archive.",
            frame.x + 24.0,
            frame.y + 204.0,
            14.0,
            visual_theme::text_dim(),
        );
    } else {
        let row_top = frame.y + 112.0;
        let row_height = ((frame.h - 174.0) / ARCHIVE_PAGE_SIZE as f32).clamp(54.0, 72.0);
        let (page_offset, page_end) =
            archive_page(ctx.session.voyage_log.len(), ctx.voyage_archive_offset);
        for (index, record) in ctx
            .session
            .voyage_log
            .iter()
            .rev()
            .skip(page_offset)
            .take(page_end - page_offset)
            .enumerate()
        {
            let row = Rect::new(
                frame.x + 18.0,
                row_top + index as f32 * (row_height + 7.0),
                frame.w - 36.0,
                row_height,
            );
            draw_archive_row(
                ctx,
                row,
                record,
                ctx.session.voyage_log.len() - page_offset - index,
            );
        }
        if page_end < ctx.session.voyage_log.len() {
            draw_text(
                &format!(
                    "{} older run(s) remain filed in the saved archive.",
                    ctx.session.voyage_log.len() - page_end
                ),
                frame.x + 22.0,
                frame.bottom() - 68.0,
                10.0,
                visual_theme::text_dim(),
            );
        }
        if button(
            ctx,
            Rect::new(frame.x + 20.0, frame.bottom() - 46.0, 118.0, 32.0),
            "NEWER RUNS",
            page_offset > 0,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::ArchiveNewer);
        }
        if button(
            ctx,
            Rect::new(frame.x + 146.0, frame.bottom() - 46.0, 118.0, 32.0),
            "OLDER RUNS",
            page_end < ctx.session.voyage_log.len(),
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::ArchiveOlder);
        }
    }

    if button(
        ctx,
        Rect::new(frame.right() - 138.0, frame.bottom() - 46.0, 118.0, 32.0),
        "CLOSE LOG",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleVoyageArchive);
    }
}

fn archive_page(total: usize, requested_offset: usize) -> (usize, usize) {
    let offset = requested_offset.min(total.saturating_sub(1));
    (offset, (offset + ARCHIVE_PAGE_SIZE).min(total))
}

fn draw_archive_row(ctx: &UiContext<'_>, row: Rect, record: &VoyageRecord, run_number: usize) {
    let site_name = ctx
        .data
        .sites
        .get(&record.site_id)
        .map_or(record.site_id.as_str(), |site| site.display_name.as_str());
    let outcome_color = if record.risk_outcome == RiskOutcome::OrdinaryReturn {
        visual_theme::safe()
    } else {
        visual_theme::warning()
    };
    draw_rectangle(
        row.x,
        row.y,
        row.w,
        row.h,
        visual_theme::with_alpha(visual_theme::panel_soft(), 0.8),
    );
    draw_rectangle(row.x, row.y, 4.0, row.h, outcome_color);
    draw_text(
        &archive_entry_label(record, run_number, site_name),
        row.x + 16.0,
        row.y + 22.0,
        13.0,
        visual_theme::text(),
    );
    draw_text(
        &format!(
            "{}  //  SCAN {}  //  RECOV {} TARGET(S)  //  ¢{}  //  EXT {}",
            archive_contract_label(record),
            record.scan_profile.short_label(),
            record.recovered_count,
            record.recovered_value,
            record.external_load
        ),
        row.x + 16.0,
        row.y + 43.0,
        10.0,
        outcome_color,
    );
}

fn archive_header(run_count: usize) -> String {
    format!("{:02} RUN(S) FILED", run_count)
}

fn archive_summary(records: &[VoyageRecord], unlocked: usize, total: usize) -> String {
    let recovered_targets: u32 = records.iter().map(|record| record.recovered_count).sum();
    let recovered_value: i64 = records.iter().map(|record| record.recovered_value).sum();
    let external_load: u32 = records.iter().map(|record| record.external_load).sum();
    format!(
        "TOTAL HAUL  ¢{}  //  TARGETS {}  //  EXTERNAL {}  //  BP {:02}/{:02}",
        recovered_value, recovered_targets, external_load, unlocked, total
    )
}

fn archive_entry_label(record: &VoyageRecord, run_number: usize, site_name: &str) -> String {
    format!(
        "RUN {:02}  //  {}  //  {}",
        run_number,
        site_name.to_uppercase(),
        risk_label(record.risk_outcome)
    )
}

fn archive_contract_label(record: &VoyageRecord) -> &'static str {
    if record.contract_completed {
        "CONTRACT COMPLETE"
    } else if record.contract_failed {
        "CONTRACT FAILED"
    } else {
        "CONTRACT OPEN"
    }
}
