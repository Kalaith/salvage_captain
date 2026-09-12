//! Deterministic station demand bands for returned salvage.

use crate::data::{MarketTuning, SalvageObjectData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketBand {
    Hot,
    Steady,
    Soft,
}

impl MarketBand {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hot => "HOT",
            Self::Steady => "STEADY",
            Self::Soft => "SOFT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketQuote {
    pub band: MarketBand,
    pub multiplier_percent: i32,
    pub sale_value: i64,
}

impl MarketQuote {
    pub const fn signed_multiplier(self) -> i32 {
        self.multiplier_percent - 100
    }
}

pub fn quote_for(object: &SalvageObjectData, cycle: u32, tuning: &MarketTuning) -> MarketQuote {
    let signal = stable_signal(&object.market_group);
    let band = match (signal.wrapping_add(u64::from(cycle))) % 3 {
        0 => MarketBand::Hot,
        1 => MarketBand::Steady,
        _ => MarketBand::Soft,
    };
    let multiplier_percent = match band {
        MarketBand::Hot => 100 + tuning.hot_bonus_percent,
        MarketBand::Steady => 100,
        MarketBand::Soft => 100 - tuning.soft_penalty_percent,
    };
    let sale_value = object.sale_value * i64::from(multiplier_percent) / 100;
    MarketQuote {
        band,
        multiplier_percent,
        sale_value: sale_value.max(0),
    }
}

fn stable_signal(group: &str) -> u64 {
    group.bytes().fold(17_u64, |signal, byte| {
        signal
            .wrapping_mul(31)
            .wrapping_add(u64::from(byte.to_ascii_lowercase()))
    })
}
