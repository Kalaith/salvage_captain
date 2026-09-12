//! Flight composition fixtures at both ends of the route and with reduced motion.

use super::Game;
use crate::state::GameState;

pub(super) fn prepare(game: &mut Game, scene: &str) -> Option<GameState> {
    let (site, elapsed) = match scene {
        "travel_departure" => ("merchant_wreck", 0.0),
        "travel_approach" => ("merchant_wreck", 3.3),
        "travel_arrived" => ("merchant_wreck", 4.0),
        "travel_details" => ("merchant_wreck", 2.0),
        "travel_military" => ("military_wreck", 3.3),
        "travel_research" => ("research_vessel", 3.3),
        "travel_reduced_motion" | "travel_paused" => ("merchant_wreck", 1.6),
        "return_departure"
        | "return_approach"
        | "return_arrived"
        | "return_empty"
        | "return_damage"
        | "return_reduced_motion" => {
            let _ = super::return_travel::prepare(game, false);
            game.return_elapsed = match scene {
                "return_departure" => 0.0,
                "return_arrived" => 3.0,
                _ => 2.5,
            };
            if scene == "return_empty" {
                game.session.returned.clear();
                if let Some(record) = game.session.voyage_log.last_mut() {
                    record.recovered_count = 0;
                    record.recovered_value = 0;
                    record.risk_outcome = crate::engine::RiskOutcome::OrdinaryReturn;
                }
            }
            if scene == "return_damage" {
                game.session.hull = 3;
                if let Some(record) = game.session.voyage_log.last_mut() {
                    record.risk_outcome = crate::engine::RiskOutcome::DamagedModule;
                }
            }
            return Some(GameState::ReturnTravel);
        }
        _ => return None,
    };
    game.session
        .begin_expedition(site, &game.data)
        .expect("capture departure must be valid");
    game.travel_elapsed = elapsed;
    game.transit_details_open = scene == "travel_details";
    Some(if scene == "travel_paused" {
        GameState::Pause
    } else {
        GameState::Travel
    })
}
