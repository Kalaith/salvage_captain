//! Validated conversion of temporary cargo into permanent ship modules.

use super::packing::ShipLayout;
use crate::data::{Footprint, GridPosition};

pub fn install_module(
    layout: &mut ShipLayout,
    item_id: &str,
    module_id: &str,
    footprint: Footprint,
    position: GridPosition,
    rotation: u8,
) -> Result<(), String> {
    let cargo = layout
        .remove(item_id)
        .ok_or_else(|| format!("temporary item '{item_id}' is not in the ship layout"))?;
    if let Err(error) = layout.can_place(module_id, footprint, position, rotation) {
        restore_removed_item(layout, cargo);
        return Err(error.to_string());
    }
    if let Err(error) = layout.place(module_id, footprint, position, rotation, true) {
        restore_removed_item(layout, cargo);
        return Err(error.to_string());
    }
    Ok(())
}

fn restore_removed_item(layout: &mut ShipLayout, item: super::packing::PlacedItem) {
    let _ = layout.place(
        item.id,
        item.footprint,
        item.position,
        item.rotation,
        item.permanent,
    );
}
