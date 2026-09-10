use super::*;
use crate::data::{Footprint, GameData, GridPosition};

#[test]
fn damaged_modules_stop_contributing_stats() {
    let data = GameData::load().unwrap();
    let mut layout = ShipLayout::new(data.config.grid_width, data.config.grid_height);
    layout
        .place(
            "reactor_module",
            data.modules.get("reactor_module").unwrap().footprint,
            GridPosition::new(0, 0),
            0,
            true,
        )
        .unwrap();

    let active = stats_from_layout(&layout, &data, &[]);
    let damaged = stats_from_layout(&layout, &data, &["reactor_module".to_owned()]);

    assert_eq!(active.power, 4);
    assert_eq!(active.external_capacity, 2);
    assert_eq!(damaged, ModuleStats::default());
}

#[test]
fn temporary_cargo_never_contributes_module_stats() {
    let data = GameData::load().unwrap();
    let mut layout = ShipLayout::new(data.config.grid_width, data.config.grid_height);
    layout
        .place(
            "cargo:industrial_battery",
            Footprint {
                width: 1,
                height: 1,
            },
            GridPosition::new(0, 0),
            0,
            false,
        )
        .unwrap();

    assert_eq!(
        stats_from_layout(&layout, &data, &[]),
        ModuleStats::default()
    );
}
