//! Data-configured conversion of recovered materials into shipyard credit.

use crate::data::RefineryTuning;
use crate::state::EconomyState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefineryResource {
    Alloy,
    Electronics,
}

impl RefineryResource {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Alloy => "ALLOY",
            Self::Electronics => "ELEC",
        }
    }

    pub const fn available(self, economy: EconomyState) -> i32 {
        match self {
            Self::Alloy => economy.alloy,
            Self::Electronics => economy.electronics,
        }
    }

    pub const fn batch_size(self, tuning: &RefineryTuning) -> i32 {
        match self {
            Self::Alloy => tuning.alloy_batch,
            Self::Electronics => tuning.electronics_batch,
        }
    }

    pub const fn payout(self, tuning: &RefineryTuning) -> i64 {
        match self {
            Self::Alloy => tuning.alloy_payout,
            Self::Electronics => tuning.electronics_payout,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefineryQuote {
    pub resource: RefineryResource,
    pub available: i32,
    pub batch_size: i32,
    pub payout: i64,
}

impl RefineryQuote {
    pub const fn batches_available(self) -> i32 {
        self.available / self.batch_size
    }

    pub const fn can_refine(self) -> bool {
        self.batches_available() > 0
    }
}

pub fn quote_for(
    resource: RefineryResource,
    economy: EconomyState,
    tuning: &RefineryTuning,
) -> RefineryQuote {
    RefineryQuote {
        resource,
        available: resource.available(economy).max(0),
        batch_size: resource.batch_size(tuning),
        payout: resource.payout(tuning),
    }
}
