//! Touch-accessible hold destination picker over the selected wreck target.

use super::*;

pub(super) fn draw(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let Some(target_id) = ctx.workspace_selected_target else {
        return;
    };
    let Some(target) = ctx.data.salvage_objects.get(target_id) else {
        return;
    };
    let rotation = ctx.workspace_placement_rotation.unwrap_or(0);
    let frame = Rect::new(190.0, 92.0, 900.0, 606.0);
    draw_placement_frame(ctx, target, rotation, frame);
    let grid_bounds = Rect::new(240.0, 256.0, 800.0, 290.0);
    draw_placement_shape(ctx, target, rotation, grid_bounds, actions);
    draw_placement_footer(ctx, target, actions);
}

fn draw_placement_frame(
    ctx: &UiContext<'_>,
    target: &crate::data::SalvageObjectData,
    rotation: u8,
    frame: Rect,
) {
    let copy = &ctx.data.salvage_ui;
    draw_rectangle(
        0.0,
        74.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT - 74.0,
        visual_theme::with_alpha(BLACK, 0.8),
    );
    panel_title(frame, &copy.placement_title);
    visual_theme::body(
        &format!(
            "{}  /  {} x {}",
            target.display_name,
            target.footprint.rotated(rotation).width,
            target.footprint.rotated(rotation).height
        ),
        Rect::new(222.0, 144.0, 836.0, 32.0),
        24.0,
        visual_theme::text(),
    );
    visual_theme::body(
        &copy.placement_hint,
        Rect::new(222.0, 184.0, 836.0, 58.0),
        20.0,
        visual_theme::text_dim(),
    );
}

fn draw_placement_shape(
    ctx: &UiContext<'_>,
    target: &crate::data::SalvageObjectData,
    rotation: u8,
    grid_bounds: Rect,
    actions: &mut Vec<UiAction>,
) {
    draw_ship_grid(ctx, grid_bounds, false, actions);
    let shape = target.footprint.rotated(rotation);
    for y in 0..shape.height {
        for x in 0..shape.width {
            draw_rectangle(
                276.0 + x as f32 * 36.0,
                330.0 + y as f32 * 36.0,
                32.0,
                32.0,
                visual_theme::safe(),
            );
        }
    }
    draw_placement_ghost(ctx, target, rotation, grid_bounds, actions);
}

fn draw_placement_ghost(
    ctx: &UiContext<'_>,
    target: &crate::data::SalvageObjectData,
    rotation: u8,
    grid_bounds: Rect,
    actions: &mut Vec<UiAction>,
) {
    let layout = &ctx.session.ship_layout;
    let grid = ship_grid::grid_rect(grid_bounds, layout.width, layout.height);
    let Some(position) =
        ship_grid::cell_at(grid, ctx.pointer.position, layout.width, layout.height)
    else {
        return;
    };
    let valid = ctx
        .session
        .validate_workspace_placement(target.id.as_str(), position, rotation, ctx.data)
        .is_ok();
    let ghost = ship_grid::item_rect(
        grid,
        position,
        target.footprint.rotated(rotation),
        layout.width,
        layout.height,
    );
    let color = if valid {
        visual_theme::safe()
    } else {
        visual_theme::warning()
    };
    draw_rectangle(
        ghost.x,
        ghost.y,
        ghost.w,
        ghost.h,
        visual_theme::with_alpha(color, 0.25),
    );
    draw_rectangle_lines(ghost.x, ghost.y, ghost.w, ghost.h, 3.0, color);
    if ctx.interaction_enabled && ctx.pointer.released_on(grid) {
        actions.push(UiAction::PlaceWorkspaceTarget(position));
    }
}

fn draw_placement_footer(
    ctx: &UiContext<'_>,
    target: &crate::data::SalvageObjectData,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.salvage_ui;
    visual_theme::body(
        ctx.message,
        Rect::new(222.0, 580.0, 836.0, 40.0),
        18.0,
        visual_theme::amber(),
    );
    if button(
        ctx,
        Rect::new(222.0, 632.0, 220.0, 46.0),
        &copy.rotate_placement,
        target.rotatable,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::RotateWorkspacePlacement);
    }
    if button(
        ctx,
        Rect::new(780.0, 632.0, 278.0, 46.0),
        &copy.cancel_placement,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::CancelWorkspacePlacement);
    }
}
