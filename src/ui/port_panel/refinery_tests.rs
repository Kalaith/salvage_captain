use super::*;

#[test]
fn refinery_buttons_name_stock_batch_and_payout() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let quote = session.refinery_quote(crate::engine::refinery::RefineryResource::Alloy, &data);

    assert_eq!(refinery_button_label(quote), "ALLOY 0/5  ->  ¢100");
}

#[test]
fn refinery_panel_totals_all_complete_batches() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.alloy = 8;
    session.economy.electronics = 5;
    let alloy = session.refinery_quote(crate::engine::refinery::RefineryResource::Alloy, &data);
    let electronics = session.refinery_quote(
        crate::engine::refinery::RefineryResource::Electronics,
        &data,
    );

    assert_eq!(refinery_total_payout(alloy, electronics), 220);
}
