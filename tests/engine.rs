//! Integration suites for deterministic simulation services.

pub mod data {
    pub use salvage_captain::data::*;
}
pub mod engine {
    pub use salvage_captain::engine::*;
}
pub mod state {
    pub use salvage_captain::state::*;
}

pub use engine::{
    Disposition, ExtractionRequest, ModuleStats, RiskOutcome, RiskResult, ShipLayout, VoyagePlan,
    WorkspaceHazard, WorkspaceOutcome, WorkspaceRiskReport,
};

#[path = "engine/expedition.rs"]
mod expedition_tests;
#[path = "engine/installation.rs"]
mod installation_tests;
#[path = "engine/insurance.rs"]
mod insurance_tests;
#[path = "engine/market.rs"]
mod market_tests;
#[path = "engine/packing.rs"]
mod packing_tests;
#[path = "engine/progression.rs"]
mod progression_tests;
#[path = "engine/reconnaissance.rs"]
mod reconnaissance_tests;
#[path = "engine/refinery.rs"]
mod refinery_tests;
#[path = "engine/risk.rs"]
mod risk_tests;
#[path = "engine/voyage_plan.rs"]
mod voyage_plan_tests;
#[path = "engine/workspace.rs"]
mod workspace_tests;
