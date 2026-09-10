//! Compact target readout and visible extraction commands.

use super::scene_layout::SalvageLayout;
use super::visual_theme;
use super::*;
use crate::state::workspace::ExtractionPhase;

pub fn draw_target_panel(ctx: &UiContext<'_>, layout: SalvageLayout, actions: &mut Vec<UiAction>) {
    let Some(target_id) = ctx.workspace_selected_target else {
        panel(layout.target_panel, visual_theme::panel_soft());
        draw_text(
            "TARGET READOUT",
            layout.target_panel.x + 18.0,
            layout.target_panel.y + 30.0,
            17.0,
            visual_theme::text(),
        );
        draw_text(
            if ctx.workspace_scanned {
                "Select a bracketed component."
            } else {
                "SCAN reveals usable components."
            },
            layout.target_panel.x + 18.0,
            layout.target_panel.y + 76.0,
            15.0,
            visual_theme::text_dim(),
        );
        draw_text(
            "The ship will reposition automatically.",
            layout.target_panel.x + 18.0,
            layout.target_panel.y + 108.0,
            13.0,
            visual_theme::text_dim(),
        );
        return;
    };
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    panel(layout.target_panel, visual_theme::panel());
    let target_name = if target.workspace_name.is_empty() {
        target.display_name.as_str()
    } else {
        target.workspace_name.as_str()
    };
    draw_text(
        &target_name.to_uppercase(),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 29.0,
        17.0,
        visual_theme::text(),
    );
    draw_text(
        &target.category,
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 51.0,
        13.0,
        visual_theme::site_accent(&site_theme(ctx)),
    );
    draw_text(
        format!("INTEGRITY     {}%", target.integrity),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 82.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("MASS          {:.1}t", target.mass_tons),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 103.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("EST. VALUE    ~{} cr", target.sale_value),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 124.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("EXTRACTION    {:.1}s", target.extraction_duration),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 145.0,
        14.0,
        visual_theme::text(),
    );
    draw_text(
        format!("RISK          {}", risk_label_for_target(target)),
        layout.target_panel.x + 16.0,
        layout.target_panel.y + 166.0,
        14.0,
        if target.hazard.is_some() {
            visual_theme::warning()
        } else {
            visual_theme::safe()
        },
    );
    visual_theme::draw_meter(
        Rect::new(
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 181.0,
            layout.target_panel.w - 32.0,
            24.0,
        ),
        (ctx.session.tractor_capacity_tons(ctx.data) / target.mass_tons).clamp(0.0, 1.0),
        if ctx.session.tractor_capacity_tons(ctx.data) >= target.mass_tons {
            visual_theme::cyan()
        } else {
            visual_theme::warning()
        },
        &format!(
            "TRACTOR  {:.0} / {:.0}t",
            ctx.session.tractor_capacity_tons(ctx.data),
            target.mass_tons
        ),
    );
    if let Some(hazard) = &target.hazard {
        draw_text(
            format!("HAZARD  {}", hazard.replace('_', " ").to_uppercase()),
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 229.0,
            12.0,
            visual_theme::warning(),
        );
        draw_text(
            clipped(&target.hazard_consequence, 34),
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 247.0,
            11.0,
            visual_theme::text_dim(),
        );
    }
    if let Some(extraction_target) = ctx.workspace_extraction_target {
        if extraction_target == target_id {
            draw_text(
                extraction_phase_label(ctx.workspace_extraction_phase),
                layout.target_panel.x + 16.0,
                layout.target_panel.y + 278.0,
                14.0,
                visual_theme::cyan(),
            );
            visual_theme::draw_meter(
                Rect::new(
                    layout.target_panel.x + 16.0,
                    layout.target_panel.y + 288.0,
                    layout.target_panel.w - 32.0,
                    22.0,
                ),
                ctx.workspace_extraction_progress,
                visual_theme::cyan(),
                "EXTRACTION",
            );
            return;
        }
    }
    let blocked = ctx
        .session
        .extraction_block_reason(target_id, ctx.data)
        .ok()
        .flatten();
    if let Some(reason) = blocked.as_deref() {
        draw_text(
            clipped(reason, 30),
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 278.0,
            12.0,
            visual_theme::warning(),
        );
    }
    if button(
        ctx,
        Rect::new(
            layout.target_panel.x + 16.0,
            layout.target_panel.y + 292.0,
            126.0,
            44.0,
        ),
        "EXTRACT",
        blocked.is_none(),
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::Extract(target_id.to_owned()));
    }
    if button(
        ctx,
        Rect::new(
            layout.target_panel.x + 148.0,
            layout.target_panel.y + 292.0,
            110.0,
            44.0,
        ),
        "ABANDON",
        true,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::AbandonTarget);
    }
}

fn site_theme(ctx: &UiContext<'_>) -> String {
    ctx.session
        .workspace_site(ctx.data)
        .map(|site| site.visual_theme.clone())
        .unwrap_or_else(|_| "merchant".to_owned())
}

fn risk_label_for_target(target: &crate::data::SalvageObjectData) -> &'static str {
    if target.hazard.is_some() {
        "ELEVATED"
    } else if target.extraction_difficulty >= 50 {
        "MEDIUM"
    } else {
        "LOW"
    }
}

fn extraction_phase_label(phase: Option<ExtractionPhase>) -> &'static str {
    match phase {
        Some(ExtractionPhase::Alignment) => "ALIGNING EMITTER",
        Some(ExtractionPhase::Connection) => "BEAM CONNECTED",
        Some(ExtractionPhase::Strain) => "HULL UNDER STRAIN",
        Some(ExtractionPhase::Separation) => "SEPARATING",
        Some(ExtractionPhase::Retrieval) => "RETRIEVING",
        Some(ExtractionPhase::Capture) => "CAPTURE CLUNK",
        None => "STANDING BY",
    }
}
