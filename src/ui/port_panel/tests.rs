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
