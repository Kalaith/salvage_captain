//! Readable, paged inventory rows with only cargo information and actions.

use super::*;
use crate::state::CargoItem;
use crate::ui::decision_panel::navigation::{ManifestPage, PAGE_SIZE};

pub(super) fn draw_manifest(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.salvage_ui.inventory_copy;
    visual_theme::surface(Rect::new(496.0, 84.0, 760.0, 588.0));
    visual_theme::body(
        &copy.cargo_title,
        Rect::new(516.0, 102.0, 700.0, 34.0),
        26.0,
        visual_theme::text(),
    );
    let count = ctx.session.inventory_cargo().count();
    let page = ctx.manifest_page.current(count);
    if count == 0 {
        visual_theme::body(
            &copy.empty,
            Rect::new(536.0, 272.0, 680.0, 40.0),
            30.0,
            visual_theme::text(),
        );
        visual_theme::body(
            &copy.empty_hint,
            Rect::new(536.0, 322.0, 660.0, 60.0),
            21.0,
            visual_theme::text_dim(),
        );
    } else {
        for (index, cargo) in ctx
            .session
            .inventory_cargo()
            .enumerate()
            .skip(page * PAGE_SIZE)
            .take(PAGE_SIZE)
        {
            draw_cargo_card(ctx, cargo, index, actions);
        }
        draw_pagination(ctx, actions, count);
    }
    draw_navigation(ctx, actions);
}

fn draw_cargo_card(
    ctx: &UiContext<'_>,
    cargo: &CargoItem,
    index: usize,
    actions: &mut Vec<UiAction>,
) {
    let Some(object) = ctx.data.salvage_objects.get(&cargo.object_id) else {
        return;
    };
    let copy = &ctx.data.salvage_ui.inventory_copy;
    let rect = Rect::new(
        516.0,
        152.0 + (index % PAGE_SIZE) as f32 * 128.0,
        720.0,
        116.0,
    );
    panel(rect, visual_theme::panel_soft());
    let name = if object.workspace_name.is_empty() {
        &object.display_name
    } else {
        &object.workspace_name
    };
    visual_theme::body(
        &(index + 1).to_string(),
        Rect::new(rect.x + 14.0, rect.y + 14.0, 32.0, 32.0),
        24.0,
        visual_theme::amber(),
    );
    visual_theme::body(
        name,
        Rect::new(rect.x + 54.0, rect.y + 10.0, 466.0, 30.0),
        24.0,
        visual_theme::text(),
    );
    let objective = ctx
        .session
        .expedition
        .as_ref()
        .filter(|expedition| expedition.contract_accepted)
        .and_then(|expedition| ctx.data.sites.get(&expedition.site_id))
        .and_then(|site| site.contract_target.as_deref())
        == Some(cargo.object_id.as_str());
    if objective {
        visual_theme::body(
            &copy.objective,
            Rect::new(rect.right() - 180.0, rect.y + 12.0, 166.0, 28.0),
            20.0,
            visual_theme::amber(),
        );
    }
    let shape = object.footprint.rotated(cargo.rotation);
    let value = ctx
        .session
        .market_quote(&cargo.object_id, ctx.data)
        .map_or(object.sale_value, |quote| quote.sale_value);
    let detail = copy
        .value
        .replace("{value}", &value.to_string())
        .replace("{width}", &shape.width.to_string())
        .replace("{height}", &shape.height.to_string());
    visual_theme::body(
        &detail,
        Rect::new(rect.x + 54.0, rect.y + 42.0, 650.0, 26.0),
        19.0,
        visual_theme::text_dim(),
    );
    draw_cargo_actions(ctx, cargo, rect, actions);
}

fn draw_cargo_actions(
    ctx: &UiContext<'_>,
    cargo: &CargoItem,
    rect: Rect,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.salvage_ui.inventory_copy;
    let rotatable = ctx
        .data
        .salvage_objects
        .get(&cargo.object_id)
        .is_some_and(|object| object.rotatable);
    for (offset, label, enabled, tone, action) in [
        (
            0.0,
            &copy.move_item,
            true,
            ButtonTone::Primary,
            UiAction::BeginDrag(cargo.object_id.clone()),
        ),
        (
            132.0,
            &copy.rotate_item,
            rotatable,
            ButtonTone::Secondary,
            UiAction::Rotate(cargo.object_id.clone()),
        ),
        (
            264.0,
            &copy.discard_item,
            true,
            ButtonTone::Warning,
            UiAction::Discard(cargo.object_id.clone()),
        ),
    ] {
        if button(
            ctx,
            Rect::new(rect.x + 54.0 + offset, rect.y + 72.0, 120.0, 36.0),
            label,
            enabled,
            tone,
        ) {
            actions.push(action);
        }
    }
}

fn draw_pagination(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>, count: usize) {
    let pages = ManifestPage::page_count(count);
    if pages <= 1 {
        return;
    }
    let copy = &ctx.data.salvage_ui.inventory_copy;
    let page = ctx.manifest_page.current(count);
    let label = copy
        .page
        .replace("{page}", &(page + 1).to_string())
        .replace("{pages}", &pages.to_string());
    visual_theme::body(
        &label,
        Rect::new(1070.0, 550.0, 166.0, 30.0),
        21.0,
        visual_theme::text_dim(),
    );
    for (next, x, label, enabled) in [
        (false, 516.0, &copy.previous, page > 0),
        (true, 688.0, &copy.next, page + 1 < pages),
    ] {
        if button(
            ctx,
            Rect::new(x, 542.0, 160.0, 44.0),
            label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::ManifestPage(next));
        }
    }
}

fn draw_navigation(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.salvage_ui;
    let pending = ctx.session.pending_count();
    if pending > 0 {
        visual_theme::body(
            &copy.inventory_copy.pending_hint,
            Rect::new(516.0, 592.0, 716.0, 24.0),
            18.0,
            visual_theme::amber(),
        );
    }
    if button(
        ctx,
        Rect::new(516.0, 620.0, 240.0, 40.0),
        &copy.back_to_wreck,
        true,
        ButtonTone::Secondary,
    ) {
        actions.push(UiAction::ReturnToWorkspace);
    }
    let (label, action, tone) = if pending > 0 {
        (
            &copy.inventory_copy.leave_pending,
            UiAction::LeaveAll,
            ButtonTone::Warning,
        )
    } else {
        (
            &copy.return_with_haul,
            UiAction::ReturnWithHaul,
            ButtonTone::Positive,
        )
    };
    if button(ctx, Rect::new(976.0, 620.0, 260.0, 40.0), label, true, tone) {
        actions.push(action);
    }
}
