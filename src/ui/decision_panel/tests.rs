use super::*;

#[test]
fn debrief_run_label_keeps_survey_memory_visible() {
    let label = debrief_run_label(
        2,
        "MERCHANT WRECK",
        1,
        0,
        280,
        6,
        3,
        WorkspaceScanProfile::Array,
        5,
        9,
        "STAND TRUSTED SALVOR // REP 4/7",
    );
    assert!(label.contains("MERCHANT WRECK  //  SCAN ARRAY"));
    assert!(label.contains("SCAN ARRAY  //  BP 05/09"));
    assert!(label.contains("BP 05/09  //  STAND TRUSTED SALVOR // REP 4/7"));
    assert!(label.ends_with("FIELD LOG 06  //  SURV 03"));
}

#[test]
fn debrief_names_the_current_contract_standing() {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        debrief_standing_label(&session),
        "STAND INDEPENDENT // REP 0/2"
    );
}

#[test]
fn returned_item_quote_is_available_for_the_disposition_readout() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("industrial_battery", &data).unwrap();
    for item in &mut session.expedition.as_mut().unwrap().cargo {
        if item.object_id != "industrial_battery" {
            item.status = crate::state::CargoStatus::LeftBehind;
        }
    }
    session.finish_packing(&data).unwrap();
    let returned = session.returned.first().unwrap();

    let quote = session.returned_market_quote(returned, &data).unwrap();
    assert_eq!(quote.band.label(), "STEADY");
    assert_eq!(quote.sale_value, 160);
    assert_eq!(result_sell_label(Some(quote)), "SELL ¢160");
}

#[test]
fn debrief_forecasts_refinery_batches_from_the_returned_haul() {
    let data = crate::data::GameData::load().unwrap();
    let session = GameSession::new(&data);
    let returned = [crate::state::ReturnedItem {
        object_id: "industrial_battery".to_owned(),
        position: crate::data::GridPosition::new(3, 3),
        rotation: 0,
        market_cycle: 0,
    }];

    let label = refinery_forecast_label(session.economy, &returned, &data);

    assert!(label.contains("REFINERY FORECAST"));
    assert!(label.contains("BATCHES"));
    assert!(label.contains("CASH ¢120"));
}
