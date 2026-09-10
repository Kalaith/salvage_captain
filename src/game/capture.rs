//! Deterministic scene setup used by the shared verification harness.

use super::{prompts, Game};
use crate::data::GridPosition;
use crate::state::workspace::ExtractionRuntime;
use crate::state::{CargoStatus, GameSession, GameState};

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.session = GameSession::new(&self.data);
        self.travel_elapsed = 0.0;
        self.workspace_elapsed = 0.0;
        self.workspace_scan_elapsed = 0.0;
        self.workspace_selected_target = None;
        self.workspace_extraction = None;
        self.workspace_risk = None;
        self.workspace_notice.clear();
        self.workspace_notice_warning = false;
        self.workspace_notice_timer = 0.0;
        self.port_selected_module = Some("engine_core".to_owned());
        self.port_hold_expanded = false;
        self.settings_open = scene == "settings";
        self.exit_requested = false;
        self.state = match scene {
            "main_menu" => GameState::MainMenu,
            "settings" => GameState::Pause,
            "gameplay" | "port" => GameState::Port,
            "port_preview" => {
                self.port_selected_module = Some("scanner_module".to_owned());
                GameState::Port
            }
            "sites" => GameState::SiteSelection,
            "travel" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                self.travel_elapsed = 2.0;
                GameState::Travel
            }
            "salvage_scan" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                GameState::SalvageWorkspace
            }
            "salvage_extract" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 2.0;
                self.workspace_selected_target = Some("industrial_battery".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("industrial_battery", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("industrial_battery", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "industrial_battery".to_owned(),
                    elapsed: 0.0,
                    duration: 3.8,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "salvage_capture" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                let _ = self.session.scan_workspace(&self.data);
                self.workspace_elapsed = 5.5;
                self.workspace_selected_target = Some("industrial_battery".to_owned());
                self.workspace_risk = self
                    .session
                    .workspace_risk_preview("industrial_battery", &self.data)
                    .ok();
                let _ = self
                    .session
                    .reserve_workspace_energy("industrial_battery", &self.data);
                self.workspace_extraction = Some(ExtractionRuntime {
                    target_id: "industrial_battery".to_owned(),
                    elapsed: 92.8,
                    duration: 100.0,
                    resolved: false,
                });
                GameState::SalvageWorkspace
            }
            "packing" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                GameState::SalvagePacking
            }
            "results" => {
                let _ = self.session.begin_expedition("merchant_wreck", &self.data);
                if let Some(expedition) = self.session.expedition.as_mut() {
                    for item in &mut expedition.cargo {
                        if item.status == CargoStatus::Pending {
                            item.status = CargoStatus::Packed;
                            item.position = Some(GridPosition::new(2, 2));
                        }
                    }
                }
                let _ = self.session.finish_packing(&self.data);
                GameState::Results
            }
            "paused" => GameState::Pause,
            _ => panic!("Unknown Salvage Captain capture scene: {scene}"),
        };
        self.resume_state = GameState::Port;
        self.dragged_item = None;
        self.message = prompts::state_prompt(self.state).to_owned();
        self.debug.visible = false;
        self.refresh_save_state();
    }
}
