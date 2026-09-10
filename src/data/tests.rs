use super::*;

#[test]
fn embedded_content_loads_and_cross_references_validate() {
    let data = GameData::load().unwrap();
    assert_eq!(data.config.grid_width, 5);
    assert_eq!(data.sites.len(), 3);
    assert!(data.salvage_objects.contains("damaged_reactor"));
    assert!(data.modules.contains("nav_module"));
}

#[test]
fn rotations_swap_rectangular_dimensions() {
    let footprint = Footprint {
        width: 1,
        height: 2,
    };
    assert_eq!(footprint.rotated(0), footprint);
    assert_eq!(
        footprint.rotated(1),
        Footprint {
            width: 2,
            height: 1
        }
    );
}
