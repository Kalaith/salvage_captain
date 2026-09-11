//! Compact service-console labels for hull, modules, and accumulated ship wear.

use crate::ui::visual_theme;
use macroquad::prelude::*;

pub(crate) fn repair_button_label(total_cost: i64) -> String {
    if total_cost > 0 {
        format!("REPAIR ¢{total_cost}")
    } else {
        "REPAIR".to_owned()
    }
}

pub(crate) fn maintenance_status_label(
    missing_hull: i32,
    offline_modules: usize,
    ship_wear: u8,
    total_cost: i64,
    credits: i64,
) -> String {
    if total_cost == 0 {
        return "SYSTEMS NOMINAL // NO SERVICE DUE".to_owned();
    }
    if credits < total_cost {
        return format!(
            "SERVICE DUE // HULL {missing_hull} // MODULES {offline_modules} // WEAR {ship_wear}% // NEED ¢{}",
            total_cost - credits
        );
    }
    format!(
        "SERVICE DUE // HULL {missing_hull} // MODULES {offline_modules} // WEAR {ship_wear}% // TOTAL ¢{total_cost}"
    )
}

pub(crate) fn maintenance_completion_label(message: &str) -> Option<&'static str> {
    (message.starts_with("Repaired ") || message.starts_with("Serviced "))
        .then_some("SYSTEMS NOMINAL // SERVICE COMPLETE")
}

pub(crate) fn draw_wear_meter(console: Rect, wear: u8) {
    let meter = Rect::new(console.right() - 142.0, console.y + 34.0, 128.0, 6.0);
    draw_rectangle(
        meter.x,
        meter.y,
        meter.w,
        meter.h,
        visual_theme::structure_dark(),
    );
    draw_rectangle(
        meter.x,
        meter.y,
        meter.w * f32::from(wear) / 100.0,
        meter.h,
        if wear == 0 {
            visual_theme::safe()
        } else {
            visual_theme::amber()
        },
    );
    draw_text(
        &format!("WEAR {wear}%"),
        meter.x,
        console.y + 44.0,
        8.0,
        if wear == 0 {
            visual_theme::text_dim()
        } else {
            visual_theme::amber()
        },
    );
}
