//! Hardpoint selection must remain unambiguous as equipment is added.

use macroquad::prelude::{vec2, Rect};
use salvage_captain::data::GameData;
use salvage_captain::ui::ship_visual::{module_mount_rect, tractor_emitter_rect};

#[test]
fn every_equipment_mount_has_a_separate_touch_target_inside_the_port_ship() {
    let data = GameData::load().unwrap();
    let ship = Rect::new(42., 246., 720., 353.);
    let mounts: Vec<_> = data
        .modules
        .iter()
        .map(|(id, module)| (id, module_mount_rect(ship, module)))
        .collect();
    for (index, (id, bounds)) in mounts.iter().enumerate() {
        assert!(bounds.w >= 64. && bounds.h >= 64., "{id}");
        assert!(
            ship.contains(bounds.point()) && ship.contains(bounds.point() + bounds.size()),
            "{id}"
        );
        assert!(
            !bounds.overlaps(&tractor_emitter_rect(ship)),
            "{id} overlaps tractor control"
        );
        for (other_id, other) in &mounts[index + 1..] {
            let overlap_width = bounds.right().min(other.right()) - bounds.x.max(other.x);
            let overlap_height = bounds.bottom().min(other.bottom()) - bounds.y.max(other.y);
            assert!(
                overlap_width <= 0.01 || overlap_height <= 0.01,
                "{id} overlaps {other_id}"
            );
        }
    }
}

#[test]
fn mount_centers_follow_the_rendered_hull_height_when_the_view_changes() {
    let data = GameData::load().unwrap();
    let tank = data.modules.get("fuel_tank").unwrap();
    for ship in [
        Rect::new(42., 246., 720., 353.),
        Rect::new(216., 298., 470., 246.),
    ] {
        let center = module_mount_rect(ship, tank).center();
        // The fuel cylinder sits on the roof of the 72%-height hull.
        let expected = vec2(ship.x + ship.w * 0.38, ship.y + ship.h * 0.72 * 83. / 360.);
        assert!(center.distance(expected) < 0.01);
    }
}
