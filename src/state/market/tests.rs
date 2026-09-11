use super::*;
use crate::data::GridPosition;
use crate::state::CargoStatus;

#[test]
fn returned_salvage_keeps_the_quote_from_its_run() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let object_id = "industrial_battery";

    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place(object_id, &data).unwrap();
    for item in &mut session.expedition.as_mut().unwrap().cargo {
        if item.object_id != object_id {
            item.status = CargoStatus::LeftBehind;
        }
    }
    session.finish_packing(&data).unwrap();

    let returned = session.returned.first().unwrap().clone();
    let locked_quote = session.returned_market_quote(&returned, &data).unwrap();
    assert_eq!(returned.market_cycle, 0);
    assert_eq!(session.market_cycle, 1);
    assert_eq!(locked_quote.sale_value, 160);

    let next_cycle_quote = session.market_quote(object_id, &data).unwrap();
    assert_ne!(next_cycle_quote.sale_value, locked_quote.sale_value);
}

#[test]
fn market_cycle_survives_save_round_trip() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.market_cycle = 9;
    session
        .ship_layout
        .place(
            "cargo:industrial_battery".to_owned(),
            data.salvage_objects
                .get("industrial_battery")
                .unwrap()
                .footprint,
            GridPosition::new(3, 3),
            0,
            false,
        )
        .unwrap();
    session.returned.push(ReturnedItem {
        object_id: "industrial_battery".to_owned(),
        position: GridPosition::new(3, 3),
        rotation: 0,
        market_cycle: 4,
    });

    let restored = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap();

    assert_eq!(restored.market_cycle, 9);
    assert_eq!(restored.returned[0].market_cycle, 4);
}
