//! Read-only Macroquad presentation that emits gameplay intents.

pub mod condition_visual;
pub mod crew_panel;
pub mod decision_panel;
pub mod drone_command;
pub mod drone_visual;
pub mod extraction_panel;
pub mod hazard_visual;
pub(crate) mod header;
pub mod loadout_panel;
pub mod main_menu;
pub mod notifications;
pub mod port_panel;
pub mod return_travel;
pub mod salvage_items;
pub mod salvage_scene;
pub mod scan_overlay;
pub mod scene_layout;
pub mod section_nav;
pub mod service_panel;
pub mod settings;
pub mod ship_grid;
pub mod ship_visual;
pub mod site_cards;
pub mod transfer_hardware;
pub mod transit;
pub mod travel;
pub mod visual_theme;
pub mod voyage_archive;
pub mod workspace_log;
mod workspace_placement;
pub mod wreck_silhouette;
pub mod wreck_visual;

use crate::data::{GameData, GridPosition};
use crate::engine::{Disposition, RiskOutcome, VoyagePlan};
use crate::state::{CargoStatus, GameSession, GameState};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{
    button_rect_enabled_styled_ex_at, ButtonTone, ButtonTrigger, TextStyle, VirtualUi,
};
pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

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
    DiscoverWreck(crate::data::discovery::LeadKind),
    WreckBoard(site_cards::BoardAction),
    WreckSelection(site_cards::SelectionAction),
    Depart(String),
    DepartPrivate(String),
    DepartInsured(String),
    BuyReconnaissance(String),
    CycleVoyagePlan,
    CycleCrew,
    RestCrew,
    TrainCrew,
    CycleReturnPolicy,
    ContinueTravel,
    Scan,
    PowerCycle,
    CycleDroneDirective,
    SelectSection(String),
    SelectTarget(String),
    Extract(String),
    PlaceWorkspaceTarget(GridPosition),
    RotateWorkspacePlacement,
    CancelWorkspacePlacement,
    Stabilize(String),
    AbandonTarget,
    CancelExtraction,
    ReturnFromWorkspace,
    ViewInventory,
    ContinueReturn,
    ToggleWorkspaceLog,
    ToggleTargetDetails,
    ToggleTransitDetails,
    BeginDrag(String),
    DropDragged(GridPosition, u8),
    CancelDrag,
    Rotate(String),
    Leave(String),
    Discard(String),
    LeaveAll,
    ReturnWithHaul,
    ReturnToWorkspace,
    Disposition(String, Disposition),
    ManifestPage(bool),
    SelectPortModule(String),
    TogglePortHold,
    UpgradeCargoBay,
    SelectPortTab(port_panel::PortTab),
    PortStockPage(bool),
    Service(crate::state::maintenance::ServicePlan),
    BuyFieldPowerCell,
    FabricateFieldPowerCell,
    UseFieldPowerCell,
    ToggleLoadoutPanel,
    StoreLoadout(usize),
    ApplyLoadout(usize),
    RemoveModule(String),
    PurchaseModule(String),
    Refuel,
    Repair,
    RefineAlloy,
    RefineElectronics,
    Save,
    Load,
    ToggleVoyageArchive,
    Archive(voyage_archive::ArchiveAction),
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
    pub workspace_placement_rotation: Option<u8>,
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
    pub return_elapsed: f32,
    pub workspace_elapsed: f32,
    pub workspace_camera_shift: f32,
    pub workspace_arrival_flash: f32,
    pub workspace_log_open: bool,
    pub target_details_open: bool,
    pub transit_details_open: bool,
    pub manifest_page: decision_panel::navigation::ManifestPage,
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
    pub voyage_plan: VoyagePlan,
    pub wreck_selection: &'a site_cards::WreckSelection,
    pub port_hold_expanded: bool,
    pub port_tab: port_panel::PortTab,
    pub port_stock_page: usize,
    pub port_loadouts_open: bool,
    pub voyage_archive_open: bool,
    pub voyage_archive: crate::ui::voyage_archive::ArchiveState,
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
        GameState::ReturnTravel => ctx.return_elapsed,
        GameState::SalvageWorkspace => ctx.workspace_elapsed,
        _ => 0.0,
    };
    if matches!(screen, GameState::Travel | GameState::ReturnTravel) {
        clear_background(visual_theme::space());
    } else {
        visual_theme::draw_space_field(elapsed);
    }
    if screen == GameState::MainMenu {
        main_menu::draw_main_menu(&ctx, &mut actions);
    } else {
        let mut scene_ctx = ctx;
        if ctx.state == GameState::Pause {
            scene_ctx.pointer = ctx.pointer.suppressed();
            scene_ctx.pointer_started = false;
            scene_ctx.interaction_enabled = false;
        }
        if ((ctx.target_details_open || ctx.workspace_placement_rotation.is_some())
            && screen == GameState::SalvageWorkspace)
            || (ctx.transit_details_open && screen == GameState::Travel)
        {
            let mut header_ctx = scene_ctx;
            header_ctx.interaction_enabled = false;
            header::draw_header(&header_ctx, &mut actions);
        } else {
            header::draw_header(&scene_ctx, &mut actions);
        }
        draw_screen(scene_ctx, screen, &mut actions);
    }
    if ctx.state == GameState::Pause {
        if ctx.settings_open {
            settings::draw_settings(&ctx, &mut actions);
        } else {
            notifications::draw_pause(&ctx, &mut actions);
        }
    }
    if !matches!(
        screen,
        GameState::SiteSelection
            | GameState::MainMenu
            | GameState::Port
            | GameState::SalvageWorkspace
            | GameState::Travel
            | GameState::ReturnTravel
    ) && !(screen == GameState::Results
        && (ctx.voyage_archive_open
            || ctx.message == crate::game::prompts::state_prompt(GameState::Results)))
    {
        header::draw_footer(&ctx);
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
    draw_grid_cells(ctx, layout_rect);
    draw_grid_items(ctx, layout_rect);
    if interactive {
        draw_dragged_item(ctx, layout_rect, actions);
    }
    draw_grid_usage(ctx, rect);
}

fn draw_grid_cells(ctx: &UiContext<'_>, layout_rect: Rect) {
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
}

fn draw_grid_items(ctx: &UiContext<'_>, layout_rect: Rect) {
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
}

fn draw_dragged_item(ctx: &UiContext<'_>, layout_rect: Rect, actions: &mut Vec<UiAction>) {
    let Some(object_id) = ctx.dragged_item else {
        return;
    };
    let Some(position) = ship_grid::cell_at(
        layout_rect,
        ctx.pointer.position,
        ctx.session.ship_layout.width,
        ctx.session.ship_layout.height,
    ) else {
        if ctx.pointer.released {
            actions.push(UiAction::CancelDrag);
        }
        return;
    };
    let rotation = dragged_rotation(ctx, object_id);
    if ctx.pointer.released {
        actions.push(UiAction::DropDragged(position, rotation));
    }
    if let Some(object) = ctx.data.salvage_objects.get(object_id) {
        draw_dragged_ghost(ctx, layout_rect, position, object, rotation);
    }
}

fn draw_dragged_ghost(
    ctx: &UiContext<'_>,
    layout_rect: Rect,
    position: GridPosition,
    object: &crate::data::SalvageObjectData,
    rotation: u8,
) {
    let ghost = ship_grid::item_rect(
        layout_rect,
        position,
        object.footprint.rotated(rotation),
        ctx.session.ship_layout.width,
        ctx.session.ship_layout.height,
    );
    draw_rectangle_lines(
        ghost.x + 2.0,
        ghost.y + 2.0,
        ghost.w - 4.0,
        ghost.h - 4.0,
        3.0,
        visual_theme::cyan(),
    );
}

fn draw_grid_usage(ctx: &UiContext<'_>, rect: Rect) {
    draw_text(
        format!(
            "USED {}/{} CELLS",
            ctx.session.ship_layout.occupied_cells(),
            ctx.session.ship_layout.width * ctx.session.ship_layout.height
        ),
        rect.x,
        rect.bottom() + 22.0,
        14.0,
        visual_theme::text_dim(),
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
    let style = visual_theme::button_style(tone, enabled);
    let interactive = enabled && ctx.interaction_enabled;
    let activated = button_rect_enabled_styled_ex_at(
        rect,
        label,
        interactive,
        &style,
        TextStyle::new(20.0, style.text_color),
        ButtonTrigger::Release,
        ctx.ui.mouse_position(),
    );
    interactive && (activated || ctx.pointer.released_on(rect))
}

pub(super) fn panel(rect: Rect, color: Color) {
    draw_rectangle(
        rect.x + 4.0,
        rect.y + 6.0,
        rect.w,
        rect.h,
        visual_theme::with_alpha(BLACK, 0.34),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        visual_theme::structure(),
    );
    draw_line(
        rect.x + 1.0,
        rect.y + 1.0,
        rect.right() - 1.0,
        rect.y + 1.0,
        1.0,
        visual_theme::cyan_dim(),
    );
}

pub(super) fn panel_title(rect: Rect, title: &str) {
    panel(rect, visual_theme::panel());
    draw_rectangle(rect.x, rect.y, rect.w, 42.0, visual_theme::structure_dark());
    draw_rectangle(rect.x, rect.y, 5.0, 42.0, visual_theme::amber());
    visual_theme::body(
        title,
        Rect::new(rect.x + 18.0, rect.y + 8.0, rect.w - 28.0, 30.0),
        18.0,
        visual_theme::text(),
    );
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
        visual_theme::safe()
    } else if danger < 60 {
        visual_theme::amber()
    } else {
        visual_theme::warning()
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

fn draw_screen(scene_ctx: UiContext<'_>, screen: GameState, actions: &mut Vec<UiAction>) {
    match screen {
        GameState::Port => draw_port_scene(scene_ctx, actions),
        GameState::SiteSelection => site_cards::draw_site_selection(&scene_ctx, actions),
        GameState::Travel => {
            if scene_ctx.transit_details_open {
                let mut blocked_ctx = scene_ctx;
                blocked_ctx.pointer = scene_ctx.pointer.suppressed();
                blocked_ctx.interaction_enabled = false;
                travel::draw_travel(&blocked_ctx, actions);
                transit::draw_details(&scene_ctx, actions);
            } else {
                travel::draw_travel(&scene_ctx, actions);
            }
        }
        GameState::ReturnTravel => return_travel::draw_return_travel(&scene_ctx, actions),
        GameState::SalvageWorkspace => {
            if scene_ctx.workspace_placement_rotation.is_some() {
                let mut blocked_ctx = scene_ctx;
                blocked_ctx.pointer = scene_ctx.pointer.suppressed();
                blocked_ctx.pointer_started = false;
                blocked_ctx.interaction_enabled = false;
                salvage_scene::draw_salvage_workspace(&blocked_ctx, actions);
                workspace_placement::draw(&scene_ctx, actions);
            } else if scene_ctx.target_details_open {
                let mut blocked_ctx = scene_ctx;
                blocked_ctx.pointer = scene_ctx.pointer.suppressed();
                blocked_ctx.interaction_enabled = false;
                salvage_scene::draw_salvage_workspace(&blocked_ctx, actions);
                extraction_panel::draw_details(&scene_ctx, actions);
            } else if scene_ctx.workspace_log_open {
                let mut blocked_ctx = scene_ctx;
                blocked_ctx.pointer = scene_ctx.pointer.suppressed();
                blocked_ctx.pointer_started = false;
                blocked_ctx.interaction_enabled = false;
                salvage_scene::draw_salvage_workspace(&blocked_ctx, actions);
                workspace_log::draw_workspace_log(&scene_ctx);
            } else {
                salvage_scene::draw_salvage_workspace(&scene_ctx, actions);
            }
        }
        GameState::CargoInventory => salvage_items::draw_packing(&scene_ctx, actions),
        GameState::Results => decision_panel::draw_results(&scene_ctx, actions),
        GameState::MainMenu | GameState::Pause => {}
    }
}

fn draw_port_scene(scene_ctx: UiContext<'_>, actions: &mut Vec<UiAction>) {
    if scene_ctx.voyage_archive_open {
        let mut blocked_ctx = scene_ctx;
        blocked_ctx.pointer = scene_ctx.pointer.suppressed();
        blocked_ctx.pointer_started = false;
        blocked_ctx.interaction_enabled = false;
        port_panel::draw_port(&blocked_ctx, actions);
        voyage_archive::draw_voyage_archive(&scene_ctx, actions);
    } else {
        port_panel::draw_port(&scene_ctx, actions);
    }
}
