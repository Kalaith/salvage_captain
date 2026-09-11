//! Persistent lifetime figures for a captain's salvage career.

use super::{RiskOutcome, VoyageRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CareerStats {
    pub voyages_completed: u32,
    pub safe_returns: u32,
    pub targets_recovered: u32,
    pub gross_haul_value: i64,
    pub highest_haul_value: i64,
    pub contracts_completed: u32,
    pub sections_cleared: u32,
    pub repairs_completed: u32,
    pub systems_restored: u32,
    pub repair_spend: i64,
    pub fuel_units_bought: i32,
    pub refuel_spend: i64,
    pub contract_income: i64,
    pub insurance_claims: i64,
    pub sale_income: i64,
    pub module_changes: u32,
    pub module_spend: i64,
}

impl CareerStats {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    pub fn record_voyage(&mut self, record: &VoyageRecord) {
        self.voyages_completed = self.voyages_completed.saturating_add(1);
        if record.risk_outcome == RiskOutcome::OrdinaryReturn {
            self.safe_returns = self.safe_returns.saturating_add(1);
        }
        self.targets_recovered = self
            .targets_recovered
            .saturating_add(record.recovered_count);
        self.gross_haul_value = self
            .gross_haul_value
            .saturating_add(record.recovered_value.max(0));
        self.highest_haul_value = self.highest_haul_value.max(record.recovered_value.max(0));
        if record.contract_completed {
            self.contracts_completed = self.contracts_completed.saturating_add(1);
        }
        self.sections_cleared = self
            .sections_cleared
            .saturating_add(record.cleared_sections.len() as u32);
    }

    pub fn from_voyage_log(records: &[VoyageRecord]) -> Self {
        let mut stats = Self::default();
        for record in records {
            stats.record_voyage(record);
            stats.record_insurance_claim(record.insurance_payout);
        }
        stats
    }

    pub fn record_repair(&mut self, total_cost: i64, systems_restored: usize) {
        self.repairs_completed = self.repairs_completed.saturating_add(1);
        self.systems_restored = self
            .systems_restored
            .saturating_add(systems_restored as u32);
        self.repair_spend = self.repair_spend.saturating_add(total_cost.max(0));
    }

    pub fn record_refuel(&mut self, units: i32, total_cost: i64) {
        self.fuel_units_bought = self.fuel_units_bought.saturating_add(units.max(0));
        self.refuel_spend = self.refuel_spend.saturating_add(total_cost.max(0));
    }

    pub fn record_contract_income(&mut self, total: i64) {
        self.contract_income = self.contract_income.saturating_add(total.max(0));
    }

    pub fn record_insurance_claim(&mut self, total: i64) {
        self.insurance_claims = self.insurance_claims.saturating_add(total.max(0));
    }

    pub fn record_sale(&mut self, total: i64) {
        self.sale_income = self.sale_income.saturating_add(total.max(0));
    }

    pub fn record_module_change(&mut self, total_cost: i64) {
        self.module_changes = self.module_changes.saturating_add(1);
        self.module_spend = self.module_spend.saturating_add(total_cost.max(0));
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.safe_returns > self.voyages_completed
            || self.contracts_completed > self.voyages_completed
            || self.highest_haul_value < 0
            || self.gross_haul_value < 0
            || self.highest_haul_value > self.gross_haul_value
            || self.repair_spend < 0
            || self.fuel_units_bought < 0
            || self.refuel_spend < 0
            || self.contract_income < 0
            || self.insurance_claims < 0
            || self.sale_income < 0
            || self.module_spend < 0
        {
            return Err("save contains invalid career totals".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
