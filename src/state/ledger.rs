//! Persistent voyage records that make each completed haul part of the ship's history.

use super::{DroneDirective, GameSession, ReturnPolicy, VoyageRecord, WorkspaceScanProfile};
use crate::data::GameData;
use crate::engine::{market, RiskResult, VoyagePlan};

pub struct VoyageRecordInput<'a> {
    pub site_id: &'a str,
    pub risk: &'a RiskResult,
    pub voyage_plan: VoyagePlan,
    pub return_policy: ReturnPolicy,
    pub reconnaissance_level: u8,
    pub external_load: i32,
    pub contract_completed: bool,
    pub contract_failed: bool,
    pub contract_accepted: bool,
    pub scan_profile: WorkspaceScanProfile,
    pub drone_directive: DroneDirective,
    pub condition_after: i32,
    pub return_fuel: i32,
    pub cleared_sections: &'a [String],
    pub clearance_payout: i64,
    pub insured: bool,
    pub insurance_premium: i64,
    pub insurance_payout: i64,
    pub data: &'a GameData,
}

impl GameSession {
    pub fn last_voyage(&self) -> Option<&VoyageRecord> {
        self.voyage_log.last()
    }

    pub fn record_voyage(&mut self, input: VoyageRecordInput<'_>) {
        let VoyageRecordInput {
            site_id,
            risk,
            voyage_plan,
            return_policy,
            reconnaissance_level,
            external_load,
            contract_completed,
            contract_failed,
            contract_accepted,
            scan_profile,
            drone_directive,
            condition_after,
            return_fuel,
            cleared_sections,
            clearance_payout,
            insured,
            insurance_premium,
            insurance_payout,
            data,
        } = input;
        let recovered: Vec<_> = self
            .returned
            .iter()
            .filter_map(|item| data.salvage_objects.get(&item.object_id))
            .collect();
        let record = VoyageRecord {
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
            return_policy,
            contract_completed,
            contract_failed,
            contract_accepted,
            scan_profile,
            drone_directive,
            condition_after,
            cleared_sections: cleared_sections.to_vec(),
            clearance_payout,
            return_fuel: return_fuel.max(0),
            insured,
            insurance_premium,
            insurance_payout,
            market_cycle: self.market_cycle,
        };
        self.career.record_voyage(&record);
        self.record_crew_experience(risk.outcome, contract_completed);
        self.voyage_log.push(record);
    }
}
