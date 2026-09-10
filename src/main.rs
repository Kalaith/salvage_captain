//! Salvage Captain entry point and capture-aware window setup.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod data;
mod engine;
mod game;
mod save;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf(
        "SALVAGE_CAPTAIN",
        "Salvage Captain",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    if let Some(configs) = capture::CaptureConfig::all_from_env("SALVAGE_CAPTAIN") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |dt| {
                game.update(dt);
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        if game.exit_requested() {
            break;
        }
        next_frame().await;
    }
}
