//! Pure simulation services for packing, travel, economy, risk, and upgrades.

pub mod economy;
pub mod expedition;
pub mod installation;
pub mod packing;
pub mod progression;
pub mod risk;

pub use economy::{resolve_disposition, Disposition};
pub use expedition::generate_salvage;
pub use packing::ShipLayout;
pub use progression::ModuleStats;
pub use risk::{resolve_risk, RiskOutcome, RiskResult};
