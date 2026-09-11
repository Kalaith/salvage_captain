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

#[test]
fn packing_manifest_keeps_material_yields_beside_the_market_ask() {
    let data = GameData::load().unwrap();
    let object = data.salvage_objects.get("industrial_battery").unwrap();
    let session = GameSession::new(&data);
    let label = packing_value_label(object, session.market_quote(&object.id, &data));

    assert!(label.contains("BASE ¢160 -> ASK ¢160"));
    assert!(label.contains("A2 E3"));
}
