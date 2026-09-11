use super::*;

#[test]
fn log_button_label_keeps_small_counts_readable() {
    assert_eq!(log_button_label(0), "LOG 00");
    assert_eq!(log_button_label(6), "LOG 06");
    assert_eq!(log_button_label(107), "LOG 107");
}
