//! Shared logical grid geometry for drawing and pointer hit-testing.

use crate::data::{Footprint, GridPosition};
use macroquad::prelude::Rect;

pub fn grid_rect(rect: Rect, width: i32, height: i32) -> Rect {
    let cell = (rect.w / width as f32).min(rect.h / height as f32);
    Rect::new(
        rect.x + (rect.w - cell * width as f32) * 0.5,
        rect.y + (rect.h - cell * height as f32) * 0.5,
        cell * width as f32,
        cell * height as f32,
    )
}

pub fn cell_rect(rect: Rect, x: i32, y: i32, width: i32, height: i32) -> Rect {
    Rect::new(
        rect.x + x as f32 * rect.w / width as f32,
        rect.y + y as f32 * rect.h / height as f32,
        rect.w / width as f32,
        rect.h / height as f32,
    )
}

pub fn item_rect(
    rect: Rect,
    position: GridPosition,
    footprint: Footprint,
    width: i32,
    height: i32,
) -> Rect {
    Rect::new(
        rect.x + position.x as f32 * rect.w / width as f32,
        rect.y + position.y as f32 * rect.h / height as f32,
        footprint.width as f32 * rect.w / width as f32,
        footprint.height as f32 * rect.h / height as f32,
    )
}

pub fn cell_at(
    rect: Rect,
    point: macroquad::prelude::Vec2,
    width: i32,
    height: i32,
) -> Option<GridPosition> {
    if !rect.contains(point) {
        return None;
    }
    Some(GridPosition::new(
        ((point.x - rect.x) / (rect.w / width as f32)).floor() as i32,
        ((point.y - rect.y) / (rect.h / height as f32)).floor() as i32,
    ))
}
