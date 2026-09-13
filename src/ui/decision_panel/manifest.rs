//! Paged cargo rows with readable ownership, prices, and material yields.

use super::navigation::{ManifestPage, PAGE_SIZE};
use super::*;

pub(super) fn draw_result_manifest(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.debrief_ui;
    text(
        &copy.cargo,
        Rect::new(32.0, 310.0, 900.0, 32.0),
        26.0,
        visual_theme::text(),
    );
    text(
        &copy.cargo_hint,
        Rect::new(32.0, 344.0, 864.0, 28.0),
        19.0,
        visual_theme::text_dim(),
    );
    if ctx.session.returned.is_empty() {
        text(
            &copy.empty,
            Rect::new(52.0, 412.0, 900.0, 36.0),
            28.0,
            visual_theme::text(),
        );
        text(
            &copy.empty_hint,
            Rect::new(52.0, 456.0, 1000.0, 32.0),
            20.0,
            visual_theme::text_dim(),
        );
        if button(
            ctx,
            Rect::new(52.0, 514.0, 220.0, 48.0),
            &copy.port,
            true,
            ButtonTone::Positive,
        ) {
            actions.push(UiAction::GoToPort);
        }
        return;
    }
    let page = ctx.manifest_page.current(ctx.session.returned.len());
    let objective = ctx
        .session
        .last_voyage()
        .map(|record| &record.site_id)
        .or(ctx.session.selected_site.as_ref())
        .and_then(|id| ctx.session.contract_objective_status(id, ctx.data))
        .filter(|objective| objective.state == ContractObjectiveState::Complete);
    for (index, returned) in ctx
        .session
        .returned
        .iter()
        .skip(page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .enumerate()
    {
        let Some(object) = ctx.data.salvage_objects.get(&returned.object_id) else {
            continue;
        };
        let credited = objective
            .as_ref()
            .is_some_and(|objective| objective.target_id == object.id);
        draw_result_card(
            ctx,
            returned,
            object,
            Rect::new(32.0, 384.0 + index as f32 * 92.0, 1216.0, 84.0),
            credited,
            actions,
        );
    }
    draw_pagination(ctx, actions);
}

fn draw_result_card(
    ctx: &UiContext<'_>,
    returned: &crate::state::ReturnedItem,
    object: &crate::data::SalvageObjectData,
    rect: Rect,
    credited: bool,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.debrief_ui;
    panel(rect, visual_theme::panel());
    if credited {
        draw_rectangle(rect.x, rect.y, 4.0, rect.h, visual_theme::safe());
    }
    draw_silhouette(Rect::new(rect.x + 16.0, rect.y + 22.0, 52.0, 40.0));
    text(
        object_name(object),
        Rect::new(rect.x + 88.0, rect.y + 12.0, 480.0, 28.0),
        24.0,
        visual_theme::text(),
    );
    text(
        if credited {
            &copy.credited_item
        } else {
            &copy.own_item
        },
        Rect::new(rect.x + 88.0, rect.y + 46.0, 480.0, 26.0),
        19.0,
        if credited {
            visual_theme::safe()
        } else {
            visual_theme::text_dim()
        },
    );
    draw_choices(ctx, returned, object, rect, actions);
}

fn draw_choices(
    ctx: &UiContext<'_>,
    returned: &crate::state::ReturnedItem,
    object: &crate::data::SalvageObjectData,
    rect: Rect,
    actions: &mut Vec<UiAction>,
) {
    let copy = &ctx.data.debrief_ui;
    let sale = ctx
        .session
        .returned_market_quote(returned, ctx.data)
        .map_or(object.sale_value, |quote| quote.sale_value);
    let module = object
        .install_module_id
        .as_ref()
        .and_then(|id| ctx.data.modules.get(id));
    let installed = module.is_some_and(|module| {
        ctx.session
            .ship_layout
            .placements
            .iter()
            .any(|item| item.id == module.id)
    });
    let affordable =
        module.is_some_and(|module| ctx.session.economy.credits >= module.install_cost);
    let install_label = module.map_or_else(
        || copy.install.replace(" ¢{cost}", ""),
        |module| {
            copy.install
                .replace("{cost}", &module.install_cost.to_string())
        },
    );
    let install_hint = match module {
        None => &copy.no_install,
        Some(_) if installed => &copy.installed,
        Some(_) if !affordable => &copy.need_credits,
        Some(module) => &module.display_name,
    };
    let choices = [
        (
            copy.sell.replace("{value}", &sale.to_string()),
            copy.sell_hint.clone(),
            Disposition::Sell,
            true,
            ButtonTone::Positive,
        ),
        (
            install_label,
            install_hint.clone(),
            Disposition::Install,
            module.is_some() && !installed && affordable,
            ButtonTone::Primary,
        ),
        (
            copy.break_down.clone(),
            copy.materials
                .replace("{alloy}", &object.alloy_yield.to_string())
                .replace("{electronics}", &object.electronics_yield.to_string()),
            Disposition::BreakDown,
            true,
            ButtonTone::Secondary,
        ),
    ];
    for (index, (label, hint, disposition, enabled, tone)) in choices.into_iter().enumerate() {
        let x = rect.right() - 580.0 + index as f32 * 192.0;
        if button(
            ctx,
            Rect::new(x, rect.y + 8.0, 180.0, 44.0),
            &label,
            enabled,
            tone,
        ) {
            actions.push(UiAction::Disposition(object.id.clone(), disposition));
        }
        text(
            &hint,
            Rect::new(x, rect.y + 57.0, 188.0, 23.0),
            18.0,
            visual_theme::text_dim(),
        );
    }
}

fn draw_pagination(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.debrief_ui;
    let count = ctx.session.returned.len();
    let pages = ManifestPage::page_count(count);
    if pages <= 1 {
        return;
    }
    let page = ctx.manifest_page.current(count);
    let label = copy
        .page
        .replace("{count}", &count.to_string())
        .replace("{page}", &(page + 1).to_string())
        .replace("{pages}", &pages.to_string());
    text(
        &label,
        Rect::new(912.0, 348.0, 336.0, 30.0),
        20.0,
        visual_theme::text_dim(),
    );
    for (next, x, label, enabled) in [
        (false, 912.0, &copy.previous, page > 0),
        (true, 1084.0, &copy.next, page + 1 < pages),
    ] {
        if button(
            ctx,
            Rect::new(x, 300.0, 164.0, 44.0),
            label,
            enabled,
            ButtonTone::Secondary,
        ) {
            actions.push(UiAction::ManifestPage(next));
        }
    }
}

fn draw_silhouette(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        visual_theme::structure_dark(),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, visual_theme::amber());
    draw_rectangle(
        rect.x + 8.0,
        rect.y + 9.0,
        rect.w - 16.0,
        rect.h - 18.0,
        visual_theme::amber(),
    );
    for x in [rect.x + 17.0, rect.right() - 17.0] {
        draw_line(
            x,
            rect.y + 5.0,
            x,
            rect.bottom() - 5.0,
            2.0,
            visual_theme::structure_light(),
        );
    }
}
