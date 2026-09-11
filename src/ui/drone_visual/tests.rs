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

#[test]
fn drone_operation_label_calls_out_active_pull_assist() {
    assert_eq!(
        drone_operation_label(false, crate::state::DroneDirective::PullSupport),
        "DRONE MESH  //  PULL READY"
    );
    assert_eq!(
        drone_operation_label(true, crate::state::DroneDirective::PullSupport),
        "DRONE MESH  //  PULL ASSIST"
    );
    assert_eq!(
        drone_operation_label(false, crate::state::DroneDirective::Survey),
        "DRONE MESH  //  SURVEY NET"
    );
}
