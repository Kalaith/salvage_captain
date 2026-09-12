//! Data-driven refinery batch and payout rules.

use crate::data::GameData;
use crate::engine::refinery::*;
use crate::state::EconomyState;

#[test]
fn refinery_quotes_use_the_configured_material_batches() {
    let data = GameData::load().unwrap();
    let economy = EconomyState {
        credits: 850,
        fuel: 12,
        alloy: 11,
        electronics: 7,
    };

    let alloy = quote_for(RefineryResource::Alloy, economy, &data.config.refinery);
    let electronics = quote_for(
        RefineryResource::Electronics,
        economy,
        &data.config.refinery,
    );

    assert_eq!(alloy.batch_size, 5);
    assert_eq!(alloy.payout, 100);
    assert_eq!(alloy.batches_available(), 2);
    assert_eq!(electronics.batch_size, 3);
    assert_eq!(electronics.payout, 120);
    assert_eq!(electronics.batches_available(), 2);
}

#[test]
fn refinery_disables_a_batch_until_enough_material_is_recovered() {
    let data = GameData::load().unwrap();
    let economy = EconomyState {
        credits: 0,
        fuel: 0,
        alloy: 4,
        electronics: 0,
    };
    let quote = quote_for(RefineryResource::Alloy, economy, &data.config.refinery);

    assert!(!quote.can_refine());
    assert_eq!(quote.batches_available(), 0);
}
