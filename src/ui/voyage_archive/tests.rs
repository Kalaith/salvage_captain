use super::*;
use crate::state::WorkspaceScanProfile;

fn record(outcome: RiskOutcome, recovered_count: u32, recovered_value: i64) -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count,
        recovered_value,
        external_load: 2,
        risk_outcome: outcome,
        danger_score: 15,
        contract_completed: true,
        contract_failed: false,
        scan_profile: WorkspaceScanProfile::Standard,
        condition_after: 80,
        market_cycle: 0,
    }
}

#[test]
fn archive_header_names_the_number_of_filed_runs() {
    assert_eq!(archive_header(4), "04 RUN(S) FILED");
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
        "TOTAL HAUL  ¢370  //  BEST ¢250  //  SAFE 1/2  //  TARGETS 3  //  EXTERNAL 4  //  BP 05/09"
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
