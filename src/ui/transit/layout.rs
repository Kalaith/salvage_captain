//! Shared flight composition and deterministic motion independent of rendering.

use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct TransitLayout {
    pub ship: Rect,
    pub destination: Rect,
    pub summary: Rect,
    pub commands: Rect,
    pub primary: Rect,
    pub details: Rect,
}

pub fn transit_layout(progress: f32, reduced_motion: bool) -> TransitLayout {
    let approach = if reduced_motion {
        1.0
    } else {
        approach_distance(progress)
    };
    TransitLayout {
        ship: Rect::new(216.0, 298.0, 470.0, 246.0),
        destination: Rect::new(
            1230.0 - approach * 486.0,
            254.0 - approach * 48.0,
            340.0 + approach * 340.0,
            218.0 + approach * 124.0,
        ),
        summary: Rect::new(24.0, 574.0, 840.0, 126.0),
        commands: Rect::new(880.0, 574.0, 372.0, 126.0),
        primary: Rect::new(898.0, 590.0, 336.0, 48.0),
        details: Rect::new(898.0, 648.0, 336.0, 40.0),
    }
}

pub fn approach_distance(progress: f32) -> f32 {
    let bounded = progress.clamp(0.0, 1.0);
    bounded * (2.0 - bounded)
}

pub fn scenery_offset(progress: f32, layer: usize, reduced_motion: bool) -> f32 {
    if reduced_motion {
        0.0
    } else {
        approach_distance(progress) * (120.0 + layer as f32 * 340.0)
    }
}
