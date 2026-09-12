//! Port rail boundaries and authoritative refuel price regression coverage.

use macroquad::prelude::Rect;
use salvage_captain::data::GameData;
use salvage_captain::state::GameSession;
use salvage_captain::ui::port_panel::{
    content_rect, departure_rect, stock_card_rect, stock_page, tab_rect, PortTab,
};

#[test]
fn tab_content_and_departure_touch_regions_are_disjoint() {
    let viewport = Rect::new(0.0, 0.0, 1280.0, 720.0);
    let mut controls: Vec<_> = PortTab::ALL.into_iter().map(tab_rect).collect();
    controls.extend([content_rect(), departure_rect()]);
    for (index, rect) in controls.iter().enumerate() {
        assert!(viewport.contains(rect.point()) && viewport.contains(rect.point() + rect.size()));
        assert!(rect.h >= 44.0);
        for other in &controls[index + 1..] {
            assert!(!rect.overlaps(other));
        }
    }
    assert_eq!(PortTab::default(), PortTab::Service);
}

#[test]
fn equipment_pages_reach_every_module_without_crossing_the_footer() {
    let data = GameData::load().unwrap();
    let count = data.modules.iter().count();
    let mut visited = Vec::new();
    let mut page = 0;
    loop {
        visited.extend((page * 4..(page * 4 + 4).min(count)).collect::<Vec<_>>());
        let next = stock_page(page, true, count);
        if next == page {
            break;
        }
        page = next;
    }
    assert_eq!(visited, (0..count).collect::<Vec<_>>());
    assert_eq!(stock_page(0, false, count), 0);
    assert_eq!(stock_page(usize::MAX, true, count), page);
    assert_eq!(stock_page(5, true, 0), 0);
    for index in 0..4 {
        let rect = stock_card_rect(index);
        assert!(content_rect().contains(rect.point() + rect.size()));
        assert!(!rect.overlaps(&departure_rect()));
        for other in index + 1..4 {
            assert!(!rect.overlaps(&stock_card_rect(other)));
        }
    }
}

#[test]
fn displayed_refuel_quotes_match_full_and_partial_transactions() {
    let data = GameData::load().unwrap();
    let price = i64::from(data.config.refuel_price_per_unit);
    for credits in [price, price * 3 + 1, 10_000] {
        let mut session = GameSession::new(&data);
        session.economy.fuel = 0;
        session.economy.credits = credits;
        let quote = session.refuel_quote(&data);
        assert_eq!(
            quote.amount,
            (credits / price).min(i64::from(session.max_fuel(&data))) as i32
        );
        session.refuel(&data).unwrap();
        assert_eq!(session.economy.fuel, quote.amount);
        assert_eq!(session.economy.credits, credits - quote.cost);
    }
}

#[test]
fn full_tanks_and_unaffordable_fuel_have_no_transaction() {
    let data = GameData::load().unwrap();
    for full in [false, true] {
        let mut session = GameSession::new(&data);
        session.economy.fuel = if full { session.max_fuel(&data) } else { 0 };
        session.economy.credits = if full { 1000 } else { 0 };
        let before = (session.economy.fuel, session.economy.credits);
        assert_eq!(session.refuel_quote(&data).amount, 0);
        assert_eq!(session.refuel_quote(&data).cost, 0);
        assert!(session.refuel(&data).is_err());
        assert_eq!((session.economy.fuel, session.economy.credits), before);
    }
}

#[test]
fn large_credit_balances_cannot_overflow_the_refuel_amount() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.economy.fuel = 0;
    session.economy.credits = i64::MAX;
    let quote = session.refuel_quote(&data);
    assert_eq!(quote.amount, session.max_fuel(&data));
    session.refuel(&data).unwrap();
    assert_eq!(session.economy.fuel, session.max_fuel(&data));
    assert_eq!(session.economy.credits, i64::MAX - quote.cost);
}
