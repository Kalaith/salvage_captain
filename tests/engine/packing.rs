//! Ship-grid bounds, overlap, and rotation rules.

use super::*;
use crate::data::{Footprint, GridPosition};
use crate::engine::packing::*;

fn square() -> Footprint {
    Footprint {
        width: 2,
        height: 2,
    }
}

#[test]
fn bounds_and_overlap_are_rejected() {
    let mut layout = ShipLayout::new(5, 5);
    layout
        .place("engine", square(), GridPosition::new(0, 0), 0, true)
        .unwrap();
    assert_eq!(
        layout.can_place("cargo", square(), GridPosition::new(4, 4), 0),
        Err(PackingError::OutsideGrid)
    );
    assert_eq!(
        layout.can_place(
            "cargo",
            Footprint {
                width: 1,
                height: 1
            },
            GridPosition::new(1, 1),
            0
        ),
        Err(PackingError::Overlap("engine".to_owned()))
    );
}

#[test]
fn rotation_changes_footprint_and_first_fit_uses_it() {
    let layout = ShipLayout::new(3, 2);
    let (position, rotation) = layout
        .first_fit(
            "panel",
            Footprint {
                width: 2,
                height: 1,
            },
            true,
        )
        .unwrap();
    assert_eq!(position, GridPosition::new(0, 0));
    assert_eq!(rotation, 0);
    assert_eq!(
        Footprint {
            width: 2,
            height: 1
        }
        .rotated(1),
        Footprint {
            width: 1,
            height: 2
        }
    );
}
