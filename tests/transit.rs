//! Flight layout and settled return-summary regressions.

use salvage_captain::data::{GameData, GridPosition};
use salvage_captain::state::{CargoStatus, GameSession, ReturnedItem};
use salvage_captain::ui::transit::{layout::*, return_summary, FlightLeg, HaulSummary};

#[test]
fn the_ship_stays_prominent_while_the_destination_approaches() {
    let first = transit_layout(0.0, false);
    let mut previous = first.destination;
    for step in 0..=100 {
        let current = transit_layout(step as f32 / 100.0, false);
        assert_eq!(current.ship, first.ship);
        assert!(current.ship.w >= 450.0);
        assert!(current.destination.x <= previous.x);
        assert!(current.destination.w >= previous.w);
        assert!(current.ship.right() < current.destination.x);
        assert!(current.destination.y >= 196.0);
        assert!(current.destination.bottom() < current.summary.y);
        previous = current.destination;
    }
    assert!(first.destination.x >= 1200.0);
    assert!(previous.x < 800.0);
}

#[test]
fn arrival_controls_do_not_overlap_the_scene_or_each_other() {
    let layout = transit_layout(1.0, false);
    assert!(!layout.summary.overlaps(&layout.commands));
    assert!(!layout.primary.overlaps(&layout.details));
    for control in [layout.primary, layout.details] {
        assert!(layout.commands.contains(control.point()));
        assert!(layout.commands.contains(control.point() + control.size()));
        assert!(control.h >= 40.0 && control.w >= 44.0);
        assert!(!layout.ship.overlaps(&control));
        assert!(!layout.destination.overlaps(&control));
        assert!(control.right() <= 1280.0 && control.bottom() <= 720.0);
    }
}

#[test]
fn reduced_motion_is_stationary_without_changing_journey_progress() {
    let start = transit_layout(0.0, true);
    for progress in [0.0, 0.2, 0.6, 1.0] {
        assert_eq!(
            transit_layout(progress, true).destination,
            start.destination
        );
        for layer in 0..3 {
            assert_eq!(scenery_offset(progress, layer, true), 0.0);
        }
    }
    assert!(scenery_offset(0.5, 2, false) > scenery_offset(0.5, 1, false));
    for (leg, duration) in [(FlightLeg::Outbound, 4.0), (FlightLeg::Homebound, 3.0)] {
        assert_eq!(leg.progress(-1.0), 0.0);
        assert_eq!(leg.progress(duration / 2.0), 0.5);
        assert_eq!(leg.progress(duration + 10.0), 1.0);
    }
}

#[test]
fn return_summary_preserves_the_settled_manifest_after_market_changes() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("industrial_battery", &data).unwrap();
    for item in &mut session.expedition.as_mut().unwrap().cargo {
        if item.object_id != "industrial_battery" {
            item.status = CargoStatus::LeftBehind;
        }
    }
    session.finish_packing(&data).unwrap();
    let record = session.last_voyage().unwrap();
    let expected = HaulSummary {
        count: record.recovered_count as usize,
        value: record.recovered_value,
    };
    session.market_cycle += 5;
    assert_eq!(return_summary(&session, &data), expected);
}

#[test]
fn empty_and_legacy_returns_have_honest_summaries() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    assert_eq!(
        return_summary(&session, &data),
        HaulSummary { count: 0, value: 0 }
    );
    let item = ReturnedItem {
        object_id: "industrial_battery".to_owned(),
        position: GridPosition::new(3, 3),
        rotation: 0,
        market_cycle: 0,
    };
    let locked_value = session
        .returned_market_quote(&item, &data)
        .unwrap()
        .sale_value;
    session.market_cycle = 4;
    session.returned.push(item);
    assert_eq!(
        return_summary(&session, &data),
        HaulSummary {
            count: 1,
            value: locked_value
        }
    );
}
