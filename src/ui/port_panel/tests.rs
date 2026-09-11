use super::*;

#[test]
fn market_ticker_uses_the_current_cycle_and_explains_the_price_lock() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let best = data
        .salvage_objects
        .iter()
        .filter_map(|(_, object)| {
            session
                .market_quote(&object.id, &data)
                .map(|quote| (object, quote))
        })
        .max_by_key(|(_, quote)| (quote.signed_multiplier(), quote.sale_value))
        .unwrap();
    let ticker = format!(
        "{}  //  BUYERS FAVOR {} {} {:+}%  //  PRICES LOCK AT RETURN",
        session.market_cycle_label(),
        best.0.market_group.to_uppercase(),
        best.1.band.label(),
        best.1.signed_multiplier()
    );

    assert!(ticker.contains("CYCLE 00"));
    assert!(ticker.contains("BUYERS FAVOR"));
    assert!(ticker.contains("PRICES LOCK AT RETURN"));
}

#[test]
fn maintenance_readout_breaks_repair_cost_into_hull_and_system_work() {
    assert_eq!(repair_button_label(0), "FULL REPAIR");
    assert_eq!(repair_button_label(55), "FULL REPAIR ¢55");
    assert_eq!(
        maintenance_status_label(2, 1, 0, 125, 850),
        "SERVICE DUE // HULL 2 // MODULES 1 // WEAR 0% // TOTAL ¢125"
    );
    assert_eq!(
        maintenance_status_label(0, 1, 22, 55, 20),
        "SERVICE DUE // HULL 0 // MODULES 1 // WEAR 22% // NEED ¢35"
    );
    assert_eq!(
        maintenance_completion_label("Repaired hull and systems for 160 credits."),
        Some("SYSTEMS NOMINAL // SERVICE COMPLETE")
    );
    assert_eq!(
        maintenance_completion_label("Serviced ship systems for 48 credits"),
        Some("SYSTEMS NOMINAL // SERVICE COMPLETE")
    );
    assert_eq!(
        maintenance_completion_label("At the port. Shipyard ready."),
        None
    );
}

#[test]
fn selected_module_status_marks_an_installed_damaged_system_offline() {
    assert_eq!(
        selected_module_status(true, true, true, true, 0),
        "INSTALLED // OFFLINE // SERVICE DUE"
    );
    assert_eq!(
        selected_module_status(true, true, true, false, 0),
        "INSTALLED"
    );
    assert_eq!(
        selected_module_status(false, false, false, false, 1200),
        "LOCKED // EARN ¢1200"
    );
}
