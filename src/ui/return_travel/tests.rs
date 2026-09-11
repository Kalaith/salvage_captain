use super::*;

#[test]
fn return_manifest_prefers_the_locked_voyage_value() {
    assert_eq!(return_manifest_value(None, 410), 410);
    assert_eq!(
        return_manifest_value(Some(&record_with_value(250)), 410),
        250
    );
}

#[test]
fn return_flight_report_keeps_the_haul_protection_policy_visible() {
    let record = record_with_value(250);

    assert_eq!(
        return_flight_report_label(&record, crate::state::ReturnPolicy::ProtectValue),
        "FLIGHT REPORT  //  ORDINARY RETURN  //  PLAN STANDARD  //  POLICY VALUE  //  DRONE PULL  //  SCAN STANDARD"
    );
}

#[test]
fn return_phase_moves_from_wreck_to_docked() {
    assert_eq!(return_phase(0.0), "DEPARTING WRECK");
    assert_eq!(return_phase(0.5), "RETURN CRUISE");
    assert_eq!(return_phase(0.9), "YARD APPROACH");
    assert_eq!(return_phase(1.0), "DOCKED");
}

#[test]
fn return_eta_closes_at_the_yard() {
    assert_eq!(return_eta(0.0), "3s");
    assert_eq!(return_eta(0.51), "2s");
    assert_eq!(return_eta(1.0), "NOW");
}

fn record_with_value(recovered_value: i64) -> crate::state::VoyageRecord {
    crate::state::VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 1,
        recovered_value,
        recovered_alloy: 0,
        recovered_electronics: 0,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        contract_completed: false,
        contract_failed: false,
        scan_profile: crate::state::WorkspaceScanProfile::Standard,
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
