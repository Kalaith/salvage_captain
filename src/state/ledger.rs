//! Persistent voyage records that make each completed haul part of the ship's history.

use super::{GameSession, VoyageRecord, WorkspaceScanProfile};
use crate::data::GameData;
use crate::engine::{market, RiskResult, VoyagePlan};

impl GameSession {
    pub fn last_voyage(&self) -> Option<&VoyageRecord> {
        self.voyage_log.last()
    }

    pub fn record_voyage(
        &mut self,
        site_id: &str,
        risk: &RiskResult,
        voyage_plan: VoyagePlan,
        reconnaissance_level: u8,
        external_load: i32,
        contract_completed: bool,
        contract_failed: bool,
        scan_profile: WorkspaceScanProfile,
        condition_after: i32,
        return_fuel: i32,
        cleared_sections: &[String],
        clearance_payout: i64,
        insured: bool,
        insurance_premium: i64,
        insurance_payout: i64,
        data: &GameData,
    ) {
        let recovered: Vec<_> = self
            .returned
            .iter()
            .filter_map(|item| data.salvage_objects.get(&item.object_id))
            .collect();
        self.voyage_log.push(VoyageRecord {
            site_id: site_id.to_owned(),
            recovered_count: recovered.len() as u32,
            recovered_value: self
                .returned
                .iter()
                .filter_map(|item| {
                    data.salvage_objects.get(&item.object_id).map(|object| {
                        market::quote_for(object, item.market_cycle, &data.config.market).sale_value
                    })
                })
                .sum(),
            recovered_alloy: recovered.iter().map(|object| object.alloy_yield).sum(),
            recovered_electronics: recovered
                .iter()
                .map(|object| object.electronics_yield)
                .sum(),
            external_load: external_load.max(0) as u32,
            risk_outcome: risk.outcome,
            danger_score: risk.danger_score,
            reconnaissance_level,
            voyage_plan,
            contract_completed,
            contract_failed,
            scan_profile,
            condition_after,
            cleared_sections: cleared_sections.to_vec(),
            clearance_payout,
            return_fuel: return_fuel.max(0),
            insured,
            insurance_premium,
            insurance_payout,
            market_cycle: self.market_cycle,
        });
    }
}

#[cfg(test)]
mod tests;
