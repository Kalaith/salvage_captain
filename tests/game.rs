//! Integration suite for player-facing state prompts.

pub mod state {
    pub use salvage_captain::state::*;
}
pub mod game {
    pub use salvage_captain::game::*;
}

pub use game::prompts::*;
pub use state::*;

#[path = "game/prompts.rs"]
mod prompt_tests;
