//! Readable contract settlement followed by choices for the captain's cargo.

use super::*;
use crate::state::contracts::ContractObjectiveState;

mod manifest;
pub mod navigation;

pub fn draw_results(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    if ctx.voyage_archive_open {
        voyage_archive::draw_voyage_archive(ctx, actions);
        return;
    }
    let copy = &ctx.data.debrief_ui;
    panel(
        Rect::new(0.0, 84.0, 1280.0, 636.0),
        visual_theme::panel_soft(),
    );
    text(
        &copy.title,
        Rect::new(32.0, 100.0, 800.0, 40.0),
        30.0,
        visual_theme::text(),
    );
    if button(
        ctx,
        Rect::new(1028.0, 96.0, 220.0, 44.0),
        &copy.journal,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ToggleVoyageArchive);
    }
    draw_outcome_summary(ctx);
    manifest::draw_result_manifest(ctx, actions);
}

fn draw_outcome_summary(ctx: &UiContext<'_>) {
    panel(Rect::new(32.0, 156.0, 1216.0, 116.0), visual_theme::panel());
    draw_line(832.0, 174.0, 832.0, 254.0, 1.0, visual_theme::structure());
    draw_contract(ctx);
    draw_return_summary(ctx);
}

fn draw_contract(ctx: &UiContext<'_>) {
    let copy = &ctx.data.debrief_ui;
    let site_id = ctx
        .session
        .last_voyage()
        .map(|record| &record.site_id)
        .or(ctx.session.selected_site.as_ref());
    let objective = site_id.and_then(|id| ctx.session.contract_objective_status(id, ctx.data));
    let item_name = objective
        .as_ref()
        .and_then(|objective| ctx.data.salvage_objects.get(&objective.target_id))
        .map_or("", object_name);
    let private = ctx
        .session
        .last_voyage()
        .is_some_and(|record| !record.contract_accepted);
    let (heading, explanation, color) = if private {
        (
            &copy.private,
            copy.private_hint.clone(),
            visual_theme::cyan(),
        )
    } else {
        match objective.as_ref().map(|objective| objective.state) {
            Some(ContractObjectiveState::Complete) => (
                &copy.complete,
                copy.complete_hint.replace("{item}", item_name),
                visual_theme::safe(),
            ),
            Some(ContractObjectiveState::Failed) => (
                &copy.failed,
                copy.failed_hint.replace("{item}", item_name),
                visual_theme::warning(),
            ),
            Some(_) => (
                &copy.open,
                copy.open_hint.replace("{item}", item_name),
                visual_theme::amber(),
            ),
            None => (
                &copy.no_contract,
                copy.no_contract_hint.clone(),
                visual_theme::cyan(),
            ),
        }
    };
    draw_rectangle(32.0, 156.0, 4.0, 116.0, color);
    text(heading, Rect::new(52.0, 170.0, 758.0, 32.0), 27.0, color);
    text(
        &explanation,
        Rect::new(52.0, 210.0, 758.0, 32.0),
        20.0,
        visual_theme::text(),
    );
}

fn draw_return_summary(ctx: &UiContext<'_>) {
    let copy = &ctx.data.debrief_ui;
    let risk = ctx.session.last_risk.as_ref();
    let ordinary = risk.is_none_or(|risk| risk.outcome == RiskOutcome::OrdinaryReturn);
    let label = risk.map_or(copy.no_report.as_str(), |risk| {
        if ordinary {
            &copy.safe_return
        } else {
            risk_label(risk.outcome)
        }
    });
    text(
        label,
        Rect::new(868.0, 170.0, 360.0, 32.0),
        24.0,
        if ordinary {
            visual_theme::safe()
        } else {
            visual_theme::warning()
        },
    );
    let haul = transit::return_summary(ctx.session, ctx.data);
    text(
        &copy
            .recovered
            .replace("{count}", &haul.count.to_string())
            .replace("{value}", &haul.value.to_string()),
        Rect::new(868.0, 210.0, 360.0, 30.0),
        19.0,
        visual_theme::text(),
    );
    if let Some(risk) = risk.filter(|_| !ordinary) {
        text(
            &risk.explanation,
            Rect::new(868.0, 244.0, 360.0, 40.0),
            18.0,
            visual_theme::text_dim(),
        );
    }
}

fn object_name(object: &crate::data::SalvageObjectData) -> &str {
    if object.workspace_name.is_empty() {
        &object.display_name
    } else {
        &object.workspace_name
    }
}

fn text(label: &str, rect: Rect, size: f32, color: Color) {
    visual_theme::body(label, rect, size, color);
}
