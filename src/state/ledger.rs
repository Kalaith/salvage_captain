//! Persistent voyage records that make each completed haul part of the ship's history.

use super::{GameSession, VoyageRecord, WorkspaceScanProfile};
use crate::data::GameData;
use crate::engine::{market, RiskResult};

impl GameSession {
    pub fn last_voyage(&self) -> Option<&VoyageRecord> {
        self.voyage_log.last()
    }

    pub fn record_voyage(
        &mut self,
        site_id: &str,
        risk: &RiskResult,
        external_load: i32,
        contract_completed: bool,
        contract_failed: bool,
        scan_profile: WorkspaceScanProfile,
        condition_after: i32,
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
            contract_completed,
            contract_failed,
            scan_profile,
            condition_after,
            market_cycle: self.market_cycle,
        });
    }
}

#[cfg(test)]
mod tests;
