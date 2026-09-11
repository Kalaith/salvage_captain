use super::*;
use crate::state::CareerStats;
use crate::state::WorkspaceScanProfile;

fn record(outcome: RiskOutcome, recovered_count: u32, recovered_value: i64) -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count,
        recovered_value,
        recovered_alloy: 5,
        recovered_electronics: 2,
        external_load: 2,
        risk_outcome: outcome,
        danger_score: 15,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 80,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

#[test]
fn archive_header_names_the_number_of_filed_runs() {
    assert_eq!(archive_header(4), "04 RUN(S) FILED");
}

#[test]
fn archive_header_names_award_progress_and_next_target() {
    let stats = CareerStats {
        voyages_completed: 1,
        ..CareerStats::default()
    };

    assert_eq!(archive_award_header(&stats), "AWARDS 01/7  //  NEXT CLEAN");
}

#[test]
fn archive_button_advertises_filed_runs_before_opening() {
    assert_eq!(archive_button_label(4, false), "LOG 04");
    assert_eq!(archive_button_label(4, true), "CLOSE");
}

#[test]
fn archive_page_exposes_older_runs_in_fixed_pages() {
    assert_eq!(archive_page(12, 0), (0, 5));
    assert_eq!(archive_page(12, 5), (5, 10));
    assert_eq!(archive_page(12, 10), (10, 12));
    assert_eq!(archive_page(3, 99), (2, 3));
}

#[test]
fn archive_filter_cycles_and_matches_authored_sites() {
    assert_eq!(ArchiveFilter::All.next(), ArchiveFilter::Merchant);
    assert_eq!(ArchiveFilter::Merchant.next(), ArchiveFilter::Military);
    assert_eq!(ArchiveFilter::Military.next(), ArchiveFilter::Research);
    assert_eq!(ArchiveFilter::Research.next(), ArchiveFilter::All);
    assert!(ArchiveFilter::All.matches("research_vessel"));
    assert!(ArchiveFilter::Research.matches("research_vessel"));
    assert!(!ArchiveFilter::Research.matches("merchant_wreck"));
    assert_eq!(
        archive_filter_button_label(ArchiveFilter::Military),
        "SITE  //  MILITARY"
    );
}

#[test]
fn archive_summary_totals_the_persistent_haul() {
    let records = vec![
        record(RiskOutcome::OrdinaryReturn, 2, 250),
        record(RiskOutcome::DamagedModule, 1, 120),
    ];

    assert_eq!(
        archive_summary(&records, 5, 9),
        "TOTAL HAUL  ¢370  //  BEST ¢250  //  SAFE 1/2  //  TARGETS 3  //  EXTERNAL 4  //  RETURN FUEL 4  //  MATS A10 E4  //  PREM ¢0  //  CLAIMS ¢0  //  CLEAR 0  //  BOUNTY ¢0  //  BP 05/09"
    );
}

#[test]
fn career_summary_names_the_lifetime_dossier() {
    let stats = CareerStats {
        voyages_completed: 7,
        safe_returns: 5,
        targets_recovered: 12,
        gross_haul_value: 3560,
        highest_haul_value: 1180,
        contracts_completed: 4,
        sections_cleared: 3,
        repairs_completed: 2,
        systems_restored: 2,
        repair_spend: 195,
        full_overhauls: 2,
        hull_patches: 0,
        systems_services: 0,
        field_power_cells_bought: 0,
        field_power_cells_used: 0,
        field_power_spend: 0,
        fuel_units_bought: 12,
        refuel_spend: 216,
        contract_income: 180,
        insurance_claims: 120,
        sale_income: 240,
        module_changes: 1,
        module_spend: 360,
    };

    assert_eq!(
        career_summary(&stats),
        "CAREER 07  //  SAFE 5/7  //  TGT 12  //  GROSS ¢3560  //  BEST ¢1180  //  CONTRACT 4  //  CLEAR 3  //  RANK FLEET FIXTURE  //  AWARDS 6/7"
    );
    assert_eq!(
        career_operations_summary(&stats),
        "OPERATING LEDGER  //  REPAIRS 2 F/H/S 2/0/0  //  SYSTEMS 2  //  SERVICE ¢195  //  CELLS +0 / -0 / ¢0  //  FUEL +12 / ¢216  //  CONTRACTS +¢180  //  CLAIMS +¢120  //  SALES +¢240  //  MODULES 1 / ¢360  //  NEXT MARKET MAKER"
    );
    assert_eq!(
        career_awards_summary(&stats),
        "COMMENDATIONS  //  FIRST RETURN / CLEAN RETURN / CONTRACT HAND / DEEP PULL / FRAME SURVEYOR / SYSTEMS VETERAN"
    );
}

#[test]
fn archive_entry_identifies_site_and_outcome() {
    let voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);

    assert_eq!(
        archive_entry_label(&voyage, 7, "Merchant Wreck"),
        "RUN 07  //  MERCHANT WRECK  //  ORDINARY RETURN"
    );
    assert_eq!(archive_contract_label(&voyage), "CONTRACT COMPLETE");
}

#[test]
fn archive_entry_keeps_the_market_cycle_beside_the_haul() {
    let mut voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);
    voyage.market_cycle = 7;

    assert_eq!(archive_market_label(&voyage), "MKT CYCLE 07");
}

#[test]
fn archive_entry_keeps_material_yields_beside_the_market_cycle() {
    let mut voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);
    voyage.recovered_alloy = 5;
    voyage.recovered_electronics = 2;

    assert_eq!(archive_material_label(&voyage), "MATS A5 E2");
}

#[test]
fn archive_entry_names_the_route_intelligence_level() {
    let mut voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);
    voyage.reconnaissance_level = 2;

    assert_eq!(archive_intelligence_label(&voyage), "INTEL L2");
}

#[test]
fn archive_entry_names_the_operating_plan() {
    let voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);

    assert_eq!(archive_plan_label(&voyage), "PLAN STANDARD");
}

#[test]
fn archive_entry_names_the_return_protection_policy() {
    let mut voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);
    voyage.return_policy = crate::state::ReturnPolicy::ProtectObjective;

    assert_eq!(archive_policy_label(&voyage), "POLICY OBJECTIVE");
}

#[test]
fn archive_entry_names_the_drone_order() {
    let mut voyage = record(RiskOutcome::OrdinaryReturn, 2, 250);
    voyage.drone_directive = crate::state::DroneDirective::Survey;

    assert_eq!(archive_drone_label(&voyage), "DRONE SURVEY");
}

#[test]
fn archive_entry_names_paid_coverage_claims() {
    let mut voyage = record(RiskOutcome::LostSalvage, 1, 80);
    voyage.insured = true;
    voyage.insurance_premium = 65;
    voyage.insurance_payout = 60;

    assert_eq!(archive_insurance_label(&voyage), "COVER ¢65 / CLAIM ¢60");
}
