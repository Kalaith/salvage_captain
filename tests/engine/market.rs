//! Market-band rotation and quote rules.

use crate::data::GameData;
use crate::engine::market::*;

#[test]
fn market_quotes_are_deterministic_and_use_the_object_group() {
    let data = GameData::load().unwrap();
    let object = data.salvage_objects.get("industrial_battery").unwrap();
    let first = quote_for(object, 3, &data.config.market);
    let second = quote_for(object, 3, &data.config.market);

    assert_eq!(first, second);
    assert_eq!(first.sale_value, 160);
    assert_eq!(first.band, MarketBand::Steady);
}

#[test]
fn market_quotes_rotate_bands_without_changing_the_base_value() {
    let data = GameData::load().unwrap();
    let object = data.salvage_objects.get("industrial_battery").unwrap();

    let quotes = [
        quote_for(object, 0, &data.config.market),
        quote_for(object, 1, &data.config.market),
        quote_for(object, 2, &data.config.market),
    ];

    assert!(quotes.iter().any(|quote| quote.band == MarketBand::Hot));
    assert!(quotes.iter().any(|quote| quote.band == MarketBand::Steady));
    assert!(quotes.iter().any(|quote| quote.band == MarketBand::Soft));
    assert_eq!(
        quotes[0].sale_value + quotes[1].sale_value + quotes[2].sale_value,
        496
    );
}
