//! Persistent lifetime figures for a captain's salvage career.

use super::{RiskOutcome, VoyageRecord};
use crate::state::crew::{CREW_EXPERIENCE_PER_LEVEL, MAX_CREW_EXPERIENCE};
use crate::state::maintenance::ServicePlan;
use serde::{Deserialize, Serialize};

pub const CONTRACT_STREAK_BONUS_STEP: i64 = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CareerAward {
    FirstReturn,
    CleanReturn,
    ContractHand,
    DeepPull,
    FrameSurveyor,
    MarketMaker,
    SystemsVeteran,
}

impl CareerAward {
    pub const ALL: [Self; 7] = [
        Self::FirstReturn,
        Self::CleanReturn,
        Self::ContractHand,
        Self::DeepPull,
        Self::FrameSurveyor,
        Self::MarketMaker,
        Self::SystemsVeteran,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstReturn => "FIRST RETURN",
            Self::CleanReturn => "CLEAN RETURN",
            Self::ContractHand => "CONTRACT HAND",
            Self::DeepPull => "DEEP PULL",
            Self::FrameSurveyor => "FRAME SURVEYOR",
            Self::MarketMaker => "MARKET MAKER",
            Self::SystemsVeteran => "SYSTEMS VETERAN",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::FirstReturn => "FIRST",
            Self::CleanReturn => "CLEAN",
            Self::ContractHand => "CONTRACT",
            Self::DeepPull => "DEEP",
            Self::FrameSurveyor => "FRAME",
            Self::MarketMaker => "MARKET",
            Self::SystemsVeteran => "SYSTEMS",
        }
    }

    pub const fn is_earned(self, stats: &CareerStats) -> bool {
        match self {
            Self::FirstReturn => stats.voyages_completed >= 1,
            Self::CleanReturn => stats.safe_returns >= 3,
            Self::ContractHand => stats.contracts_completed >= 3,
            Self::DeepPull => stats.highest_haul_value >= 1_000,
            Self::FrameSurveyor => stats.sections_cleared >= 3,
            Self::MarketMaker => stats.sale_income >= 1_000,
            Self::SystemsVeteran => stats.repairs_completed >= 2,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CareerStats {
    pub voyages_completed: u32,
    pub safe_returns: u32,
    pub targets_recovered: u32,
    pub gross_haul_value: i64,
    pub highest_haul_value: i64,
    pub contracts_completed: u32,
    #[serde(default)]
    pub contract_streak: u32,
    #[serde(default)]
    pub best_contract_streak: u32,
    pub sections_cleared: u32,
    pub repairs_completed: u32,
    pub systems_restored: u32,
    pub repair_spend: i64,
    #[serde(default)]
    pub full_overhauls: u32,
    #[serde(default)]
    pub hull_patches: u32,
    #[serde(default)]
    pub systems_services: u32,
    #[serde(default)]
    pub field_power_cells_bought: u32,
    #[serde(default)]
    pub field_power_cells_fabricated: u32,
    #[serde(default)]
    pub field_power_alloy_used: u32,
    #[serde(default)]
    pub field_power_electronics_used: u32,
    #[serde(default)]
    pub crew_experience: [u16; 5],
    #[serde(default)]
    pub field_power_cells_used: u32,
    #[serde(default)]
    pub field_power_spend: i64,
    pub fuel_units_bought: i32,
    pub refuel_spend: i64,
    pub contract_income: i64,
    pub insurance_claims: i64,
    pub sale_income: i64,
    pub module_changes: u32,
    pub module_spend: i64,
    #[serde(default)]
    pub cargo_bay_upgrades: u32,
}

impl CareerStats {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    pub fn crew_qualified_count(&self) -> usize {
        self.crew_experience
            .iter()
            .filter(|experience| **experience >= CREW_EXPERIENCE_PER_LEVEL)
            .count()
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
            if record.contract_accepted && record.contract_completed {
                stats.record_contract_success();
            } else if record.contract_accepted && record.contract_failed {
                stats.record_contract_failure();
            }
        }
        stats
    }

    pub fn record_repair(&mut self, total_cost: i64, systems_restored: usize) {
        self.record_service(ServicePlan::Full, total_cost, systems_restored);
    }

    pub fn record_service(&mut self, plan: ServicePlan, total_cost: i64, systems_restored: usize) {
        self.repairs_completed = self.repairs_completed.saturating_add(1);
        self.systems_restored = self
            .systems_restored
            .saturating_add(systems_restored as u32);
        self.repair_spend = self.repair_spend.saturating_add(total_cost.max(0));
        match plan {
            ServicePlan::Full => self.full_overhauls = self.full_overhauls.saturating_add(1),
            ServicePlan::Hull => self.hull_patches = self.hull_patches.saturating_add(1),
            ServicePlan::Systems => self.systems_services = self.systems_services.saturating_add(1),
        }
    }

    pub fn record_refuel(&mut self, units: i32, total_cost: i64) {
        self.fuel_units_bought = self.fuel_units_bought.saturating_add(units.max(0));
        self.refuel_spend = self.refuel_spend.saturating_add(total_cost.max(0));
    }

    pub fn record_field_power_cell_purchase(&mut self, total_cost: i64) {
        self.field_power_cells_bought = self.field_power_cells_bought.saturating_add(1);
        self.field_power_spend = self.field_power_spend.saturating_add(total_cost.max(0));
    }

    pub fn record_field_power_cell_fabrication(&mut self, alloy: i32, electronics: i32) {
        self.field_power_cells_fabricated = self.field_power_cells_fabricated.saturating_add(1);
        self.field_power_alloy_used = self
            .field_power_alloy_used
            .saturating_add(alloy.max(0) as u32);
        self.field_power_electronics_used = self
            .field_power_electronics_used
            .saturating_add(electronics.max(0) as u32);
    }

    pub fn record_field_power_cell_use(&mut self) {
        self.field_power_cells_used = self.field_power_cells_used.saturating_add(1);
    }

    pub fn record_contract_income(&mut self, total: i64) {
        self.contract_income = self.contract_income.saturating_add(total.max(0));
    }

    pub fn record_contract_success(&mut self) -> (u32, i64) {
        self.contract_streak = self.contract_streak.saturating_add(1);
        self.best_contract_streak = self.best_contract_streak.max(self.contract_streak);
        let bonus = i64::from(self.contract_streak.saturating_sub(1)) * CONTRACT_STREAK_BONUS_STEP;
        (self.contract_streak, bonus)
    }

    pub fn record_contract_failure(&mut self) {
        self.contract_streak = 0;
    }

    pub fn next_contract_streak_bonus(&self) -> i64 {
        i64::from(self.contract_streak) * CONTRACT_STREAK_BONUS_STEP
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

    pub fn record_cargo_bay_upgrade(&mut self) {
        self.cargo_bay_upgrades = self.cargo_bay_upgrades.saturating_add(1);
    }

    pub fn earned_awards(&self) -> Vec<CareerAward> {
        CareerAward::ALL
            .into_iter()
            .filter(|award| award.is_earned(self))
            .collect()
    }

    pub fn next_award(&self) -> Option<CareerAward> {
        CareerAward::ALL
            .into_iter()
            .find(|award| !award.is_earned(self))
    }

    pub fn rank_label(&self) -> &'static str {
        match self.earned_awards().len() {
            0 => "UNRANKED",
            1..=2 => "WORKING CAPTAIN",
            3..=4 => "SEASONED SALVOR",
            5..=6 => "FLEET FIXTURE",
            _ => "SCRAPLANE LEGEND",
        }
    }

    pub fn rank_code(&self) -> &'static str {
        match self.earned_awards().len() {
            0 => "UNRANKED",
            1..=2 => "WORKING",
            3..=4 => "SEASONED",
            5..=6 => "FLEET",
            _ => "LEGEND",
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.safe_returns > self.voyages_completed
            || self.contracts_completed > self.voyages_completed
            || self.highest_haul_value < 0
            || self.gross_haul_value < 0
            || self.highest_haul_value > self.gross_haul_value
            || self.best_contract_streak < self.contract_streak
            || self.repair_spend < 0
            || self.fuel_units_bought < 0
            || self.refuel_spend < 0
            || self.contract_income < 0
            || self.insurance_claims < 0
            || self.sale_income < 0
            || self.module_spend < 0
            || self
                .crew_experience
                .iter()
                .any(|experience| *experience > MAX_CREW_EXPERIENCE)
        {
            return Err("save contains invalid career or crew totals".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
