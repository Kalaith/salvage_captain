//! Compact service-console labels for hull, modules, and accumulated ship wear.

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
