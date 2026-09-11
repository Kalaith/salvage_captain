use super::*;

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
