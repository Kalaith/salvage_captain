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
        game.data.discovery.pools[0].loot = vec![LootWeight {
            object_id: "industrial_battery".to_owned(),
            weight: 1,
        }];
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
            | "sites_board_full"
            | "sites_board_page"
            | "sites_archive"
            | "sites_archive_empty"
            | "salvage_generated"
    )
}
