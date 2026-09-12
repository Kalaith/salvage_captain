//! Deterministic coverage quote and claim rules.

use super::*;
use crate::data::{InsuranceTuning, SiteData};
use crate::engine::insurance::*;

fn site(danger: i32) -> SiteData {
    SiteData {
        id: "insured_site".to_owned(),
        display_name: "Insured Site".to_owned(),
        category: "Test".to_owned(),
        description: String::new(),
        fuel_cost: 4,
        danger,
        condition: 80,
        known_reward: String::new(),
        candidate_salvage: Vec::new(),
        wreck_class: String::new(),
        visual_theme: "merchant".to_owned(),
        background_asset: String::new(),
        hull_asset: String::new(),
        arrival_text: String::new(),
        contract_target: None,
        contract_reward: 0,
        contract_brief: String::new(),
        sections: Vec::new(),
    }
}

#[test]
fn insurance_premium_scales_with_authored_danger() {
    let tuning = InsuranceTuning {
        premium_base: 30,
        premium_per_danger: 2,
        coverage_percent: 75,
        damaged_module_payout: 90,
    };

    assert_eq!(quote_for(&site(15), &tuning).premium, 60);
    assert_eq!(quote_for(&site(70), &tuning).premium, 170);
}

#[test]
fn danger_quote_clamps_negative_route_inputs_before_pricing() {
    let tuning = InsuranceTuning {
        premium_base: 30,
        premium_per_danger: 2,
        coverage_percent: 75,
        damaged_module_payout: 90,
    };

    assert_eq!(quote_for_danger(-15, &tuning).premium, 30);
    assert_eq!(quote_for_danger(15, &tuning).premium, 60);
}

#[test]
fn claims_cover_only_the_authoritative_setback() {
    let tuning = InsuranceTuning {
        premium_base: 0,
        premium_per_danger: 0,
        coverage_percent: 75,
        damaged_module_payout: 90,
    };

    assert_eq!(claim_payout(RiskOutcome::LostSalvage, 200, 0, &tuning), 150);
    assert_eq!(
        claim_payout(RiskOutcome::EmergencyRepair, 0, 120, &tuning),
        90
    );
    assert_eq!(claim_payout(RiskOutcome::DamagedModule, 0, 0, &tuning), 67);
    assert_eq!(
        claim_payout(RiskOutcome::OrdinaryReturn, 200, 120, &tuning),
        0
    );
}
