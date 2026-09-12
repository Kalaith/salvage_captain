//! Salvage standing and contract bonus rules.

use super::*;
use crate::data::GameData;

#[test]
fn reputation_maps_to_progressive_salvage_standings() {
    assert_eq!(
        SalvageStanding::from_reputation(0),
        SalvageStanding::Independent
    );
    assert_eq!(
        SalvageStanding::from_reputation(2),
        SalvageStanding::LocalContractor
    );
    assert_eq!(
        SalvageStanding::from_reputation(4),
        SalvageStanding::TrustedSalvor
    );
    assert_eq!(
        SalvageStanding::from_reputation(7),
        SalvageStanding::FleetPartner
    );
}

#[test]
fn standing_names_the_next_threshold_and_contract_bonus() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.reputation = 2;

    assert_eq!(session.next_standing_threshold(), Some(4));
    assert_eq!(session.contract_reward_bonus(320), 16);
}

#[test]
fn fleet_partner_has_no_next_threshold_and_pays_fifteen_percent_more() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.reputation = 7;

    assert_eq!(session.salvage_standing(), SalvageStanding::FleetPartner);
    assert_eq!(session.next_standing_threshold(), None);
    assert_eq!(session.contract_reward_bonus(500), 75);
}
