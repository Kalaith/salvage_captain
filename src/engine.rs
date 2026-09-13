//! Pure simulation services for packing, travel, economy, risk, and upgrades.

pub mod discovery;
pub mod economy;
pub mod expedition;
pub mod installation;
pub mod insurance;
pub mod market;
pub mod packing;
pub mod progression;
pub mod reconnaissance;
pub mod refinery;
pub mod risk;
pub mod voyage_plan;
pub mod workspace;

pub use economy::{resolve_disposition, Disposition};
pub use expedition::generate_salvage;
pub use insurance::{
    claim_payout, quote_for as insurance_quote_for, quote_for_danger as insurance_quote_for_danger,
    InsuranceQuote,
};
pub use packing::ShipLayout;
pub use progression::ModuleStats;
pub use reconnaissance::{
    danger_after_intel, quote_for as reconnaissance_quote_for, ReconnaissanceQuote,
};
pub use risk::{resolve_risk, RiskOutcome, RiskResult};
pub use voyage_plan::VoyagePlan;
pub use workspace::{
    exposure_label, resolve_extraction, ExtractionRequest, WorkspaceHazard, WorkspaceOutcome,
    WorkspaceRiskReport,
};
