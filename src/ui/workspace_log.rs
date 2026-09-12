//! Persistent operation record for the wreck currently under the workboat.

use super::visual_theme;
use super::*;
use crate::state::workspace_energy::{POWER_CYCLE_ENERGY_RESTORE, POWER_CYCLE_FUEL_COST};
use crate::state::{
    CrewRole, DroneDirective, WorkspaceLogEntry, WorkspaceLogEvent, WorkspaceScanProfile,
};

const LOG_FRAME: Rect = Rect::new(154.0, 108.0, 972.0, 552.0);
const MAX_VISIBLE_ENTRIES: usize = 9;

pub fn draw_workspace_log(ctx: &UiContext<'_>) {
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
    draw_text(
        format!("SALVAGE OPERATION LOG  //  {}", site_name.to_uppercase()),
        LOG_FRAME.x + 18.0,
        LOG_FRAME.y + 30.0,
        18.0,
        visual_theme::text(),
    );
    draw_text(
        "PERSISTENT FIELD RECORD",
        LOG_FRAME.right() - 184.0,
        LOG_FRAME.y + 29.0,
        10.0,
        visual_theme::amber(),
    );

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
    draw_log_entries(ctx, entries);
    draw_text(
        "Tap LOG in the header to close this record and return to the workspace.",
        LOG_FRAME.x + 22.0,
        LOG_FRAME.bottom() - 20.0,
        12.0,
        visual_theme::text_dim(),
    );
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
    draw_text(
        format!(
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
        x,
        y,
        13.0,
        visual_theme::cyan(),
    );
    draw_text(
        format!(
            "PULLS {:02}  //  CANCEL {:02}  //  RECOVERED {:02}  //  LOST {:02}  //  RESET {:02}  //  CELLS {:02}  //  LOCKS {:02}",
            pulls, cancelled, recovered, lost, resets, cells, locks
        ),
        x,
        y + 16.0,
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
        draw_text(
            "NO FIELD EVENTS ON FILE",
            LOG_FRAME.x + 22.0,
            LOG_FRAME.y + 164.0,
            20.0,
            visual_theme::text(),
        );
        draw_text(
            "Scan a section to begin the persistent operation record.",
            LOG_FRAME.x + 22.0,
            LOG_FRAME.y + 192.0,
            13.0,
            visual_theme::text_dim(),
        );
        return;
    }
    let first = entries.len().saturating_sub(MAX_VISIBLE_ENTRIES);
    for (row, entry) in entries[first..].iter().rev().enumerate() {
        let y = LOG_FRAME.y + 124.0 + row as f32 * 40.0;
        draw_log_row(
            ctx,
            entry,
            Rect::new(LOG_FRAME.x + 16.0, y, LOG_FRAME.w - 32.0, 32.0),
        );
    }
    if first > 0 {
        draw_text(
            format!("... {} earlier event(s) retained", first),
            LOG_FRAME.right() - 230.0,
            LOG_FRAME.y + 100.0,
            10.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_log_row(ctx: &UiContext<'_>, entry: &WorkspaceLogEntry, rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        visual_theme::with_alpha(visual_theme::panel(), 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        visual_theme::with_alpha(event_color(entry.event), 0.7),
    );
    draw_text(
        format!("{:03}", entry.sequence),
        rect.x + 12.0,
        rect.y + 21.0,
        12.0,
        visual_theme::text_dim(),
    );
    draw_text(
        entry.event.label(),
        rect.x + 58.0,
        rect.y + 21.0,
        12.0,
        event_color(entry.event),
    );
    draw_text(
        clipped(&entry_context(ctx, entry), 72),
        rect.x + 220.0,
        rect.y + 21.0,
        12.0,
        visual_theme::text(),
    );
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
