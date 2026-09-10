//! Read-only Macroquad presentation that emits gameplay intents.

pub mod decision_panel;
pub mod extraction_panel;
pub mod main_menu;
pub mod notifications;
pub mod port_panel;
pub mod salvage_items;
pub mod salvage_scene;
pub mod scan_overlay;
pub mod scene_layout;
pub mod settings;
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
    ContinueGame,
    NewGame,
    BackToMainMenu,
    ExitGame,
    OpenSettings,
    CloseSettings,
    ToggleFullscreen,
    ToggleReducedMotion,
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
    SelectPortModule(String),
    TogglePortHold,
    RemoveModule(String),
    PurchaseModule(String),
    Refuel,
    Repair,
    Save,
    Load,
    TogglePause,
    ToggleStats,
}

#[derive(Clone, Copy)]
pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub state: GameState,
    pub resume_state: GameState,
    pub dragged_item: Option<&'a str>,
    pub message: &'a str,
    pub save_exists: bool,
    pub settings_open: bool,
    pub fullscreen: bool,
    pub reduced_motion: bool,
    pub interaction_enabled: bool,
    pub pointer: Pointer,
    pub pointer_started: bool,
    pub ui: &'a VirtualUi,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub travel_elapsed: f32,
    pub workspace_elapsed: f32,
    pub workspace_camera_shift: f32,
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
    pub port_selected_module: Option<&'a str>,
    pub port_hold_expanded: bool,
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
    if screen == GameState::MainMenu {
        main_menu::draw_main_menu(&ctx, &mut actions);
    } else {
        let mut scene_ctx = ctx;
        if ctx.state == GameState::Pause {
            scene_ctx.pointer = ctx.pointer.suppressed();
            scene_ctx.pointer_started = false;
            scene_ctx.interaction_enabled = false;
        }
        draw_header(&scene_ctx, &mut actions);
        match screen {
            GameState::Port => port_panel::draw_port(&scene_ctx, &mut actions),
            GameState::SiteSelection => site_cards::draw_site_selection(&scene_ctx, &mut actions),
            GameState::Travel => travel::draw_travel(&scene_ctx, &mut actions),
            GameState::SalvageWorkspace => {
                salvage_scene::draw_salvage_workspace(&scene_ctx, &mut actions)
            }
            GameState::SalvagePacking => salvage_items::draw_packing(&scene_ctx, &mut actions),
            GameState::Results => decision_panel::draw_results(&scene_ctx, &mut actions),
            GameState::MainMenu | GameState::Pause => {}
        }
    }
    if ctx.state == GameState::Pause {
        if ctx.settings_open {
            settings::draw_settings(&ctx, &mut actions);
        } else {
            notifications::draw_pause(&ctx, &mut actions);
        }
    }
    if screen != GameState::MainMenu && screen != GameState::Port {
        draw_footer(&ctx);
    }
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
    let screen = active_screen(ctx);
    if screen == GameState::Port {
        draw_port_header(ctx, actions);
        return;
    }
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
    if matches!(screen, GameState::Travel | GameState::SalvageWorkspace) {
        draw_operation_badges(ctx);
        let action_rect = Rect::new(1000.0, 20.0, 108.0, 46.0);
        let action_label = if screen == GameState::Travel {
            if ctx.travel_elapsed >= travel::TRAVEL_DURATION_SECONDS {
                "CONTINUE"
            } else {
                "ARRIVE"
            }
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
        draw_resource_value(
            330.0,
            "CREDITS",
            &format!("¢{}", ctx.session.economy.credits),
            visual_theme::safe(),
        );
        draw_resource_value(
            458.0,
            "FUEL",
            &format!(
                "{}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        );
        draw_resource_value(
            584.0,
            "HULL",
            &format!(
                "{}/{}",
                ctx.session.hull,
                ctx.session.max_hull_with_modules(ctx.data)
            ),
            if ctx.session.hull <= 3 {
                visual_theme::warning()
            } else {
                visual_theme::amber()
            },
        );
        draw_resource_value(
            712.0,
            "ALLOY",
            &ctx.session.economy.alloy.to_string(),
            visual_theme::text_dim(),
        );
        draw_resource_value(
            818.0,
            "ELEC",
            &ctx.session.economy.electronics.to_string(),
            visual_theme::text_dim(),
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

fn draw_port_header(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let width = ctx.viewport_width;
    let height = port_panel::HEADER_HEIGHT;
    draw_rectangle(
        0.0,
        0.0,
        width,
        height,
        visual_theme::with_alpha(visual_theme::panel(), 0.97),
    );
    draw_rectangle(0.0, 0.0, 6.0, height, visual_theme::amber());
    draw_line(
        22.0,
        height - 1.0,
        width - 22.0,
        height - 1.0,
        1.0,
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.85),
    );
    draw_text(
        screen_title(ctx.state, ctx.resume_state),
        26.0,
        35.0,
        16.0,
        visual_theme::text(),
    );
    draw_text(
        "SC-07  //  SHIPYARD ONLINE",
        26.0,
        49.0,
        9.0,
        visual_theme::text_dim(),
    );

    let pause_width = 74.0;
    let pause_x = (width - pause_width - 18.0).max(230.0);
    let resources_left = 238.0;
    let resources_right = pause_x - 16.0;
    let cell_width = ((resources_right - resources_left) / 5.0).max(74.0);
    let resources = [
        (
            "CREDITS",
            format!("¢{}", ctx.session.economy.credits),
            visual_theme::safe(),
        ),
        (
            "FUEL",
            format!(
                "{}/{}",
                ctx.session.economy.fuel,
                ctx.session.max_fuel(ctx.data)
            ),
            visual_theme::cyan(),
        ),
        (
            "HULL",
            format!(
                "{}/{}",
                ctx.session.hull,
                ctx.session.max_hull_with_modules(ctx.data)
            ),
            if ctx.session.hull <= 3 {
                visual_theme::warning()
            } else {
                visual_theme::amber()
            },
        ),
        (
            "ALLOY",
            ctx.session.economy.alloy.to_string(),
            visual_theme::text_dim(),
        ),
        (
            "ELEC",
            ctx.session.economy.electronics.to_string(),
            visual_theme::text_dim(),
        ),
    ];
    for (index, (label, value, color)) in resources.into_iter().enumerate() {
        draw_resource_value_at(
            resources_left + index as f32 * cell_width,
            label,
            &value,
            color,
            16.0,
            16,
        );
    }
    if button(
        ctx,
        Rect::new(pause_x, 13.0, pause_width, 32.0),
        "PAUSE",
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::TogglePause);
    }
}

fn draw_resource_value(x: f32, label: &str, value: &str, color: Color) {
    draw_resource_value_at(x, label, value, color, 20.0, 18);
}

fn draw_resource_value_at(
    x: f32,
    label: &str,
    value: &str,
    color: Color,
    top: f32,
    value_size: u16,
) {
    draw_text(label, x, top + 9.0, 9.0, visual_theme::text_dim());
    draw_text(value, x, top + 32.0, value_size as f32, color);
    draw_line(
        x - 20.0,
        top,
        x - 20.0,
        top + 38.0,
        1.0,
        visual_theme::with_alpha(visual_theme::structure_light(), 0.3),
    );
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
        Rect::new(286.0, 20.0, 260.0, 46.0),
        &clipped(&site_label, 25),
        visual_theme::with_alpha(visual_theme::amber(), 0.22),
    );
    badge(
        Rect::new(554.0, 20.0, 104.0, 46.0),
        &format!("FUEL {}", ctx.session.economy.fuel),
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.75),
    );
    badge(
        Rect::new(666.0, 20.0, 104.0, 46.0),
        &format!("HULL {}", ctx.session.hull),
        visual_theme::with_alpha(visual_theme::warning(), 0.26),
    );
    badge(
        Rect::new(778.0, 20.0, 108.0, 46.0),
        &ctx.session.workspace_energy().map_or_else(
            || "POWER --".to_owned(),
            |(remaining, capacity)| format!("POWER {remaining}/{capacity}"),
        ),
        power_badge_color(ctx.session.workspace_energy()),
    );
    badge(
        Rect::new(894.0, 20.0, 90.0, 46.0),
        &format!("CARGO {}", expedition_cargo_count(ctx)),
        visual_theme::with_alpha(visual_theme::safe(), 0.22),
    );
}

fn power_badge_color(reserve: Option<(i32, i32)>) -> Color {
    let Some((remaining, capacity)) = reserve else {
        return visual_theme::with_alpha(visual_theme::cyan_dim(), 0.7);
    };
    if remaining <= 0 || remaining * 3 <= capacity {
        visual_theme::with_alpha(visual_theme::warning(), 0.34)
    } else {
        visual_theme::with_alpha(visual_theme::cyan_dim(), 0.7)
    }
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
    let footer_message = if active_screen(ctx) == GameState::Travel && ctx.travel_elapsed >= 4.0 {
        "Arrival locked. Tap CONTINUE to enter the wreck workspace."
    } else {
        ctx.message
    };
    let text = clipped(footer_message, 92);
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
    if !ctx.interaction_enabled {
        button_rect_tone_at(rect, label, false, tone, ctx.ui.mouse_position());
        return false;
    }
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
        GameState::MainMenu => state::main_menu::TITLE,
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

pub(super) fn contract_status_label(completed: bool, failed: bool) -> &'static str {
    if completed {
        "COMPLETE"
    } else if failed {
        "FAILED"
    } else {
        "RECOVER"
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
