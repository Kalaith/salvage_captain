//! Shared flight scene, compact HUD, and intentional arrival controls.

use super::*;
use crate::state::VoyageRecord;

mod details;
pub mod layout;
mod world;
pub use details::draw_details;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightLeg {
    Outbound,
    Homebound,
}

impl FlightLeg {
    pub fn progress(self, elapsed: f32) -> f32 {
        let duration = match self {
            Self::Outbound => travel::TRAVEL_DURATION_SECONDS,
            Self::Homebound => return_travel::RETURN_TRAVEL_DURATION_SECONDS,
        };
        (elapsed / duration).clamp(0.0, 1.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HaulSummary {
    pub count: usize,
    pub value: i64,
}

/// Use the settled voyage quote, even if market conditions have changed.
pub fn return_summary(session: &GameSession, data: &GameData) -> HaulSummary {
    if let Some(record) = session.last_voyage() {
        return HaulSummary {
            count: record.recovered_count as usize,
            value: record.recovered_value,
        };
    }
    HaulSummary {
        count: session.returned.len(),
        value: session
            .returned
            .iter()
            .filter_map(|item| {
                data.salvage_objects.get(&item.object_id).map(|object| {
                    session
                        .returned_market_quote(item, data)
                        .map_or(object.sale_value, |quote| quote.sale_value)
                })
            })
            .sum(),
    }
}

pub fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let homebound = ctx.state == GameState::ReturnTravel
        || (ctx.state == GameState::Pause && ctx.resume_state == GameState::ReturnTravel);
    crate::ui::header::draw_standard_header(
        ctx,
        actions,
        if homebound {
            "RETURN // SC-07"
        } else {
            "TRANSIT // SC-07"
        },
        if homebound {
            "DOCKING RUN"
        } else {
            "ROUTE ACTIVE"
        },
        crate::ui::header::HeaderNavigation {
            label: "",
            action: UiAction::GoToPort,
            enabled: false,
            pause_enabled: true,
        },
    );
}

pub fn draw_transit(ctx: &UiContext<'_>, leg: FlightLeg, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.transit_ui;
    let site = match leg {
        FlightLeg::Outbound => ctx
            .session
            .expedition
            .as_ref()
            .and_then(|expedition| ctx.data.sites.get(&expedition.site_id)),
        FlightLeg::Homebound => ctx
            .session
            .last_voyage()
            .and_then(|record| ctx.data.sites.get(&record.site_id)),
    };
    let elapsed = if leg == FlightLeg::Outbound {
        ctx.travel_elapsed
    } else {
        ctx.return_elapsed
    };
    let progress = leg.progress(elapsed);
    let layout = layout::transit_layout(progress, ctx.reduced_motion);
    world::draw_world(
        ctx,
        leg,
        layout,
        progress,
        site.map_or("merchant", |site| site.visual_theme.as_str()),
    );
    let title = if leg == FlightLeg::Homebound {
        &copy.yard
    } else {
        site.map_or(copy.no_transit.as_str(), |site| site.display_name.as_str())
    };
    visual_theme::body(
        title,
        Rect::new(36.0, 112.0, 850.0, 48.0),
        36.0,
        visual_theme::text(),
    );
    let subtitle = site.map_or_else(String::new, |site| {
        if leg == FlightLeg::Homebound {
            copy.return_from.replace("{site}", &site.display_name)
        } else {
            site.wreck_class.clone()
        }
    });
    visual_theme::body(
        &subtitle,
        Rect::new(38.0, 166.0, 740.0, 30.0),
        23.0,
        visual_theme::text_dim(),
    );
    if leg == FlightLeg::Outbound {
        if let Some(expedition) = &ctx.session.expedition {
            visual_theme::body(
                &format!("{} {}%", copy.danger, expedition.risk.danger_score),
                Rect::new(1030.0, 118.0, 220.0, 34.0),
                25.0,
                if expedition.risk.danger_score >= 30 {
                    visual_theme::warning()
                } else {
                    visual_theme::text()
                },
            );
        }
        if ctx.session.crew_fatigue_danger_delta() > 0 {
            visual_theme::body(
                &ctx.data.salvage_ui.crew_tired,
                Rect::new(1030.0, 160.0, 220.0, 30.0),
                21.0,
                visual_theme::warning(),
            );
        }
    } else if let Some(warning) = return_warning(ctx.session.last_voyage(), copy) {
        visual_theme::body(
            warning,
            Rect::new(38.0, 214.0, 610.0, 30.0),
            22.0,
            visual_theme::warning(),
        );
    }
    draw_progress(ctx, leg, layout, progress);
    draw_commands(ctx, leg, layout, progress, site.is_some(), actions);
}

fn draw_progress(
    ctx: &UiContext<'_>,
    leg: FlightLeg,
    layout: layout::TransitLayout,
    progress: f32,
) {
    let rect = layout.summary;
    let copy = &ctx.data.transit_ui;
    visual_theme::surface(rect);
    let phase = match (leg, progress) {
        (FlightLeg::Outbound, value) if value >= 1.0 => &copy.arrived,
        (FlightLeg::Outbound, value) if value >= 0.7 => &copy.approach,
        (FlightLeg::Outbound, value) if value >= 0.18 => &copy.cruise,
        (FlightLeg::Outbound, _) => &copy.departure,
        (FlightLeg::Homebound, value) if value >= 1.0 => &copy.return_ready,
        (FlightLeg::Homebound, value) if value >= 0.7 => &copy.return_approach,
        (FlightLeg::Homebound, value) if value >= 0.18 => &copy.return_cruise,
        _ => &copy.return_departure,
    };
    visual_theme::body(
        phase,
        Rect::new(rect.x + 20.0, rect.y + 10.0, 650.0, 32.0),
        24.0,
        visual_theme::text(),
    );
    visual_theme::body(
        &format!("{}%", (progress * 100.0).round() as i32),
        Rect::new(rect.right() - 82.0, rect.y + 10.0, 66.0, 32.0),
        24.0,
        visual_theme::text(),
    );
    draw_rectangle(
        rect.x + 20.0,
        rect.y + 50.0,
        rect.w - 40.0,
        5.0,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        rect.x + 20.0,
        rect.y + 50.0,
        (rect.w - 40.0) * progress,
        5.0,
        visual_theme::cyan(),
    );
    let summary = if leg == FlightLeg::Homebound {
        let haul = return_summary(ctx.session, ctx.data);
        if haul.count == 0 {
            copy.no_cargo.clone()
        } else {
            copy.haul
                .replace("{count}", &haul.count.to_string())
                .replace("{value}", &haul.value.to_string())
        }
    } else {
        objective_summary(ctx)
    };
    visual_theme::body(
        &summary,
        Rect::new(rect.x + 20.0, rect.y + 70.0, rect.w - 40.0, 40.0),
        23.0,
        visual_theme::text_dim(),
    );
}

fn objective_summary(ctx: &UiContext<'_>) -> String {
    let copy = &ctx.data.transit_ui;
    let Some(expedition) = &ctx.session.expedition else {
        return String::new();
    };
    if !expedition.contract_accepted {
        return copy.private_haul.clone();
    }
    let target = ctx
        .data
        .sites
        .get(&expedition.site_id)
        .and_then(|site| site.contract_target.as_deref())
        .and_then(|id| ctx.data.salvage_objects.get(id));
    target.map_or_else(String::new, |target| {
        copy.objective.replace("{target}", &target.display_name)
    })
}

fn draw_commands(
    ctx: &UiContext<'_>,
    leg: FlightLeg,
    layout: layout::TransitLayout,
    progress: f32,
    has_site: bool,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.transit_ui;
    let command_surface = if leg == FlightLeg::Homebound {
        Rect::new(
            layout.commands.x,
            layout.commands.y,
            layout.commands.w,
            80.0,
        )
    } else {
        layout.commands
    };
    visual_theme::surface(command_surface);
    let label = match leg {
        FlightLeg::Homebound => &copy.dock,
        FlightLeg::Outbound if progress >= 1.0 => &copy.continue_action,
        FlightLeg::Outbound => &copy.arrive,
    };
    if button(
        ctx,
        layout.primary,
        label,
        has_site || leg == FlightLeg::Homebound,
        ButtonTone::Primary,
    ) {
        actions.push(if leg == FlightLeg::Outbound {
            UiAction::ContinueTravel
        } else {
            UiAction::ContinueReturn
        });
    }
    if leg == FlightLeg::Outbound
        && button(
            ctx,
            layout.details,
            &copy.details,
            has_site,
            ButtonTone::Secondary,
        )
    {
        actions.push(UiAction::ToggleTransitDetails);
    }
}

fn return_warning<'a>(
    record: Option<&VoyageRecord>,
    copy: &'a crate::data::transit_ui::TransitUiCopy,
) -> Option<&'a str> {
    match record?.risk_outcome {
        RiskOutcome::OrdinaryReturn => None,
        RiskOutcome::DamagedModule => Some(&copy.damage_reported),
        RiskOutcome::LostSalvage => Some(&copy.cargo_lost),
        RiskOutcome::EmergencyRepair => Some(&copy.emergency_repair),
        RiskOutcome::ForcedAbandon => Some(&copy.cargo_abandoned),
    }
}
