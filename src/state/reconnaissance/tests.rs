use super::*;
use crate::data::GameData;

#[test]
fn route_briefs_persist_and_escalate_in_price() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    session.buy_reconnaissance("merchant_wreck", &data).unwrap();

    assert_eq!(session.reconnaissance_level("merchant_wreck"), 1);
    assert_eq!(session.economy.credits, 780);
    assert_eq!(
        session
            .reconnaissance_quote("merchant_wreck", &data)
            .unwrap()
            .cost,
        120
    );
}

#[test]
fn route_briefing_stops_at_the_configured_intelligence_ceiling() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.buy_reconnaissance("merchant_wreck", &data).unwrap();
    session.buy_reconnaissance("merchant_wreck", &data).unwrap();

    assert_eq!(session.reconnaissance_level("merchant_wreck"), 2);
    assert!(session
        .reconnaissance_quote("merchant_wreck", &data)
        .is_none());
    assert!(!session.can_buy_reconnaissance("merchant_wreck", &data));
}
