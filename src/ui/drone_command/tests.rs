use super::*;

#[test]
fn drone_button_names_offline_and_selected_orders() {
    assert_eq!(
        directive_button_label(0, DroneDirective::PullSupport),
        "DRONE BAY OFFLINE"
    );
    assert_eq!(
        directive_button_label(1, DroneDirective::Survey),
        "DRONE // SURVEY"
    );
    assert_eq!(
        directive_button_label(2, DroneDirective::Standby),
        "DRONE // STANDBY"
    );
}
