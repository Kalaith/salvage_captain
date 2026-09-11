use super::*;
use crate::data::GameData;
use crate::state::GameSession;

#[test]
fn slot_title_reports_empty_and_stored_states() {
    assert_eq!(slot_title(0, None), "SLOT 1 // EMPTY");
    assert_eq!(
        slot_title(
            2,
            Some(&LoadoutPreset {
                placements: Vec::new()
            })
        ),
        "SLOT 3 // STORED"
    );
}

#[test]
fn capacity_label_explains_the_live_refit_limits() {
    let data = GameData::load().expect("game data");
    let session = GameSession::new(&data);
    let virtual_ui = VirtualUi::from_screen_size(1280.0, 720.0, 1280.0, 720.0);
    let context = UiContext {
        data: &data,
        session: &session,
        state: GameState::Port,
        resume_state: GameState::Port,
        dragged_item: None,
        message: "",
        save_exists: false,
        settings_open: false,
        fullscreen: false,
        reduced_motion: false,
        interaction_enabled: false,
        pointer: Pointer::default(),
        pointer_started: false,
        ui: &virtual_ui,
        viewport_width: 1280.0,
        viewport_height: 720.0,
        travel_elapsed: 0.0,
        return_elapsed: 0.0,
        workspace_elapsed: 0.0,
        workspace_camera_shift: 0.0,
        workspace_arrival_flash: 0.0,
        workspace_log_open: false,
        workspace_scanned: false,
        workspace_scan_progress: 0.0,
        workspace_selected_target: None,
        workspace_extraction_target: None,
        workspace_extraction_progress: 0.0,
        workspace_extraction_phase: None,
        workspace_risk: None,
        workspace_notice: "",
        workspace_notice_warning: false,
        workspace_notice_timer: 0.0,
        port_selected_module: None,
        voyage_plan: VoyagePlan::Standard,
        port_hold_expanded: false,
        port_loadouts_open: true,
        voyage_archive_open: false,
        voyage_archive_offset: 0,
        voyage_archive_filter: voyage_archive::ArchiveFilter::All,
    };

    assert_eq!(
        capacity_label(&context),
        "LIVE CAPACITY  //  FUEL 12/24  //  HULL 8/10"
    );
}
