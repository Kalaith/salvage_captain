use super::*;

#[test]
fn visible_drone_count_tracks_support_without_overcrowding_the_scene() {
    assert_eq!(visible_drone_count(-1), 0);
    assert_eq!(visible_drone_count(1), 1);
    assert_eq!(visible_drone_count(3), 2);
}

#[test]
fn drone_signal_packets_stay_on_the_tether() {
    assert_eq!(drone_signal_progress(0.0, 0), 0.0);
    assert_eq!(drone_signal_progress(1.0, 0), 0.65);
    assert!((0.0..1.0).contains(&drone_signal_progress(9.0, 1)));
}
