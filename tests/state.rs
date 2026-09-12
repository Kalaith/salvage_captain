//! Integration suites for persistent session state and progression rules.

pub mod data {
    pub use salvage_captain::data::*;
}
pub mod engine {
    pub use salvage_captain::engine::*;
}
pub mod state {
    pub use salvage_captain::state::*;
}

pub use data::GameData;
pub use engine::{Disposition, RiskOutcome, RiskResult, VoyagePlan};
pub use state::career::*;
pub use state::cargo_bay::*;
pub use state::contracts::*;
pub use state::crew::*;
pub use state::loadout::*;
pub use state::maintenance::*;
pub use state::reputation::*;
pub use state::return_policy::*;
pub use state::route_familiarity::*;
pub use state::section_clearance::*;
pub use state::workspace::*;
pub use state::workspace_energy::*;
pub use state::*;

#[path = "state/career.rs"]
mod career_tests;
#[path = "state/cargo_bay.rs"]
mod cargo_bay_tests;
#[path = "state/contracts.rs"]
mod contracts_tests;
#[path = "state/crew_readiness.rs"]
mod crew_readiness_tests;
#[path = "state/crew.rs"]
mod crew_tests;
#[path = "state/drone_directive.rs"]
mod drone_directive_tests;
#[path = "state/expedition.rs"]
mod expedition_tests;
#[path = "state/insurance.rs"]
mod insurance_tests;
#[path = "state/ledger.rs"]
mod ledger_tests;
#[path = "state/loadout.rs"]
mod loadout_tests;
#[path = "state/maintenance.rs"]
mod maintenance_tests;
#[path = "state/market.rs"]
mod market_tests;
#[path = "state/progression.rs"]
mod progression_tests;
#[path = "state/reconnaissance.rs"]
mod reconnaissance_tests;
#[path = "state/refinery.rs"]
mod refinery_tests;
#[path = "state/reputation.rs"]
mod reputation_tests;
#[path = "state/return_policy.rs"]
mod return_policy_tests;
#[path = "state/route_familiarity.rs"]
mod route_familiarity_tests;
#[path = "state/scan_profile.rs"]
mod scan_profile_tests;
#[path = "state/section_clearance.rs"]
mod section_clearance_tests;
#[path = "state/session.rs"]
mod session_tests;
#[path = "state/ship_wear.rs"]
mod ship_wear_tests;
#[path = "state/workspace.rs"]
mod workspace_tests;
