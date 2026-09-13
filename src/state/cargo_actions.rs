//! Cargo placement and returned-salvage disposition operations.

use super::*;

impl GameSession {
    pub fn auto_place(&mut self, object_id: &str, data: &GameData) -> Result<String, String> {
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let layout_id = format!("cargo:{object_id}");
        let Some((position, rotation)) =
            self.ship_layout
                .first_fit(&layout_id, object.footprint, object.rotatable)
        else {
            return Err(format!(
                "{} has no open fit in the ship grid",
                object.display_name
            ));
        };
        self.place_cargo(object_id, position, rotation, data)
    }

    pub fn place_cargo(
        &mut self,
        object_id: &str,
        position: GridPosition,
        rotation: u8,
        data: &GameData,
    ) -> Result<String, String> {
        if self.workspace_transfer().is_some() {
            return Err(data.salvage_ui.transfer_busy.clone());
        }
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        if TransferMode::from_target(object).uses_external_rig() {
            let used = self.external_cargo_count(data, Some(object_id));
            let capacity = self.external_capacity(data);
            if used >= capacity {
                return Err(format!(
                    "No external clamp is free ({used}/{capacity}). Leave this load behind or upgrade the ship."
                ));
            }
        } else if self.internal_cargo_count(data, Some(object_id)) >= self.internal_cargo_capacity()
        {
            return Err(format!(
                "No internal cargo berth is free ({}/{}). Leave this load behind or expand the cargo bay.",
                self.internal_cargo_count(data, Some(object_id)),
                self.internal_cargo_capacity()
            ));
        }
        let previous = self
            .expedition
            .as_ref()
            .and_then(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .find(|item| item.object_id == object_id)
            })
            .and_then(|item| item.position.map(|old| (old, item.rotation)));
        if previous.is_none() {
            let is_pending = self.expedition.as_ref().is_some_and(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .any(|item| item.object_id == object_id && item.status == CargoStatus::Pending)
            });
            if !is_pending {
                return Err("that salvage is no longer available".to_owned());
            }
        }
        let layout_id = cargo_layout_id(object_id);
        self.ship_layout.remove(&layout_id);
        if let Err(error) =
            self.ship_layout
                .can_place(&layout_id, object.footprint, position, rotation)
        {
            if let Some((old_position, old_rotation)) = previous {
                let _ = self.ship_layout.place(
                    layout_id.clone(),
                    object.footprint,
                    old_position,
                    old_rotation,
                    false,
                );
            }
            return Err(error.to_string());
        }
        self.ship_layout
            .place(layout_id, object.footprint, position, rotation, false)
            .map_err(|error| error.to_string())?;
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        let cargo = expedition
            .cargo
            .iter_mut()
            .find(|item| item.object_id == object_id)
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        cargo.status = CargoStatus::Packed;
        cargo.position = Some(position);
        cargo.rotation = rotation % 2;
        Ok(format!("Packed {}", object.display_name))
    }

    pub fn set_cargo_status(
        &mut self,
        object_id: &str,
        status: CargoStatus,
    ) -> Result<String, String> {
        if self.workspace_transfer().is_some() {
            return Err("Finish or cancel the active pull first.".to_owned());
        }
        let expedition = self
            .expedition
            .as_mut()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        let cargo = expedition
            .cargo
            .iter_mut()
            .find(|item| item.object_id == object_id)
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        self.ship_layout.remove(&cargo_layout_id(object_id));
        cargo.status = status;
        cargo.position = None;
        Ok(format!("{}: {:?}", object_id, status))
    }

    pub fn rotate_cargo(&mut self, object_id: &str, data: &GameData) -> Result<String, String> {
        let (rotation, position) = self
            .expedition
            .as_ref()
            .and_then(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .find(|item| item.object_id == object_id)
            })
            .map(|item| (item.rotation, item.position))
            .ok_or_else(|| "that salvage is not in the current expedition".to_owned())?;
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let next_rotation = rotation.wrapping_add(1) % 2;
        if !object.rotatable {
            return Err(format!("{} cannot rotate", object.display_name));
        }
        let Some(position) = position else {
            let expedition = self.expedition.as_mut().expect("checked above");
            let cargo = expedition
                .cargo
                .iter_mut()
                .find(|item| item.object_id == object_id)
                .expect("checked above");
            cargo.rotation = next_rotation;
            return Ok(format!("Rotated {}", object.display_name));
        };
        self.place_cargo(object_id, position, next_rotation, data)?;
        Ok(format!("Rotated {}", object.display_name))
    }

    pub fn leave_all_pending(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self
            .expedition
            .as_ref()
            .ok_or_else(|| "there is no active expedition".to_owned())?
            .cargo
            .iter()
            .filter(|item| item.status == CargoStatus::Pending)
            .map(|item| item.object_id.clone())
            .collect();
        for id in ids {
            self.set_cargo_status(&id, CargoStatus::LeftBehind)?;
        }
        Ok(())
    }

    pub fn pending_count(&self) -> usize {
        self.expedition
            .as_ref()
            .map(|expedition| {
                expedition
                    .cargo
                    .iter()
                    .filter(|item| item.status == CargoStatus::Pending)
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn dispose(
        &mut self,
        object_id: &str,
        disposition: Disposition,
        data: &GameData,
    ) -> Result<String, String> {
        let index = self
            .returned
            .iter()
            .position(|item| item.object_id == object_id)
            .ok_or_else(|| "that item has already been resolved".to_owned())?;
        let returned = self.returned[index].clone();
        let object = data
            .salvage_objects
            .get(object_id)
            .ok_or_else(|| format!("unknown salvage object '{object_id}'"))?;
        let mut delta = resolve_disposition(object, disposition)?;
        if disposition == Disposition::Sell {
            delta.credits = crate::engine::market::quote_for(
                object,
                returned.market_cycle,
                &data.config.market,
            )
            .sale_value;
        }
        match disposition {
            Disposition::Sell | Disposition::BreakDown => {
                self.ship_layout.remove(&cargo_layout_id(object_id));
                self.economy.credits += delta.credits;
                self.economy.alloy += delta.alloy;
                self.economy.electronics += delta.electronics;
                if disposition == Disposition::Sell {
                    self.career.record_sale(delta.credits);
                }
            }
            Disposition::Install => {
                let module_id = object
                    .install_module_id
                    .as_ref()
                    .ok_or_else(|| "that object has no installation effect".to_owned())?;
                let module = data
                    .modules
                    .get(module_id)
                    .ok_or_else(|| format!("missing install module '{module_id}'"))?;
                if self
                    .ship_layout
                    .placements
                    .iter()
                    .any(|item| item.id == *module_id)
                {
                    return Err(format!("{} is already installed", module.display_name));
                }
                if self.economy.credits < module.install_cost {
                    return Err(format!(
                        "installation requires {} credits",
                        module.install_cost
                    ));
                }
                installation::install_module(
                    &mut self.ship_layout,
                    &cargo_layout_id(object_id),
                    module_id,
                    module.footprint,
                    returned.position,
                    returned.rotation,
                )?;
                self.economy.credits -= module.install_cost;
                self.career.record_module_change(module.install_cost);
                if !self.unlocked_modules.contains(module_id) {
                    self.unlocked_modules.push(module_id.clone());
                }
            }
        }
        self.returned.remove(index);
        let mut label = match disposition {
            Disposition::Sell => {
                format!("Sold {} for {} credits", object.display_name, delta.credits)
            }
            Disposition::Install => format!("Installed {}", object.display_name),
            Disposition::BreakDown => format!("Broke down {} for materials", object.display_name),
        };
        self.decisions.push(DecisionRecord {
            object_id: object_id.to_owned(),
            disposition: format!("{disposition:?}"),
        });
        let newly_unlocked = self.refresh_module_unlocks(data);
        if !newly_unlocked.is_empty() {
            let names = newly_unlocked
                .iter()
                .filter_map(|id| data.modules.get(id))
                .map(|module| module.display_name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            label.push_str(&format!(". Blueprint unlocked: {names}."));
        }
        Ok(label)
    }
}
