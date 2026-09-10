//! Read-only Macroquad presentation that emits gameplay intents.

pub mod decision_panel;
pub mod notifications;
pub mod port_panel;
pub mod salvage_items;
pub mod ship_grid;
pub mod site_cards;

use crate::data::{GameData, GridPosition};
use crate::engine::{Disposition, RiskOutcome};
use crate::state;
use crate::state::{CargoStatus, GameSession, GameState};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{button_rect_tone_at, ButtonTone, VirtualUi};

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewGame,
    GoToPort,
    GoToSites,
    Depart(String),
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
    pub save_slots: &'a [String],
    pub loaded_assets: usize,
    pub pointer: Pointer,
    pub pointer_started: bool,
    pub ui: &'a VirtualUi,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    draw_header(&ctx, &mut actions);
    let screen = if ctx.state == GameState::Pause {
        ctx.resume_state
    } else {
        ctx.state
    };
    match screen {
        GameState::Port => port_panel::draw_port(&ctx, &mut actions),
        GameState::SiteSelection => site_cards::draw_site_selection(&ctx, &mut actions),
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
    panel(
        Rect::new(24.0, 18.0, 1232.0, 74.0),
        Color::new(0.06, 0.09, 0.13, 1.0),
    );
    draw_text("SALVAGE CAPTAIN", 46.0, 52.0, 28.0, dark::TEXT_BRIGHT);
    draw_text(
        screen_title(ctx.state, ctx.resume_state),
        48.0,
        76.0,
        13.0,
        dark::TEXT_DIM,
    );
    badge(
        Rect::new(360.0, 34.0, 148.0, 36.0),
        &format!("¢ {}", ctx.session.economy.credits),
        Color::new(0.15, 0.25, 0.18, 1.0),
    );
    badge(
        Rect::new(520.0, 34.0, 136.0, 36.0),
        &format!("FUEL {}", ctx.session.economy.fuel),
        Color::new(0.15, 0.23, 0.30, 1.0),
    );
    badge(
        Rect::new(668.0, 34.0, 124.0, 36.0),
        &format!("HULL {}", ctx.session.hull),
        Color::new(0.28, 0.19, 0.16, 1.0),
    );
    badge(
        Rect::new(804.0, 34.0, 178.0, 36.0),
        &format!(
            "ALLOY {}  ELEC {}",
            ctx.session.economy.alloy, ctx.session.economy.electronics
        ),
        Color::new(0.23, 0.18, 0.28, 1.0),
    );
    let port_enabled = match ctx.state {
        GameState::Port | GameState::SiteSelection => true,
        GameState::Pause => matches!(ctx.resume_state, GameState::Port | GameState::SiteSelection),
        GameState::SalvagePacking | GameState::Results => false,
    };
    if button(
        ctx,
        Rect::new(1000.0, 32.0, 100.0, 40.0),
        "PORT",
        port_enabled,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::GoToPort);
    }
    if button(
        ctx,
        Rect::new(1108.0, 32.0, 126.0, 40.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
}

fn draw_footer(ctx: &UiContext<'_>) {
    draw_text(ctx.message, 28.0, 650.0, 16.0, dark::TEXT);
    draw_text(
        "N: new game   S: save   L: load   Esc: pause   F1: debug",
        28.0,
        682.0,
        14.0,
        dark::TEXT_DIM,
    );
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
        draw_text(
            short_label(label),
            item_rect.x + 5.0,
            item_rect.y + item_rect.h * 0.55,
            12.0,
            WHITE,
        );
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

pub(super) fn clipped(value: &str, max_chars: usize) -> String {
    let mut result: String = value.chars().take(max_chars).collect();
    if value.chars().count() > max_chars {
        result.push_str("...");
    }
    result
}
