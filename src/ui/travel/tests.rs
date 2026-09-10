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
fn travel_instructions_name_the_visible_next_control() {
    assert!(travel_instruction(TravelPhase::Cruise).contains("ARRIVE"));
    assert!(travel_instruction(TravelPhase::Docked).contains("CONTINUE"));
}
