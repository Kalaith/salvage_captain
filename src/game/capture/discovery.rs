//! Discovery and depleted-board verification scenes.

use crate::data::discovery::{LeadKind, LootWeight};
use crate::game::Game;
use crate::state::GameState;
use crate::ui::{site_cards::BoardAction, UiAction};

pub(super) fn prepare(game: &mut Game, scene: &str) {
    if !handles(scene) {
        return;
    }
    game.state = GameState::SiteSelection;
    if matches!(
        scene,
        "sites_balance" | "sites_no_access" | "salvage_generated_large"
    ) {
        prepare_balance(game, scene);
        return;
    }
    if scene == "sites_archive_empty" {
        game.apply_action(UiAction::WreckBoard(BoardAction::ToggleArchive));
        return;
    }
    if scene == "sites_archive" {
        let roster = game
            .data
            .sites
            .get("merchant_wreck")
            .unwrap()
            .candidate_salvage
            .clone();
        game.session
            .site_progress
            .get_mut("merchant_wreck")
            .unwrap()
            .removed_targets = roster;
        game.apply_action(UiAction::WreckBoard(BoardAction::ToggleArchive));
        return;
    }
    if scene == "salvage_generated" {
        let pool = game.data.discovery.pools[0].loot.clone();
        game.data.discovery.pools[0].loot = vec![
            LootWeight {
                object_id: "industrial_battery".to_owned(),
                weight: 1,
            },
            LootWeight {
                object_id: "damaged_reactor".to_owned(),
                weight: 1,
            },
        ];
        game.apply_action(UiAction::DiscoverWreck(LeadKind::Local));
        game.data.discovery.pools[0].loot = pool;
        let id = game.wreck_selection.site_id.clone().unwrap();
        game.apply_action(UiAction::Depart(id));
        game.apply_action(UiAction::ContinueTravel);
        game.session.scan_workspace(&game.data).unwrap();
        game.workspace_camera_shift = 1.0;
        game.workspace_selected_target = game
            .session
            .expedition
            .as_ref()
            .unwrap()
            .revealed_targets
            .first()
            .cloned();
        return;
    }
    game.session.reputation = 4;
    game.apply_action(UiAction::DiscoverWreck(LeadKind::Local));
    if scene != "sites_discovery" {
        game.apply_action(UiAction::DiscoverWreck(LeadKind::Specialist));
        game.apply_action(UiAction::DiscoverWreck(LeadKind::Local));
        if scene == "sites_board_page" {
            game.apply_action(UiAction::WreckBoard(BoardAction::Next));
        }
    }
}

pub(super) fn handles(scene: &str) -> bool {
    matches!(
        scene,
        "sites_discovery"
            | "sites_balance"
            | "sites_no_access"
            | "salvage_generated_large"
            | "sites_board_full"
            | "sites_board_page"
            | "sites_archive"
            | "sites_archive_empty"
            | "salvage_generated"
    )
}

fn prepare_balance(game: &mut Game, scene: &str) {
    for site in game.data.ordered_sites() {
        game.session
            .site_progress
            .get_mut(&site.id)
            .unwrap()
            .removed_targets = site.candidate_salvage.clone();
    }
    for (index, pool) in game.data.discovery.pools.iter().enumerate() {
        let mut tuning = pool.clone();
        if index == 2 {
            tuning.minimum_targets = 8;
            tuning.maximum_targets = 8;
        }
        let wreck =
            crate::engine::discovery::generate_wreck(&tuning, index as u64 + 1, 7, &game.data)
                .unwrap();
        game.session.site_progress.insert(
            wreck.site.id.clone(),
            crate::state::SiteProgress::fresh(wreck.site.condition),
        );
        game.session.wrecks.push(wreck);
    }
    game.session.discovery_serial = 3;
    game.data = game.session.resolved_data(&game.data).unwrap();
    let id = game.session.wrecks[if scene == "salvage_generated_large" {
        2
    } else {
        0
    }]
    .site
    .id
    .clone();
    if scene == "sites_no_access" {
        for wreck in &game.session.wrecks {
            let reachable: Vec<_> = wreck
                .targets
                .iter()
                .filter(|target| {
                    wreck.site.sections[target.section_index]
                        .required_capability
                        .is_none()
                        && game
                            .session
                            .target_equipment_block_reason(&target.object, &game.data)
                            .is_none()
                })
                .map(|target| target.object.id.clone())
                .collect();
            game.session
                .site_progress
                .get_mut(&wreck.site.id)
                .unwrap()
                .removed_targets = reachable;
        }
    }
    game.wreck_selection.site_id = Some(id.clone());
    if scene == "salvage_generated_large" {
        game.session.economy.fuel = 100;
        game.apply_action(UiAction::Depart(id));
        game.apply_action(UiAction::ContinueTravel);
        game.session.scan_workspace(&game.data).unwrap();
        game.workspace_camera_shift = 1.0;
    }
}
