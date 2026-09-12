//! Wreck selection verification states layered over the existing progression fixtures.

use crate::game::Game;

pub(super) fn prepare(game: &mut Game, scene: &str) {
    if scene.starts_with("sites_") {
        match scene {
            "sites_military" | "sites_insured" => {
                game.wreck_selection.site_id = Some("military_wreck".to_owned())
            }
            "sites_research" | "sites_low_fuel" => {
                game.wreck_selection.site_id = Some("research_vessel".to_owned())
            }
            _ => {}
        }
        game.wreck_selection.details_open = matches!(
            scene,
            "sites_details" | "sites_progress" | "sites_route_familiarity" | "sites_crew_veteran"
        );
        game.wreck_selection.private_haul = scene == "sites_private_haul";
        game.wreck_selection.insured = matches!(scene, "sites_insured" | "sites_low_credits");
        if scene == "sites_low_fuel" {
            game.session.economy.fuel = 0;
        }
        if scene == "sites_low_credits" {
            game.session.economy.credits = 0;
        }
        if scene == "sites_tired" {
            game.session.crew_fatigue = 30;
        }
        if scene == "sites_completed" {
            game.session
                .site_progress
                .get_mut("merchant_wreck")
                .unwrap()
                .contract_completed = true;
        }
        if scene == "sites_failed" {
            game.session
                .site_progress
                .get_mut("merchant_wreck")
                .unwrap()
                .contract_failed = true;
        }
    }
}
