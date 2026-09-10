//! Versioned save-slot operations kept separate from game-state mutation.

use crate::data::GameData;
use crate::state::{migrate_save_value, GameSession, SaveData};
use macroquad_toolkit::persistence::{
    get_save_slots, load_from_slot_with_migration, save_to_slot_with_version_and_backup,
    slot_exists,
};

pub fn save_session(session: &GameSession, data: &GameData) -> Result<(), String> {
    let save = session.to_save(&data.config.version);
    save_to_slot_with_version_and_backup(
        &data.config.game_name,
        &data.config.save_slot,
        &save,
        &data.config.version,
    )
}

pub fn load_session(data: &GameData) -> Result<GameSession, String> {
    let save: SaveData = load_from_slot_with_migration(
        &data.config.game_name,
        &data.config.save_slot,
        &data.config.version,
        |version, value| migrate_save_value(version, value, data),
    )?;
    GameSession::from_save(save, data)
}

pub fn has_save(data: &GameData) -> bool {
    slot_exists(&data.config.game_name, &data.config.save_slot)
}

pub fn slots(data: &GameData) -> Vec<String> {
    get_save_slots(&data.config.game_name)
}
