//! Read-only Macroquad presentation that emits gameplay intents.

pub mod decision_panel;
pub mod extraction_panel;
pub mod notifications;
pub mod port_panel;
pub mod salvage_items;
pub mod salvage_scene;
pub mod scan_overlay;
pub mod scene_layout;
pub mod ship_grid;
pub mod ship_visual;
pub mod site_cards;
pub mod travel;
pub mod visual_theme;
pub mod wreck_visual;

use crate::data::{GameData, GridPosition};
use crate::engine::{Disposition, RiskOutcome};
use crate::state;
use crate::state::{CargoStatus, GameSession, GameState};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{button_rect_tone_at, ButtonTone, VirtualUi};

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewGame,
    GoToPort,
    GoToSites,
    Depart(String),
    ContinueTravel,
    Scan,
    SelectSection(String),
    SelectTarget(String),
    Extract(String),
    AbandonTarget,
    CancelExtraction,
    ReturnFromWorkspace,
    AutoPlace(String),
    BeginDrag(String),
    DropDragged(GridPosition, u8),
    CancelDrag,
    Rotate(String),
    Leave(String),
    Discard(String),
    LeaveAll,
    FinishPacking,
    Disposition(String, Disposition),
    RemoveModule(String),
    PurchaseModule(String),
    Refuel,
    Repair,
    Save,
    Load,
    TogglePause,
    ToggleStats,
}

pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub state: GameState,
    pub resume_state: GameState,
    pub dragged_item: Option<&'a str>,
    pub message: &'a str,
    pub save_exists: bool,
    pub pointer: Pointer,
    pub pointer_started: bool,
    pub ui: &'a VirtualUi,
    pub travel_elapsed: f32,
    pub workspace_elapsed: f32,
    pub workspace_scanned: bool,
    pub workspace_scan_progress: f32,
    pub workspace_selected_target: Option<&'a str>,
    pub workspace_extraction_target: Option<&'a str>,
    pub workspace_extraction_progress: f32,
    pub workspace_extraction_phase: Option<crate::state::workspace::ExtractionPhase>,
    pub workspace_risk: Option<&'a crate::engine::WorkspaceRiskReport>,
    pub workspace_notice: &'a str,
    pub workspace_notice_warning: bool,
    pub workspace_notice_timer: f32,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let screen = if ctx.state == GameState::Pause {
        ctx.resume_state
    } else {
        ctx.state
    };
    let elapsed = match screen {
        GameState::Travel => ctx.travel_elapsed,
        GameState::SalvageWorkspace => ctx.workspace_elapsed,
        _ => 0.0,
    };
    visual_theme::draw_space_field(elapsed);
    draw_header(&ctx, &mut actions);
    match screen {
        GameState::Port => port_panel::draw_port(&ctx, &mut actions),
        GameState::SiteSelection => site_cards::draw_site_selection(&ctx, &mut actions),
        GameState::Travel => travel::draw_travel(&ctx, &mut actions),
        GameState::SalvageWorkspace => salvage_scene::draw_salvage_workspace(&ctx, &mut actions),
        GameState::SalvagePacking => salvage_items::draw_packing(&ctx, &mut actions),
        GameState::Results => decision_panel::draw_results(&ctx, &mut actions),
        GameState::Pause => {}
    }
    if ctx.state == GameState::Pause {
        notifications::draw_pause(&ctx, &mut actions);
    }
    draw_footer(&ctx);
    actions
}

pub fn keyboard_actions() -> Vec<UiAction> {
    let mut actions = Vec::new();
    if is_key_pressed(KeyCode::N) {
        actions.push(UiAction::NewGame);
    }
    if is_key_pressed(KeyCode::F1) {
        actions.push(UiAction::ToggleStats);
    }
    actions
}

fn draw_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        84.0,
        visual_theme::with_alpha(visual_theme::panel(), 0.94),
    );
    draw_rectangle(0.0, 0.0, 7.0, 84.0, visual_theme::amber());
    draw_line(
        24.0,
        83.0,
        LOGICAL_WIDTH - 24.0,
        83.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.8),
    );
    draw_text(
        screen_title(ctx.state, ctx.resume_state),
        32.0,
        48.0,
        18.0,
        visual_theme::text(),
    );
    let screen = active_screen(ctx);
    if matches!(screen, GameState::Travel | GameState::SalvageWorkspace) {
        draw_operation_badges(ctx);
        let action_rect = Rect::new(1000.0, 20.0, 108.0, 46.0);
        let action_label = if screen == GameState::Travel {
            "ARRIVE"
        } else {
            "RETURN"
        };
        let action_enabled = screen == GameState::Travel
            || ctx.workspace_extraction_target.is_none()
            || ctx.workspace_extraction_progress >= 1.0;
        if button(
            ctx,
            action_rect,
            action_label,
            action_enabled,
            ButtonTone::Positive,
        ) {
            actions.push(if screen == GameState::Travel {
                UiAction::ContinueTravel
            } else {
                UiAction::ReturnFromWorkspace
            });
        }
    } else {
        badge(
            Rect::new(306.0, 20.0, 156.0, 46.0),
            &format!("¢ {}", ctx.session.economy.credits),
            visual_theme::with_alpha(visual_theme::safe(), 0.22),
        );
        badge(
            Rect::new(474.0, 20.0, 148.0, 46.0),
            &format!(
                "FUEL {}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::with_alpha(visual_theme::cyan_dim(), 0.75),
        );
        badge(
            Rect::new(634.0, 20.0, 144.0, 46.0),
            &format!(
                "HULL {}/{}",
                ctx.session.hull,
                ctx.session.max_hull_with_modules(ctx.data)
            ),
            visual_theme::with_alpha(visual_theme::warning(), 0.26),
        );
        badge(
            Rect::new(790.0, 20.0, 194.0, 46.0),
            &format!(
                "ALLOY {}  ELEC {}",
                ctx.session.economy.alloy, ctx.session.economy.electronics
            ),
            visual_theme::with_alpha(visual_theme::amber(), 0.22),
        );
        let port_enabled = matches!(screen, GameState::Port | GameState::SiteSelection);
        if button(
            ctx,
            Rect::new(1000.0, 20.0, 108.0, 46.0),
            "PORT",
            port_enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::GoToPort);
        }
    }
    if button(
        ctx,
        Rect::new(1120.0, 20.0, 136.0, 46.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
}

fn active_screen(ctx: &UiContext<'_>) -> GameState {
    if ctx.state == GameState::Pause {
        ctx.resume_state
    } else {
        ctx.state
    }
}

fn draw_operation_badges(ctx: &UiContext<'_>) {
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
        Rect::new(306.0, 20.0, 302.0, 46.0),
        &clipped(&site_label, 29),
        visual_theme::with_alpha(visual_theme::amber(), 0.22),
    );
    badge(
        Rect::new(620.0, 20.0, 122.0, 46.0),
        &format!("FUEL {}", ctx.session.economy.fuel),
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.75),
    );
    badge(
        Rect::new(754.0, 20.0, 108.0, 46.0),
        &format!("HULL {}", ctx.session.hull),
        visual_theme::with_alpha(visual_theme::warning(), 0.26),
    );
    badge(
        Rect::new(874.0, 20.0, 110.0, 46.0),
        &format!("CARGO {}", expedition_cargo_count(ctx)),
        visual_theme::with_alpha(visual_theme::safe(), 0.22),
    );
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

fn draw_footer(ctx: &UiContext<'_>) {
    if ctx.message.is_empty() {
        return;
    }
    let text = clipped(ctx.message, 92);
    let measured = measure_text(&text, None, 16, 1.0).width;
    let rect = Rect::new(
        ((LOGICAL_WIDTH - measured - 36.0) * 0.5).max(24.0),
        LOGICAL_HEIGHT - 42.0,
        measured + 36.0,
        28.0,
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        visual_theme::with_alpha(visual_theme::panel(), 0.92),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        visual_theme::cyan_dim(),
    );
    draw_text(&text, rect.x + 18.0, rect.y + 19.0, 16.0, dark::TEXT);
}

pub(super) fn draw_ship_grid(
    ctx: &UiContext<'_>,
    rect: Rect,
    interactive: bool,
    actions: &mut Vec<UiAction>,
) {
    let layout_rect = ship_grid::grid_rect(
        rect,
        ctx.session.ship_layout.width,
        ctx.session.ship_layout.height,
    );
    for y in 0..ctx.session.ship_layout.height {
        for x in 0..ctx.session.ship_layout.width {
            let cell = ship_grid::cell_rect(
                layout_rect,
                x,
                y,
                ctx.session.ship_layout.width,
                ctx.session.ship_layout.height,
            );
            draw_rectangle(
                cell.x,
                cell.y,
                cell.w - 2.0,
                cell.h - 2.0,
                Color::new(0.08, 0.12, 0.16, 1.0),
            );
            draw_rectangle_lines(
                cell.x,
                cell.y,
                cell.w - 2.0,
                cell.h - 2.0,
                1.0,
                Color::new(0.22, 0.32, 0.40, 1.0),
            );
        }
    }
    for item in &ctx.session.ship_layout.placements {
        let shape = item.footprint.rotated(item.rotation);
        let item_rect = ship_grid::item_rect(
            layout_rect,
            item.position,
            shape,
            ctx.session.ship_layout.width,
            ctx.session.ship_layout.height,
        );
        let fill = if item.permanent {
            Color::new(0.12, 0.32, 0.44, 1.0)
        } else {
            Color::new(0.55, 0.30, 0.12, 1.0)
        };
        draw_rectangle(
            item_rect.x + 2.0,
            item_rect.y + 2.0,
            item_rect.w - 4.0,
            item_rect.h - 4.0,
            fill,
        );
        let label = item.id.strip_prefix("cargo:").unwrap_or(&item.id);
        let compact = layout_rect.w / (ctx.session.ship_layout.width as f32) < 36.0
            || layout_rect.h / (ctx.session.ship_layout.height as f32) < 18.0;
        if !compact {
            draw_text(
                short_label(label),
                item_rect.x + 5.0,
                item_rect.y + item_rect.h * 0.55,
                12.0,
                WHITE,
            );
        }
    }
    if interactive {
        if let Some(object_id) = ctx.dragged_item {
            let pointer = ctx.pointer.position;
            if let Some(position) = ship_grid::cell_at(
                layout_rect,
                pointer,
                ctx.session.ship_layout.width,
                ctx.session.ship_layout.height,
            ) {
                if ctx.pointer.released {
                    actions.push(UiAction::DropDragged(
                        position,
                        dragged_rotation(ctx, object_id),
                    ));
                }
                if let Some(object) = ctx.data.salvage_objects.get(object_id) {
                    let ghost = ship_grid::item_rect(
                        layout_rect,
                        position,
                        object.footprint.rotated(dragged_rotation(ctx, object_id)),
                        ctx.session.ship_layout.width,
                        ctx.session.ship_layout.height,
                    );
                    draw_rectangle_lines(
                        ghost.x + 2.0,
                        ghost.y + 2.0,
                        ghost.w - 4.0,
                        ghost.h - 4.0,
                        3.0,
                        dark::ACCENT,
                    );
                }
            } else if ctx.pointer.released {
                actions.push(UiAction::CancelDrag);
            }
        }
    }
    draw_text(
        format!(
            "USED {}/{} CELLS",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        rect.x,
        rect.bottom() + 22.0,
        14.0,
        dark::TEXT_DIM,
    );
}

fn dragged_rotation(ctx: &UiContext<'_>, object_id: &str) -> u8 {
    ctx.session
        .expedition
        .as_ref()
        .and_then(|expedition| {
            expedition
                .cargo
                .iter()
                .find(|cargo| cargo.object_id == object_id)
        })
        .map_or(0, |cargo| cargo.rotation % 2)
}

pub(super) fn button(
    ctx: &UiContext<'_>,
    rect: Rect,
    label: &str,
    enabled: bool,
    tone: ButtonTone,
) -> bool {
    button_rect_tone_at(rect, label, enabled, tone, ctx.ui.mouse_position())
        || (enabled && ctx.pointer.released_on(rect))
}

pub(super) fn panel(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(0.22, 0.34, 0.42, 1.0),
    );
}

pub(super) fn panel_title(rect: Rect, title: &str) {
    panel(rect, Color::new(0.07, 0.11, 0.15, 1.0));
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        42.0,
        Color::new(0.10, 0.16, 0.20, 1.0),
    );
    draw_text(title, rect.x + 18.0, rect.y + 28.0, 18.0, dark::TEXT_BRIGHT);
}

pub(super) fn badge(rect: Rect, text: &str, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(0.32, 0.48, 0.56, 1.0),
    );
    draw_text(text, rect.x + 10.0, rect.y + 24.0, 15.0, dark::TEXT_BRIGHT);
}

pub(super) fn screen_title(state: GameState, resume: GameState) -> &'static str {
    match if state == GameState::Pause {
        resume
    } else {
        state
    } {
        GameState::Port => state::port::TITLE,
        GameState::SiteSelection => state::site_selection::TITLE,
        GameState::Travel => "TRANSIT",
        GameState::SalvageWorkspace => "SALVAGE WORKSPACE",
        GameState::SalvagePacking => state::salvage_packing::TITLE,
        GameState::Results => state::results::TITLE,
        GameState::Pause => state::pause::TITLE,
    }
}

pub(super) fn risk_label(outcome: RiskOutcome) -> &'static str {
    match outcome {
        RiskOutcome::OrdinaryReturn => "ORDINARY RETURN",
        RiskOutcome::DamagedModule => "DAMAGED MODULE",
        RiskOutcome::LostSalvage => "LOST SALVAGE",
        RiskOutcome::EmergencyRepair => "EMERGENCY REPAIR",
        RiskOutcome::ForcedAbandon => "FORCED ABANDON",
    }
}

pub(super) fn danger_color(danger: i32) -> Color {
    if danger < 30 {
        dark::POSITIVE
    } else if danger < 60 {
        dark::WARNING
    } else {
        dark::NEGATIVE
    }
}

pub(super) fn short_label(value: &str) -> String {
    value
        .replace('_', " ")
        .chars()
        .take(11)
        .collect::<String>()
        .to_uppercase()
}

pub(super) fn hazard_label(value: &str) -> String {
    value.replace('_', " ").to_uppercase()
}

pub(super) fn clipped(value: &str, max_chars: usize) -> String {
    let mut result: String = value.chars().take(max_chars).collect();
    if value.chars().count() > max_chars {
        result.push_str("...");
    }
    result
}
