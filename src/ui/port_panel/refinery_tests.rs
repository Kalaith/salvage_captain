use super::*;

#[test]
fn refinery_buttons_name_stock_batch_and_payout() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let quote = session.refinery_quote(crate::engine::refinery::RefineryResource::Alloy, &data);

    assert_eq!(refinery_button_label(quote), "ALLOY 0/5  ->  ¢100");
}
