use super::*;

#[test]
fn packed_contract_target_pays_once() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let site = data.sites.get("merchant_wreck").unwrap();
    let target_id = site.contract_target.as_ref().unwrap().clone();
    let before = session.economy.credits;
    let cargo = vec![CargoItem {
        object_id: target_id,
        status: CargoStatus::Packed,
        position: Some(crate::data::GridPosition::new(1, 1)),
        rotation: 0,
    }];

    let message = session
        .complete_site_contract("merchant_wreck", &cargo, &data)
        .unwrap();
    let second = session.complete_site_contract("merchant_wreck", &cargo, &data);

    assert!(message.contains("Bonus +180 credits"));
    assert_eq!(session.economy.credits, before + 180);
    assert!(second.is_none());
}
