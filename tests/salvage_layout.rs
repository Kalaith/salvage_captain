//! Regression coverage for scene hit areas and fixed control boundaries.

use salvage_captain::{data::GameData, ui::scene_layout::salvage_layout};

#[test]
fn every_section_target_has_a_distinct_selectable_mount() {
    let data = GameData::load().unwrap();
    let layout = salvage_layout();
    for (_, site) in data.sites.iter() {
        for section in &site.sections {
            let mounts: Vec<_> = section
                .candidate_targets
                .iter()
                .map(|id| (id, layout.target_rect(id).unwrap()))
                .collect();
            for (index, (id, mount)) in mounts.iter().enumerate() {
                assert!(mount.w >= 44.0 && mount.h >= 44.0, "small hit area: {id}");
                for (other_id, other) in &mounts[index + 1..] {
                    assert!(
                        !mount.overlaps(other),
                        "overlapping targets in {}: {id} / {other_id}",
                        section.id
                    );
                }
            }
        }
    }
}

#[test]
fn mounts_remain_inside_the_world_and_clear_of_controls() {
    let layout = salvage_layout();
    for mount in [
        layout.power_relay,
        layout.navigation_core,
        layout.engine_assembly,
    ] {
        assert!(layout.wreck.contains(mount.point()));
        assert!(layout.wreck.contains(mount.point() + mount.size()));
        assert!(!mount.overlaps(&layout.command));
        assert!(!mount.overlaps(&layout.target_panel));
    }
    assert!(!layout.command.overlaps(&layout.target_panel));
    assert!(layout.command.bottom() <= 720.0 && layout.target_panel.bottom() <= 720.0);
}

#[test]
fn wide_wreck_and_ship_share_the_same_working_height() {
    let layout = salvage_layout();
    assert!((0.70..=0.80).contains(&(layout.wreck.w / layout.viewport.w)));
    assert!(layout.ship.center().y > layout.wreck.y);
    assert!(layout.ship.center().y < layout.wreck.bottom());
    assert!(layout.ship.right() >= layout.wreck.x);
}
