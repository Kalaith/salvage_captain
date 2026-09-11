use super::*;

#[test]
fn travel_phase_advances_in_order() {
    assert_eq!(travel_phase(0.0), TravelPhase::Departure);
    assert_eq!(travel_phase(0.4), TravelPhase::Cruise);
    assert_eq!(travel_phase(0.9), TravelPhase::FinalApproach);
    assert_eq!(travel_phase(1.0), TravelPhase::Docked);
}

#[test]
fn travel_phase_clamps_out_of_range_progress() {
    assert_eq!(travel_phase(-0.5), TravelPhase::Departure);
    assert_eq!(travel_phase(1.5), TravelPhase::Docked);
}

#[test]
fn travel_duration_is_the_shared_docking_endpoint() {
    assert_eq!(TRAVEL_DURATION_SECONDS, 4.0);
    assert_eq!(travel_phase(1.0), TravelPhase::Docked);
}

#[test]
fn travel_eta_reports_seconds_then_now() {
    assert_eq!(travel_eta_label(0.0), "4s");
    assert_eq!(travel_eta_label(0.51), "2s");
    assert_eq!(travel_eta_label(1.0), "NOW");
}

#[test]
fn transit_wake_shortens_as_docking_approaches() {
    assert_eq!(wake_segment_count(TravelPhase::Departure), 5);
    assert_eq!(wake_segment_count(TravelPhase::Cruise), 4);
    assert_eq!(wake_segment_count(TravelPhase::FinalApproach), 2);
    assert_eq!(wake_segment_count(TravelPhase::Docked), 0);
}

#[test]
fn site_hazard_count_covers_the_full_arrival_plan() {
    let data = crate::data::GameData::load().unwrap();
    let site = data.sites.get("merchant_wreck").unwrap();
    assert_eq!(site_hazard_count(site), 4);
}

#[test]
fn transit_brief_names_retained_survey_count() {
    assert_eq!(travel_survey_label(0), "SURV 00");
    assert_eq!(travel_survey_label(3), "SURV 03");
}

#[test]
fn transit_brief_names_the_departure_scan_profile() {
    assert_eq!(
        travel_scan_label(crate::state::WorkspaceScanProfile::Standard),
        "SCAN STANDARD"
    );
    assert_eq!(
        travel_scan_label(crate::state::WorkspaceScanProfile::Array),
        "SCAN ARRAY"
    );
}

#[test]
fn travel_instructions_name_the_visible_next_control() {
    assert!(travel_instruction(TravelPhase::Cruise).contains("ARRIVE"));
    assert!(travel_instruction(TravelPhase::Docked).contains("CONTINUE"));
}
