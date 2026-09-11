use super::*;
use crate::engine::{RiskOutcome, VoyagePlan};
use crate::state::maintenance::ServicePlan;
use crate::state::{DroneDirective, WorkspaceScanProfile};

fn record(outcome: RiskOutcome, value: i64, target_count: u32) -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: target_count,
        recovered_value: value,
        recovered_alloy: 3,
        recovered_electronics: 2,
        external_load: 1,
        risk_outcome: outcome,
        danger_score: 20,
        reconnaissance_level: 0,
        voyage_plan: VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: outcome == RiskOutcome::OrdinaryReturn,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: DroneDirective::PullSupport,
        condition_after: 80,
        cleared_sections: vec!["cargo_bay".to_owned()],
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

#[test]
fn career_stats_rebuild_the_dossier_from_the_voyage_log() {
    let records = vec![
        record(RiskOutcome::OrdinaryReturn, 250, 2),
        record(RiskOutcome::LostSalvage, 120, 1),
    ];

    let stats = CareerStats::from_voyage_log(&records);

    assert_eq!(stats.voyages_completed, 2);
    assert_eq!(stats.safe_returns, 1);
    assert_eq!(stats.targets_recovered, 3);
    assert_eq!(stats.gross_haul_value, 370);
    assert_eq!(stats.highest_haul_value, 250);
    assert_eq!(stats.contracts_completed, 1);
    assert_eq!(stats.sections_cleared, 2);
    stats.validate().unwrap();
}

#[test]
fn career_stats_rebuild_contract_streaks_from_completed_runs() {
    let records = vec![
        record(RiskOutcome::OrdinaryReturn, 250, 2),
        record(RiskOutcome::OrdinaryReturn, 120, 1),
    ];

    let stats = CareerStats::from_voyage_log(&records);

    assert_eq!(stats.contract_streak, 2);
    assert_eq!(stats.best_contract_streak, 2);
}

#[test]
fn career_stats_keep_the_best_run_after_a_failed_contract() {
    let mut failed = record(RiskOutcome::LostSalvage, 120, 1);
    failed.contract_failed = true;
    let records = vec![
        record(RiskOutcome::OrdinaryReturn, 250, 2),
        record(RiskOutcome::OrdinaryReturn, 120, 1),
        failed,
        record(RiskOutcome::OrdinaryReturn, 180, 1),
    ];

    let stats = CareerStats::from_voyage_log(&records);

    assert_eq!(stats.contract_streak, 1);
    assert_eq!(stats.best_contract_streak, 2);
}

#[test]
fn career_stats_record_service_work() {
    let mut stats = CareerStats::default();

    stats.record_repair(160, 1);
    stats.record_repair(35, 0);

    assert_eq!(stats.repairs_completed, 2);
    assert_eq!(stats.systems_restored, 1);
    assert_eq!(stats.repair_spend, 195);
    assert_eq!(stats.full_overhauls, 2);
    stats.record_field_power_cell_purchase(80);
    stats.record_field_power_cell_fabrication(1, 1);
    stats.record_field_power_cell_use();
    assert_eq!(stats.field_power_cells_bought, 1);
    assert_eq!(stats.field_power_cells_fabricated, 1);
    assert_eq!(stats.field_power_alloy_used, 1);
    assert_eq!(stats.field_power_electronics_used, 1);
    assert_eq!(stats.field_power_cells_used, 1);
    assert_eq!(stats.field_power_spend, 80);
    stats.record_refuel(12, 216);
    assert_eq!(stats.fuel_units_bought, 12);
    assert_eq!(stats.refuel_spend, 216);
    stats.record_contract_income(180);
    stats.record_insurance_claim(120);
    stats.record_sale(240);
    stats.record_module_change(360);
    stats.record_cargo_bay_upgrade();
    assert_eq!(stats.contract_income, 180);
    assert_eq!(stats.insurance_claims, 120);
    assert_eq!(stats.sale_income, 240);
    assert_eq!(stats.module_changes, 1);
    assert_eq!(stats.module_spend, 360);
    assert_eq!(stats.cargo_bay_upgrades, 1);
    stats.validate().unwrap();
}

#[test]
fn career_stats_keep_the_service_plan_mix() {
    let mut stats = CareerStats::default();

    stats.record_service(ServicePlan::Hull, 35, 0);
    stats.record_service(ServicePlan::Systems, 60, 1);
    stats.record_service(ServicePlan::Full, 95, 2);

    assert_eq!(stats.repairs_completed, 3);
    assert_eq!(stats.full_overhauls, 1);
    assert_eq!(stats.hull_patches, 1);
    assert_eq!(stats.systems_services, 1);
    assert_eq!(stats.systems_restored, 3);
    assert_eq!(stats.repair_spend, 190);
}

#[test]
fn career_stats_count_qualified_crew_specialties() {
    let mut stats = CareerStats::default();
    stats.crew_experience = [0, 3, 6, 2, 0];

    assert_eq!(stats.crew_qualified_count(), 2);
}

#[test]
fn career_stats_reject_impossible_totals() {
    let stats = CareerStats {
        voyages_completed: 1,
        safe_returns: 2,
        ..CareerStats::default()
    };

    assert_eq!(
        stats.validate().unwrap_err(),
        "save contains invalid career or crew totals"
    );
}

#[test]
fn career_stats_reject_overtrained_crew_profiles() {
    let mut stats = CareerStats::default();
    stats.crew_experience[0] = crate::state::crew::MAX_CREW_EXPERIENCE + 1;

    assert_eq!(
        stats.validate().unwrap_err(),
        "save contains invalid career or crew totals"
    );
}

#[test]
fn career_stats_reject_a_streak_beyond_its_recorded_best() {
    let stats = CareerStats {
        contract_streak: 3,
        best_contract_streak: 2,
        ..CareerStats::default()
    };

    assert_eq!(
        stats.validate().unwrap_err(),
        "save contains invalid career or crew totals"
    );
}

#[test]
fn career_awards_turn_lifetime_figures_into_a_rank() {
    let stats = CareerStats {
        voyages_completed: 4,
        safe_returns: 3,
        highest_haul_value: 1_200,
        contracts_completed: 3,
        sale_income: 1_050,
        ..CareerStats::default()
    };

    assert_eq!(stats.rank_label(), "FLEET FIXTURE");
    assert_eq!(stats.rank_code(), "FLEET");
    assert_eq!(stats.earned_awards().len(), 5);
    assert_eq!(stats.next_award(), Some(CareerAward::FrameSurveyor));
    assert!(stats.earned_awards().contains(&CareerAward::ContractHand));
}
