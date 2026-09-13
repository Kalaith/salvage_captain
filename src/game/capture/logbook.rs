//! Deterministic archive fixture used by the logbook verification scene.

use super::Game;
use crate::engine::RiskOutcome;
use crate::state::{VoyageRecord, WorkspaceScanProfile};

pub(super) fn prepare(game: &mut Game) {
    game.session.voyage_log = records();
    game.voyage_archive_open = true;
}

pub(super) fn configure(game: &mut Game, scene: &str) {
    use crate::ui::voyage_archive::{ArchiveAction, ArchiveFilter, ArchiveTab};
    let actions = match scene {
        "logbook_preparation" => vec![ArchiveAction::Page(1)],
        "logbook_claim" => vec![ArchiveAction::Select(5), ArchiveAction::Page(1)],
        "logbook_older" => vec![ArchiveAction::Older],
        "logbook_filtered" => vec![ArchiveAction::Filter(ArchiveFilter::Military)],
        "logbook_career" => vec![ArchiveAction::Tab(ArchiveTab::Career)],
        "logbook_totals" => vec![
            ArchiveAction::Tab(ArchiveTab::Career),
            ArchiveAction::Page(1),
        ],
        "logbook_service" => vec![ArchiveAction::Tab(ArchiveTab::Ledger)],
        "logbook_supplies" => vec![
            ArchiveAction::Tab(ArchiveTab::Ledger),
            ArchiveAction::Page(1),
        ],
        "logbook_income" => vec![
            ArchiveAction::Tab(ArchiveTab::Ledger),
            ArchiveAction::Page(2),
        ],
        "logbook_awards" => vec![ArchiveAction::Tab(ArchiveTab::Awards)],
        "logbook_empty" => {
            game.session.voyage_log.clear();
            game.session.career = Default::default();
            vec![]
        }
        "logbook_no_matches" => {
            game.session
                .voyage_log
                .retain(|record| record.site_id == "merchant_wreck");
            vec![ArchiveAction::Filter(ArchiveFilter::Research)]
        }
        _ => vec![],
    };
    for action in actions {
        game.voyage_archive.apply(action, &game.session.voyage_log);
    }
}

pub(super) fn private_record() -> VoyageRecord {
    records().pop().expect("private haul fixture")
}

fn records() -> Vec<VoyageRecord> {
    vec![
        merchant_opening(),
        military_damage(),
        research_contract(),
        merchant_loss(),
        military_contract(),
        research_insured(),
        merchant_follow_up(),
        merchant_private(),
    ]
}

fn merchant_opening() -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 2,
        recovered_value: 250,
        recovered_alloy: 5,
        recovered_electronics: 2,
        external_load: 1,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Cautious,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 74,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 0,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn military_damage() -> VoyageRecord {
    VoyageRecord {
        site_id: "military_wreck".to_owned(),
        recovered_count: 1,
        recovered_value: 520,
        recovered_alloy: 5,
        recovered_electronics: 6,
        external_load: 2,
        risk_outcome: RiskOutcome::DamagedModule,
        danger_score: 45,
        reconnaissance_level: 1,
        voyage_plan: crate::engine::VoyagePlan::Expedited,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: false,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Array,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 48,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 1,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn research_contract() -> VoyageRecord {
    VoyageRecord {
        site_id: "research_vessel".to_owned(),
        recovered_count: 3,
        recovered_value: 1_180,
        recovered_alloy: 3,
        recovered_electronics: 17,
        external_load: 1,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 70,
        reconnaissance_level: 2,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Array,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 68,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 2,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn merchant_loss() -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 1,
        recovered_value: 160,
        recovered_alloy: 2,
        recovered_electronics: 3,
        external_load: 0,
        risk_outcome: RiskOutcome::LostSalvage,
        danger_score: 15,
        reconnaissance_level: 0,
        voyage_plan: crate::engine::VoyagePlan::Expedited,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: false,
        contract_failed: true,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 60,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 3,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn military_contract() -> VoyageRecord {
    VoyageRecord {
        site_id: "military_wreck".to_owned(),
        recovered_count: 2,
        recovered_value: 700,
        recovered_alloy: 9,
        recovered_electronics: 8,
        external_load: 3,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 45,
        reconnaissance_level: 1,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Array,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 42,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 4,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn research_insured() -> VoyageRecord {
    VoyageRecord {
        site_id: "research_vessel".to_owned(),
        recovered_count: 1,
        recovered_value: 420,
        recovered_alloy: 2,
        recovered_electronics: 7,
        external_load: 1,
        risk_outcome: RiskOutcome::EmergencyRepair,
        danger_score: 70,
        reconnaissance_level: 2,
        voyage_plan: crate::engine::VoyagePlan::Expedited,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: false,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Standard,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 51,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 5,
        insured: true,
        insurance_premium: 175,
        insurance_payout: 120,
    }
}

fn merchant_follow_up() -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 2,
        recovered_value: 330,
        recovered_alloy: 6,
        recovered_electronics: 3,
        external_load: 1,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        reconnaissance_level: 1,
        voyage_plan: crate::engine::VoyagePlan::Cautious,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: true,
        contract_failed: false,
        contract_accepted: true,
        scan_profile: WorkspaceScanProfile::Array,
        drone_directive: crate::state::DroneDirective::PullSupport,
        condition_after: 53,
        cleared_sections: Vec::new(),
        clearance_payout: 0,
        return_fuel: 2,
        market_cycle: 6,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}

fn merchant_private() -> VoyageRecord {
    VoyageRecord {
        site_id: "merchant_wreck".to_owned(),
        recovered_count: 1,
        recovered_value: 210,
        recovered_alloy: 4,
        recovered_electronics: 2,
        external_load: 0,
        risk_outcome: RiskOutcome::OrdinaryReturn,
        danger_score: 15,
        reconnaissance_level: 1,
        voyage_plan: crate::engine::VoyagePlan::Standard,
        return_policy: crate::state::ReturnPolicy::default(),
        contract_completed: false,
        contract_failed: false,
        contract_accepted: false,
        scan_profile: WorkspaceScanProfile::Array,
        drone_directive: crate::state::DroneDirective::Survey,
        condition_after: 45,
        cleared_sections: vec!["cargo_bay".to_owned()],
        clearance_payout: 70,
        return_fuel: 2,
        market_cycle: 7,
        insured: false,
        insurance_premium: 0,
        insurance_payout: 0,
    }
}
