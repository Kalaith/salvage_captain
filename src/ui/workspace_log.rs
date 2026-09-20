//! Persistent operation record for the wreck currently under the workboat.

use super::visual_theme;
use super::*;
use crate::state::workspace_energy::{POWER_CYCLE_ENERGY_RESTORE, POWER_CYCLE_FUEL_COST};
use crate::state::{
    CrewRole, DroneDirective, WorkspaceLogEntry, WorkspaceLogEvent, WorkspaceScanProfile,
};

const LOG_FRAME: Rect = Rect::new(154.0, 108.0, 972.0, 552.0);
const MAX_VISIBLE_ENTRIES: usize = 6;

pub fn page_count(entry_count: usize) -> usize {
    entry_count.div_ceil(MAX_VISIBLE_ENTRIES).max(1)
}

pub fn turn_page(current: usize, next: bool, entry_count: usize) -> usize {
    let last = page_count(entry_count).saturating_sub(1);
    if next {
        (current + 1).min(last)
    } else {
        current.saturating_sub(1)
    }
}

pub fn draw_workspace_log(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        84.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 84.0,
        visual_theme::with_alpha(visual_theme::space(), 0.72),
    );
    panel(LOG_FRAME, visual_theme::panel_soft());
    draw_rectangle(
        LOG_FRAME.x,
        LOG_FRAME.y,
        LOG_FRAME.w,
        46.0,
        visual_theme::structure_dark(),
    );
    let site_name = ctx
        .session
        .expedition
        .as_ref()
        .and_then(|expedition| ctx.data.sites.get(&expedition.site_id))
        .map_or("UNKNOWN WRECK", |site| site.display_name.as_str());
    log_text(
        &format!("SALVAGE OPERATION LOG  //  {}", site_name.to_uppercase()),
        Rect::new(LOG_FRAME.x + 18.0, LOG_FRAME.y + 9.0, 600.0, 30.0),
        18.0,
        visual_theme::text(),
    );
    if button(
        ctx,
        Rect::new(LOG_FRAME.right() - 218.0, LOG_FRAME.y + 1.0, 96.0, 44.0),
        "SUMMARY",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleWorkspaceLogSummary);
    }
    if button(
        ctx,
        Rect::new(LOG_FRAME.right() - 112.0, LOG_FRAME.y + 1.0, 94.0, 44.0),
        "CLOSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CloseWorkspaceLog);
    }

    let entries = ctx.session.workspace_log().unwrap_or(&[]);
    let survey_count = ctx.session.expedition.as_ref().map_or(0, |expedition| {
        ctx.session.site_survey_count(&expedition.site_id)
    });
    let scan_profile = ctx
        .session
        .expedition
        .as_ref()
        .map_or(WorkspaceScanProfile::Standard, |expedition| {
            expedition.scan_profile
        });
    let voyage_plan = ctx
        .session
        .expedition
        .as_ref()
        .map_or(crate::engine::VoyagePlan::Standard, |expedition| {
            expedition.voyage_plan
        });
    let drone_directive = ctx
        .session
        .expedition
        .as_ref()
        .map_or(DroneDirective::default(), |expedition| {
            expedition.drone_directive
        });
    let crew_role = ctx.session.crew_role();
    let crew_readiness = ctx.session.crew_readiness();
    if ctx.workspace_log_summary_open {
        draw_log_summary(LogSummaryView {
            entries,
            survey_count,
            scan_profile,
            voyage_plan,
            drone_directive,
            crew_role,
            crew_readiness,
            x: LOG_FRAME.x + 22.0,
            y: LOG_FRAME.y + 78.0,
        });
    } else {
        log_text(
            &format!(
                "{} FIELD EVENTS  //  TAP SUMMARY FOR OPERATION TOTALS",
                entries.len()
            ),
            Rect::new(
                LOG_FRAME.x + 22.0,
                LOG_FRAME.y + 68.0,
                LOG_FRAME.w - 44.0,
                24.0,
            ),
            15.0,
            visual_theme::cyan(),
        );
        draw_line(
            LOG_FRAME.x + 22.0,
            LOG_FRAME.y + 102.0,
            LOG_FRAME.right() - 22.0,
            LOG_FRAME.y + 102.0,
            1.0,
            visual_theme::cyan_dim(),
        );
    }
    draw_log_entries(ctx, entries);
    log_text(
        &format!(
            "PAGE {} / {}  ·  OLDER / NEWER retrieve the complete field record",
            ctx.workspace_log_page + 1,
            page_count(entries.len())
        ),
        Rect::new(
            LOG_FRAME.x + 22.0,
            LOG_FRAME.bottom() - 32.0,
            LOG_FRAME.w - 44.0,
            22.0,
        ),
        13.0,
        visual_theme::text_dim(),
    );
    draw_log_navigation(ctx, actions, entries.len());
}

struct LogSummaryView<'a> {
    entries: &'a [WorkspaceLogEntry],
    survey_count: usize,
    scan_profile: WorkspaceScanProfile,
    voyage_plan: crate::engine::VoyagePlan,
    drone_directive: DroneDirective,
    crew_role: CrewRole,
    crew_readiness: u8,
    x: f32,
    y: f32,
}

fn draw_log_summary(view: LogSummaryView<'_>) {
    let LogSummaryView {
        entries,
        survey_count,
        scan_profile,
        voyage_plan,
        drone_directive,
        crew_role,
        crew_readiness,
        x,
        y,
    } = view;
    let scans = log_event_count(entries, WorkspaceLogEvent::SectionScanned);
    let clearances = log_event_count(entries, WorkspaceLogEvent::SectionCleared);
    let drones = log_event_count(entries, WorkspaceLogEvent::DronesDeployed);
    let locks = log_event_count(entries, WorkspaceLogEvent::TargetStabilized);
    let recovered = log_event_count(entries, WorkspaceLogEvent::TargetRecovered);
    let lost = log_event_count(entries, WorkspaceLogEvent::TargetLost);
    let pulls = log_event_count(entries, WorkspaceLogEvent::ExtractionStarted);
    let cancelled = log_event_count(entries, WorkspaceLogEvent::ExtractionCancelled);
    let resets = log_event_count(entries, WorkspaceLogEvent::PowerCycled);
    let cells = log_event_count(entries, WorkspaceLogEvent::FieldPowerCellUsed);
    log_text(
        &format!(
            "ENTRIES {:02}  //  SURVEY {:02}  //  {}  //  {}  //  {}  //  DRONE {}  //  DRONES {:02}  //  SCANS {:02}  //  CLEAR {:02}",
            entries.len(),
            survey_count,
            scan_log_label(scan_profile),
            voyage_plan_log_label(voyage_plan),
            crew_log_label(crew_role, crew_readiness),
            drone_directive.short_label(),
            drones,
            scans,
            clearances
        ),
        Rect::new(x, y - 12.0, LOG_FRAME.w - 44.0, 22.0),
        13.0,
        visual_theme::cyan(),
    );
    log_text(
        &format!(
            "PULLS {:02}  //  CANCEL {:02}  //  RECOVERED {:02}  //  LOST {:02}  //  RESET {:02}  //  CELLS {:02}  //  LOCKS {:02}",
            pulls, cancelled, recovered, lost, resets, cells, locks
        ),
        Rect::new(x, y + 4.0, LOG_FRAME.w - 44.0, 22.0),
        13.0,
        visual_theme::cyan(),
    );
    draw_line(
        x,
        y + 30.0,
        LOG_FRAME.right() - 22.0,
        y + 30.0,
        1.0,
        visual_theme::cyan_dim(),
    );
}

fn scan_log_label(scan_profile: WorkspaceScanProfile) -> &'static str {
    match scan_profile {
        WorkspaceScanProfile::Standard => "SCAN STANDARD",
        WorkspaceScanProfile::Array => "SCAN ARRAY",
    }
}

fn voyage_plan_log_label(voyage_plan: crate::engine::VoyagePlan) -> String {
    format!("PLAN {}", voyage_plan.label())
}

fn crew_log_label(crew_role: CrewRole, readiness: u8) -> String {
    format!("CREW {} // READY {}%", crew_role.short_label(), readiness)
}

fn log_event_count(entries: &[WorkspaceLogEntry], event: WorkspaceLogEvent) -> usize {
    entries.iter().filter(|entry| entry.event == event).count()
}

fn draw_log_entries(ctx: &UiContext<'_>, entries: &[WorkspaceLogEntry]) {
    if entries.is_empty() {
        log_text(
            "NO FIELD EVENTS ON FILE",
            Rect::new(LOG_FRAME.x + 22.0, LOG_FRAME.y + 148.0, 700.0, 30.0),
            20.0,
            visual_theme::text(),
        );
        log_text(
            "Scan a section to begin the persistent operation record.",
            Rect::new(LOG_FRAME.x + 22.0, LOG_FRAME.y + 182.0, 700.0, 24.0),
            13.0,
            visual_theme::text_dim(),
        );
        return;
    }
    let page = ctx.workspace_log_page.min(page_count(entries.len()) - 1);
    let end = entries.len().saturating_sub(page * MAX_VISIBLE_ENTRIES);
    let first = end.saturating_sub(MAX_VISIBLE_ENTRIES);
    for (row, entry) in entries[first..end].iter().rev().enumerate() {
        let y = LOG_FRAME.y + 132.0 + row as f32 * 52.0;
        draw_log_row(
            ctx,
            entry,
            Rect::new(LOG_FRAME.x + 22.0, y, LOG_FRAME.w - 44.0, 44.0),
        );
    }
}

fn draw_log_row(ctx: &UiContext<'_>, entry: &WorkspaceLogEntry, rect: Rect) {
    draw_rectangle(rect.x, rect.y, 4.0, rect.h, event_color(entry.event));
    log_text(
        &format!("{:03}", entry.sequence),
        Rect::new(rect.x + 14.0, rect.y + 4.0, 42.0, 34.0),
        14.0,
        visual_theme::text_dim(),
    );
    log_text(
        entry.event.label(),
        Rect::new(rect.x + 66.0, rect.y + 4.0, 140.0, 34.0),
        15.0,
        event_color(entry.event),
    );
    log_text(
        &clipped(&entry_context(ctx, entry), 88),
        Rect::new(rect.x + 220.0, rect.y + 4.0, rect.w - 232.0, 34.0),
        15.0,
        visual_theme::text(),
    );
    draw_line(
        rect.x + 66.0,
        rect.bottom() - 1.0,
        rect.right(),
        rect.bottom() - 1.0,
        1.0,
        visual_theme::structure(),
    );
}

fn log_text(value: &str, rect: Rect, size: f32, color: Color) {
    visual_theme::body(value, rect, size, color);
}

fn draw_log_navigation(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, entry_count: usize) {
    let pages = page_count(entry_count);
    if pages <= 1 {
        return;
    }
    let page = ctx.workspace_log_page.min(pages - 1);
    for (next, x, label, enabled) in [
        (false, LOG_FRAME.x + 22.0, "NEWER", page > 0),
        (true, LOG_FRAME.x + 150.0, "OLDER", page + 1 < pages),
    ] {
        if button(
            ctx,
            Rect::new(x, LOG_FRAME.bottom() - 58.0, 112.0, 42.0),
            label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::WorkspaceLogPage(next));
        }
    }
}

fn entry_context(ctx: &UiContext<'_>, entry: &WorkspaceLogEntry) -> String {
    let section = ctx
        .session
        .expedition
        .as_ref()
        .and_then(|expedition| ctx.data.sites.get(&expedition.site_id))
        .and_then(|site| {
            site.sections
                .iter()
                .find(|section| section.id == entry.section_id)
        })
        .map_or_else(
            || entry.section_id.replace('_', " ").to_uppercase(),
            |section| section.display_name.to_uppercase(),
        );
    let target = entry.target_id.as_deref().and_then(|target_id| {
        ctx.data.salvage_objects.get(target_id).map(|target| {
            if target.workspace_name.is_empty() {
                target.display_name.to_uppercase()
            } else {
                target.workspace_name.to_uppercase()
            }
        })
    });
    let survey_count = ctx.session.expedition.as_ref().map_or(0, |expedition| {
        ctx.session
            .section_survey_count(&expedition.site_id, &entry.section_id)
    });
    let survey_suffix = survey_log_suffix(entry.event, survey_count);
    let scan_profile = ctx
        .session
        .expedition
        .as_ref()
        .map_or(WorkspaceScanProfile::Standard, |expedition| {
            expedition.scan_profile
        });
    let scan_suffix = scan_log_suffix(entry.event, scan_profile);
    let event_suffix = event_context_suffix(
        entry.event,
        entry
            .drone_directive
            .unwrap_or_else(|| ctx.session.workspace_drone_directive()),
    );
    match target {
        Some(target) => format!(
            "{}  //  FRAME {}{}{}{}",
            target, section, survey_suffix, scan_suffix, event_suffix
        ),
        None => format!(
            "FRAME {}{}{}{}",
            section, survey_suffix, scan_suffix, event_suffix
        ),
    }
}

fn event_context_suffix(event: WorkspaceLogEvent, drone_directive: DroneDirective) -> String {
    match event {
        WorkspaceLogEvent::PowerCycled => format!(
            "  //  FUEL -{}  //  POWER +{}",
            POWER_CYCLE_FUEL_COST, POWER_CYCLE_ENERGY_RESTORE
        ),
        WorkspaceLogEvent::FieldPowerCellUsed => format!(
            "  //  CELL -1  //  POWER +{}",
            crate::state::workspace_energy::FIELD_POWER_CELL_ENERGY_RESTORE
        ),
        WorkspaceLogEvent::DroneDirectiveChanged => {
            format!("  //  ORDER {}", drone_directive.short_label())
        }
        _ => String::new(),
    }
}

fn survey_log_suffix(event: WorkspaceLogEvent, survey_count: usize) -> String {
    if event == WorkspaceLogEvent::SectionScanned && survey_count > 0 {
        format!("  //  SURV {:02}", survey_count)
    } else {
        String::new()
    }
}

fn scan_log_suffix(event: WorkspaceLogEvent, scan_profile: WorkspaceScanProfile) -> String {
    if event == WorkspaceLogEvent::SectionScanned {
        format!("  //  {}", scan_log_label(scan_profile))
    } else {
        String::new()
    }
}

fn event_color(event: WorkspaceLogEvent) -> Color {
    match event {
        WorkspaceLogEvent::TargetLost => visual_theme::warning(),
        WorkspaceLogEvent::TargetRecovered => visual_theme::safe(),
        WorkspaceLogEvent::SectionCleared => visual_theme::safe(),
        WorkspaceLogEvent::TargetStabilized => visual_theme::with_alpha(visual_theme::safe(), 0.9),
        WorkspaceLogEvent::ExtractionStarted | WorkspaceLogEvent::ExtractionCancelled => {
            visual_theme::amber()
        }
        WorkspaceLogEvent::DroneDirectiveChanged => visual_theme::amber(),
        WorkspaceLogEvent::Departed
        | WorkspaceLogEvent::EnteredSection
        | WorkspaceLogEvent::SectionScanned
        | WorkspaceLogEvent::DronesDeployed
        | WorkspaceLogEvent::PowerCycled
        | WorkspaceLogEvent::FieldPowerCellUsed => visual_theme::cyan(),
        WorkspaceLogEvent::DronesRecalled => visual_theme::amber(),
    }
}
