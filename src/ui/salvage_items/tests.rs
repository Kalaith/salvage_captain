use super::*;

#[test]
fn packing_manifest_names_the_current_market_band() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(
        packing_market_label(session.market_quote("industrial_battery", &data)),
        "MKT STEADY +0%"
    );
}
