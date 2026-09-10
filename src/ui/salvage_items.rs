//! Packing cards and direct manipulation affordances for temporary salvage.

use super::*;

pub fn draw_packing(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    panel_title(
        Rect::new(24.0, 112.0, 450.0, 494.0),
        state::salvage_packing::TITLE,
    );
    draw_ship_grid(ctx, Rect::new(56.0, 174.0, 386.0, 386.0), true, actions);
    let (site_id, risk) = ctx
        .session
        .expedition
        .as_ref()
        .map_or(("unknown", None), |expedition| {
            (expedition.site_id.as_str(), Some(&expedition.risk))
        });
    draw_text(
        format!("SITE  {}", site_id.replace('_', " ").to_uppercase()),
        54.0,
        600.0,
        15.0,
        dark::TEXT_DIM,
    );
    if let Some(risk) = risk {
        draw_text(
            format!("Previewed danger: {}%", risk.danger_score),
            260.0,
            600.0,
            15.0,
            danger_color(risk.danger_score),
        );
    }
    panel_title(Rect::new(496.0, 112.0, 760.0, 494.0), "DISCOVERED SALVAGE");
    draw_text(
        "Tap PLACE, or drag salvage onto the grid with a mouse or touch.",
        522.0,
        160.0,
        16.0,
        dark::TEXT_DIM,
    );
    if let Some(expedition) = &ctx.session.expedition {
        for (index, cargo) in expedition.cargo.iter().enumerate() {
            let y = 180.0 + index as f32 * 72.0;
            draw_cargo_card(
                ctx,
                cargo.object_id.as_str(),
                cargo.status,
                Rect::new(518.0, y, 714.0, 62.0),
                actions,
            );
        }
    }
    let pending = ctx.session.pending_count();
    if button(
        ctx,
        Rect::new(518.0, 552.0, 190.0, 38.0),
        "LEAVE ALL",
        pending > 0,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::LeaveAll);
    }
    if button(
        ctx,
        Rect::new(718.0, 552.0, 240.0, 38.0),
        "RETURN WITH HAUL",
        pending == 0,
        ButtonTone::Positive,
    ) {
        actions.push(UiAction::FinishPacking);
    }
    draw_text(
        format!("{} object(s) still need a decision", pending),
        978.0,
        576.0,
        14.0,
        if pending == 0 {
            dark::POSITIVE
        } else {
            dark::TEXT_DIM
        },
    );
}

fn draw_cargo_card(
    ctx: &UiContext<'_>,
    object_id: &str,
    status: CargoStatus,
    rect: Rect,
    actions: &mut Vec<UiAction>,
) {
    let Some(object) = ctx.data.salvage_objects.get(object_id) else {
        return;
    };
    let space_cells = object.footprint.width * object.footprint.height;
    let cell_label = if space_cells == 1 { "CELL" } else { "CELLS" };
    panel(
        rect,
        if status == CargoStatus::Packed {
            Color::new(0.13, 0.18, 0.18, 1.0)
        } else {
            Color::new(0.10, 0.13, 0.17, 1.0)
        },
    );
    draw_text(
        &object.display_name,
        rect.x + 14.0,
        rect.y + 23.0,
        18.0,
        dark::TEXT_BRIGHT,
    );
    draw_text(
        format!(
            "{}  SPACE {}x{} / {} {}  ¢{}  A{} E{}",
            object.category,
            object.footprint.width,
            object.footprint.height,
            space_cells,
            cell_label,
            object.sale_value,
            object.alloy_yield,
            object.electronics_yield
        ),
        rect.x + 14.0,
        rect.y + 46.0,
        13.0,
        dark::TEXT_DIM,
    );
    let status_text = match status {
        CargoStatus::Pending => "PENDING",
        CargoStatus::Packed => "PACKED",
        CargoStatus::LeftBehind => "LEFT",
        CargoStatus::Discarded => "DISCARDED",
        CargoStatus::Lost => "LOST",
    };
    draw_text(
        status_text,
        rect.x + 366.0,
        rect.y + 36.0,
        14.0,
        if status == CargoStatus::Packed {
            dark::POSITIVE
        } else {
            dark::WARNING
        },
    );
    let bx = rect.right() - 332.0;
    let active = matches!(status, CargoStatus::Pending | CargoStatus::Packed);
    if button(
        ctx,
        Rect::new(bx, rect.y + 11.0, 74.0, 40.0),
        "PLACE",
        active,
        ButtonTone::Primary,
    ) {
        actions.push(UiAction::AutoPlace(object_id.to_owned()));
    }
    if button(
        ctx,
        Rect::new(bx + 80.0, rect.y + 11.0, 74.0, 40.0),
        "ROTATE",
        object.rotatable && active,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Rotate(object_id.to_owned()));
    }
    if button(
        ctx,
        Rect::new(bx + 160.0, rect.y + 11.0, 78.0, 40.0),
        "LEAVE",
        active,
        ButtonTone::Warning,
    ) {
        actions.push(UiAction::Leave(object_id.to_owned()));
    }
    if button(
        ctx,
        Rect::new(bx + 244.0, rect.y + 11.0, 78.0, 40.0),
        "DROP",
        active,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::Discard(object_id.to_owned()));
    }
    let drag_zone = Rect::new(rect.x, rect.y, 355.0, rect.h);
    if status == CargoStatus::Pending
        && ctx.dragged_item.is_none()
        && ctx.pointer_started
        && ctx.pointer.pressing(drag_zone)
    {
        actions.push(UiAction::BeginDrag(object_id.to_owned()));
    }
    if ctx.dragged_item == Some(object_id) {
        draw_text(
            "DRAGGING",
            rect.x + 520.0,
            rect.y + 36.0,
            14.0,
            dark::ACCENT,
        );
    }
}
