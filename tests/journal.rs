//! Journal navigation regressions: filtered pages, selection and empty records.

use salvage_captain::state::VoyageRecord;
use salvage_captain::ui::voyage_archive::{ArchiveAction, ArchiveFilter, ArchiveState, ArchiveTab};

fn records(count: usize) -> Vec<VoyageRecord> {
    (0..count)
        .map(|index| {
            serde_json::from_value(serde_json::json!({
                "site_id": if index % 2 == 0 { "merchant_wreck" } else { "military_wreck" },
                "recovered_count": 1, "recovered_value": 200, "external_load": 0,
                "risk_outcome": "OrdinaryReturn", "danger_score": 10,
                "contract_completed": false, "condition_after": 80
            }))
            .unwrap()
        })
        .collect()
}

#[test]
fn every_record_is_reachable_once_newest_first() {
    for count in [1, 5, 6, 10, 13] {
        let log = records(count);
        let mut state = ArchiveState::default();
        let mut visited = Vec::new();
        loop {
            visited.extend(state.visible_indices(&log));
            let previous = state.offset;
            state.apply(ArchiveAction::Older, &log);
            if state.offset == previous {
                break;
            }
        }
        assert_eq!(visited, (0..count).rev().collect::<Vec<_>>());
        for _ in 0..count {
            state.apply(ArchiveAction::Newer, &log);
        }
        assert_eq!(state.offset, 0);
    }
}

#[test]
fn changing_filter_resets_selection_and_pagination_uses_filtered_count() {
    let log = records(22);
    let mut state = ArchiveState::default();
    state.apply(ArchiveAction::Older, &log);
    state.apply(ArchiveAction::Select(14), &log);
    state.apply(ArchiveAction::Filter(ArchiveFilter::Merchant), &log);
    assert_eq!(state.offset, 0);
    assert_eq!(state.selected, None);
    assert_eq!(state.selected_index(&log), Some(20));
    state.apply(ArchiveAction::Older, &log);
    assert_eq!(state.visible_indices(&log), vec![10, 8, 6, 4, 2]);
    for _ in 0..5 {
        state.apply(ArchiveAction::Older, &log);
    }
    assert_eq!(state.offset, 10);
    assert_eq!(state.visible_indices(&log), vec![0]);
}

#[test]
fn selected_record_stays_on_the_visible_page() {
    let log = records(12);
    let mut state = ArchiveState::default();
    state.apply(ArchiveAction::Select(8), &log);
    assert_eq!(state.selected_index(&log), Some(8));
    state.apply(ArchiveAction::Select(0), &log);
    assert_eq!(state.selected_index(&log), Some(8));
    state.apply(ArchiveAction::Older, &log);
    assert_eq!(state.selected_index(&log), Some(6));
    state.apply(ArchiveAction::Newer, &log);
    assert_eq!(state.selected_index(&log), Some(11));
}

#[test]
fn empty_and_shrinking_archives_recover_from_stale_offsets() {
    for log in [vec![], records(3)] {
        let mut state = ArchiveState {
            offset: usize::MAX,
            selected: Some(200),
            ..Default::default()
        };
        state.apply(ArchiveAction::Older, &log);
        assert_eq!(state.offset, 0);
        assert_eq!(state.selected_index(&log), log.len().checked_sub(1));
        state.apply(ArchiveAction::Filter(ArchiveFilter::Research), &log);
        assert!(state.visible_indices(&log).is_empty());
        assert_eq!(state.selected_index(&log), None);
    }
}

#[test]
fn changing_tabs_resets_subpages_without_losing_voyage_selection() {
    let log = records(8);
    let mut state = ArchiveState::default();
    state.apply(ArchiveAction::Select(5), &log);
    for tab in ArchiveTab::ALL {
        state.apply(ArchiveAction::Tab(tab), &log);
        assert_eq!(state.page, 0);
        state.apply(ArchiveAction::Page(usize::MAX), &log);
        assert_eq!(state.page, tab.page_count() - 1);
    }
    assert_eq!(state.selected_index(&log), Some(5));
}
