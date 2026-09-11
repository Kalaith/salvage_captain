use super::*;

#[test]
fn visible_drone_count_tracks_support_without_overcrowding_the_scene() {
    assert_eq!(visible_drone_count(-1), 0);
    assert_eq!(visible_drone_count(1), 1);
    assert_eq!(visible_drone_count(3), 2);
}
