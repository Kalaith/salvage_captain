//! Authored copy for the salvage workspace.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SalvageUiCopy {
    pub select_target: String,
    pub scan_hint: String,
    pub select_hint: String,
    pub details: String,
    pub close: String,
    pub extract: String,
    pub stabilize: String,
    pub abandon: String,
    pub return_to_hold: String,
    pub back_to_wreck: String,
    pub cancel: String,
    pub crew_tired: String,
    pub no_hazard: String,
    pub objective: String,
    pub recovered: String,
    pub removed: String,
    pub exposure: String,
    pub mitigation: String,
    pub danger: String,
    pub value: String,
    pub power: String,
    pub integrity: String,
    pub mass: String,
    pub duration: String,
    pub tractor: String,
    pub condition: String,
    pub inspection: String,
    pub field: String,
}
