//! Shared ship-grid geometry and placement validation.

use crate::data::{Footprint, GridPosition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlacedItem {
    pub id: String,
    pub footprint: Footprint,
    pub position: GridPosition,
    pub rotation: u8,
    pub permanent: bool,
}

impl PlacedItem {
    pub fn new(
        id: impl Into<String>,
        footprint: Footprint,
        position: GridPosition,
        rotation: u8,
        permanent: bool,
    ) -> Self {
        Self {
            id: id.into(),
            footprint,
            position,
            rotation: rotation % 2,
            permanent,
        }
    }

    pub fn cells(&self) -> Vec<GridPosition> {
        let shape = self.footprint.rotated(self.rotation);
        (self.position.y..self.position.y + shape.height)
            .flat_map(|y| {
                (self.position.x..self.position.x + shape.width)
                    .map(move |x| GridPosition::new(x, y))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShipLayout {
    pub width: i32,
    pub height: i32,
    pub placements: Vec<PlacedItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackingError {
    OutsideGrid,
    Overlap(String),
    DuplicateId(String),
}

impl std::fmt::Display for PackingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutsideGrid => write!(formatter, "that footprint is outside the ship grid"),
            Self::Overlap(id) => write!(formatter, "that space is occupied by {id}"),
            Self::DuplicateId(id) => write!(formatter, "an item named {id} is already placed"),
        }
    }
}

impl ShipLayout {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            placements: Vec::new(),
        }
    }

    pub fn can_place(
        &self,
        id: &str,
        footprint: Footprint,
        position: GridPosition,
        rotation: u8,
    ) -> Result<(), PackingError> {
        if self.placements.iter().any(|item| item.id == id) {
            return Err(PackingError::DuplicateId(id.to_owned()));
        }
        let shape = footprint.rotated(rotation);
        if position.x < 0
            || position.y < 0
            || position.x + shape.width > self.width
            || position.y + shape.height > self.height
        {
            return Err(PackingError::OutsideGrid);
        }
        for cell in (position.y..position.y + shape.height).flat_map(|y| {
            (position.x..position.x + shape.width).map(move |x| GridPosition::new(x, y))
        }) {
            if let Some(other) = self
                .placements
                .iter()
                .find(|item| item.cells().contains(&cell))
            {
                return Err(PackingError::Overlap(other.id.clone()));
            }
        }
        Ok(())
    }

    pub fn place(
        &mut self,
        id: impl Into<String>,
        footprint: Footprint,
        position: GridPosition,
        rotation: u8,
        permanent: bool,
    ) -> Result<(), PackingError> {
        let id = id.into();
        self.can_place(&id, footprint, position, rotation)?;
        self.placements.push(PlacedItem::new(
            id, footprint, position, rotation, permanent,
        ));
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Option<PlacedItem> {
        let index = self.placements.iter().position(|item| item.id == id)?;
        Some(self.placements.remove(index))
    }

    pub fn occupied_cells(&self) -> usize {
        self.placements.iter().map(|item| item.cells().len()).sum()
    }

    pub fn first_fit(
        &self,
        id: &str,
        footprint: Footprint,
        rotatable: bool,
    ) -> Option<(GridPosition, u8)> {
        let rotations = if rotatable { 0..2 } else { 0..1 };
        rotations
            .flat_map(|rotation| {
                (0..self.height).flat_map(move |y| {
                    (0..self.width).map(move |x| (GridPosition::new(x, y), rotation))
                })
            })
            .find(|(position, rotation)| {
                self.can_place(id, footprint, *position, *rotation).is_ok()
            })
    }
}

#[cfg(test)]
mod tests;
