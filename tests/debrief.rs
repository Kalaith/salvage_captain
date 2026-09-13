//! Returned objective ownership and manifest navigation regressions.

use salvage_captain::data::{GameData, GridPosition};
use salvage_captain::engine::Disposition;
use salvage_captain::state::{contracts::ContractObjectiveState, GameSession};
use salvage_captain::ui::decision_panel::navigation::{ManifestPage, PAGE_SIZE};

#[test]
fn completed_objective_is_available_for_every_cargo_choice_without_losing_reward() {
    let data = GameData::load().unwrap();
    for disposition in [
        Disposition::Sell,
        Disposition::Install,
        Disposition::BreakDown,
    ] {
        let mut session = GameSession::new(&data);
        session.begin_expedition("merchant_wreck", &data).unwrap();
        session.scan_workspace(&data).unwrap();
        session
            .begin_workspace_transfer("industrial_battery", GridPosition::new(3, 2), 0, &data)
            .unwrap();
        session
            .recover_workspace_target("industrial_battery", &data)
            .unwrap();
        session.leave_all_pending().unwrap();
        session.finish_packing(&data).unwrap();
        assert_eq!(session.career.contract_income, 180);
        assert!(session
            .returned
            .iter()
            .any(|item| item.object_id == "industrial_battery"));
        let before = session.economy;
        let object = data.salvage_objects.get("industrial_battery").unwrap();
        let sale = session
            .returned_market_quote(&session.returned[0], &data)
            .unwrap()
            .sale_value;
        session
            .dispose("industrial_battery", disposition, &data)
            .unwrap();
        assert!(session.returned.is_empty());
        assert_eq!(session.career.contract_income, 180);
        assert_eq!(
            session
                .contract_objective_status("merchant_wreck", &data)
                .unwrap()
                .state,
            ContractObjectiveState::Complete
        );
        match disposition {
            Disposition::Sell => assert_eq!(session.economy.credits, before.credits + sale),
            Disposition::Install => {
                assert_eq!(session.economy.credits, before.credits - 80);
                assert!(session
                    .ship_layout
                    .placements
                    .iter()
                    .any(|item| item.id == "battery_module"));
            }
            Disposition::BreakDown => {
                assert_eq!(session.economy.credits, before.credits);
                assert_eq!(session.economy.alloy, before.alloy + object.alloy_yield);
                assert_eq!(
                    session.economy.electronics,
                    before.electronics + object.electronics_yield
                );
            }
        }
        assert!(session
            .complete_site_contract("merchant_wreck", &[], &data)
            .is_none());
    }
}

#[test]
fn pages_expose_every_item_and_stop_at_both_ends() {
    for count in [0, 1, 3, 4, 6, 7, 24] {
        let mut page = ManifestPage::default();
        page.turn(false, count);
        assert_eq!(page.index, 0);
        let mut visited = Vec::new();
        for _ in 0..ManifestPage::page_count(count) {
            let start = page.current(count) * PAGE_SIZE;
            visited.extend(start..(start + PAGE_SIZE).min(count));
            page.turn(true, count);
        }
        assert_eq!(visited, (0..count).collect::<Vec<_>>());
        assert_eq!(page.index, ManifestPage::page_count(count) - 1);
    }
}

#[test]
fn resolving_the_last_item_on_a_page_returns_to_remaining_cargo() {
    let mut page = ManifestPage { index: 2 };
    assert_eq!(page.current(7), 2);
    assert_eq!(page.current(6), 1);
    page.turn(false, 6);
    assert_eq!(page.index, 0);
    assert_eq!(ManifestPage { index: 4 }.current(0), 0);
}
