//! Deterministic route-intelligence quotes and danger adjustments.

use crate::data::ReconnaissanceTuning;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconnaissanceQuote {
    pub current_level: u8,
    pub next_level: u8,
    pub max_level: u8,
    pub cost: i64,
    pub danger_reduction: i32,
}

pub fn quote_for(level: u8, tuning: &ReconnaissanceTuning) -> Option<ReconnaissanceQuote> {
    if level >= tuning.max_level {
        return None;
    }
    Some(ReconnaissanceQuote {
        current_level: level,
        next_level: level.saturating_add(1),
        max_level: tuning.max_level,
        cost: tuning
            .first_cost
            .saturating_add(i64::from(level).saturating_mul(tuning.cost_step))
            .max(0),
        danger_reduction: tuning
            .danger_reduction_per_level
            .saturating_mul(i32::from(level.saturating_add(1))),
    })
}

pub fn danger_after_intel(base_danger: i32, level: u8, tuning: &ReconnaissanceTuning) -> i32 {
    base_danger
        .saturating_sub(
            tuning
                .danger_reduction_per_level
                .saturating_mul(i32::from(level)),
        )
        .clamp(0, 100)
}
