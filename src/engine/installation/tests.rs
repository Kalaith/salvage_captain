use super::*;
use crate::data::{Footprint, GridPosition};

#[test]
fn failed_install_restores_the_temporary_item() {
    let mut layout = ShipLayout::new(5, 5);
    layout
        .place(
            "cargo:test",
            Footprint {
                width: 1,
                height: 1,
            },
            GridPosition::new(0, 0),
            0,
            false,
        )
        .unwrap();

    let error = install_module(
        &mut layout,
        "cargo:test",
        "large_module",
        Footprint {
            width: 2,
            height: 2,
        },
        GridPosition::new(4, 4),
        0,
    );

    assert!(error.is_err());
    assert_eq!(layout.placements.len(), 1);
    assert_eq!(layout.placements[0].id, "cargo:test");
}
