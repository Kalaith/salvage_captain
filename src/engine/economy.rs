//! Stateless sell, install, and breakdown accounting.

use crate::data::SalvageObjectData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Sell,
    Install,
    BreakDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceDelta {
    pub credits: i64,
    pub alloy: i32,
    pub electronics: i32,
}

pub fn resolve_disposition(
    object: &SalvageObjectData,
    disposition: Disposition,
) -> Result<ResourceDelta, String> {
    match disposition {
        Disposition::Sell => Ok(ResourceDelta {
            credits: object.sale_value,
            ..Default::default()
        }),
        Disposition::BreakDown => Ok(ResourceDelta {
            alloy: object.alloy_yield,
            electronics: object.electronics_yield,
            ..Default::default()
        }),
        Disposition::Install => object
            .install_module_id
            .as_ref()
            .map(|_| ResourceDelta::default())
            .ok_or_else(|| format!("{} cannot be installed", object.display_name)),
    }
}
