use super::*;

#[test]
fn directive_cycle_covers_recall_survey_and_pull_orders() {
    assert_eq!(DroneDirective::Standby.next(), DroneDirective::Survey);
    assert_eq!(DroneDirective::Survey.next(), DroneDirective::PullSupport);
    assert_eq!(DroneDirective::PullSupport.next(), DroneDirective::Standby);
}

#[test]
fn survey_trades_extra_field_support_for_extraction_speed() {
    assert_eq!(DroneDirective::Survey.effective_support(1), 2);
    assert_eq!(DroneDirective::PullSupport.effective_support(1), 1);
    assert_eq!(DroneDirective::Standby.effective_support(1), 0);
    assert_eq!(DroneDirective::Survey.extraction_reduction(), 0.0);
    assert_eq!(DroneDirective::PullSupport.extraction_reduction(), 0.12);
}

#[test]
fn directive_labels_explain_the_operator_tradeoff() {
    assert!(DroneDirective::Survey.description().contains("safer"));
    assert!(DroneDirective::PullSupport
        .description()
        .contains("shorten"));
    assert!(DroneDirective::Standby.description().contains("recalled"));
}
