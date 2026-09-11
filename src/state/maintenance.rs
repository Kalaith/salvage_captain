//! Port maintenance quotes for hull damage and offline ship modules.

use super::GameSession;
use crate::data::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicePlan {
    Full,
    Hull,
    Systems,
}

impl ServicePlan {
    pub const ALL: [Self; 3] = [Self::Full, Self::Hull, Self::Systems];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Full => "FULL OVERHAUL",
            Self::Hull => "HULL PATCH",
            Self::Systems => "SYSTEMS SERVICE",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Full => "restore hull, offline modules, and operational wear",
            Self::Hull => "restore hull integrity; leave systems and wear untouched",
            Self::Systems => "restore offline modules and clear accumulated wear",
        }
    }

    pub const fn button_label(self) -> &'static str {
        match self {
            Self::Full => "OVERHAUL",
            Self::Hull => "PATCH HULL",
            Self::Systems => "SERVICE SYSTEMS",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairQuote {
    pub missing_hull: i32,
    pub offline_modules: usize,
    pub ship_wear: u8,
    pub hull_cost: i64,
    pub module_cost: i64,
    pub wear_cost: i64,
    pub total_cost: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceQuote {
    pub plan: ServicePlan,
    pub missing_hull: i32,
    pub offline_modules: usize,
    pub ship_wear: u8,
    pub total_cost: i64,
}

impl ServiceQuote {
    pub const fn is_due(self) -> bool {
        self.total_cost > 0
    }
}

impl RepairQuote {
    pub const fn is_due(self) -> bool {
        self.total_cost > 0
    }
}

impl GameSession {
    pub fn repair_quote(&self, data: &GameData) -> RepairQuote {
        let missing_hull = (self.max_hull_with_modules(data) - self.hull).max(0);
        let offline_modules = self.damaged_modules.len();
        let ship_wear = self.ship_wear();
        let hull_cost = i64::from(missing_hull) * i64::from(data.config.repair_price_per_hull);
        let module_cost = offline_modules as i64 * i64::from(data.config.module_repair_price);
        let wear_cost = self.maintenance_cost(data);
        RepairQuote {
            missing_hull,
            offline_modules,
            ship_wear,
            hull_cost,
            module_cost,
            wear_cost,
            total_cost: hull_cost + module_cost + wear_cost,
        }
    }

    pub fn repair(&mut self, data: &GameData) -> Result<String, String> {
        self.service(ServicePlan::Full, data)
    }

    pub fn service_quote(&self, plan: ServicePlan, data: &GameData) -> ServiceQuote {
        let quote = self.repair_quote(data);
        let (missing_hull, offline_modules, ship_wear, total_cost) = match plan {
            ServicePlan::Full => (
                quote.missing_hull,
                quote.offline_modules,
                quote.ship_wear,
                quote.total_cost,
            ),
            ServicePlan::Hull => (quote.missing_hull, 0, 0, quote.hull_cost),
            ServicePlan::Systems => (
                0,
                quote.offline_modules,
                quote.ship_wear,
                quote.module_cost + quote.wear_cost,
            ),
        };
        ServiceQuote {
            plan,
            missing_hull,
            offline_modules,
            ship_wear,
            total_cost,
        }
    }

    pub fn service(&mut self, plan: ServicePlan, data: &GameData) -> Result<String, String> {
        let repair_quote = self.repair_quote(data);
        let quote = self.service_quote(plan, data);
        if !quote.is_due() {
            return Err(format!("{} is not due", plan.label()));
        }
        if self.economy.credits < quote.total_cost {
            let detail = match plan {
                ServicePlan::Full => format!(
                    "hull {} + modules {} + wear {}",
                    quote.missing_hull,
                    self.damaged_modules.len(),
                    self.ship_wear()
                ),
                ServicePlan::Hull => format!("hull {}", quote.missing_hull),
                ServicePlan::Systems => format!(
                    "modules {} + wear {}",
                    quote.offline_modules, quote.ship_wear
                ),
            };
            return if plan == ServicePlan::Full && quote.ship_wear == 0 {
                Err(format!(
                    "repairs require {} credits (hull {} + modules {})",
                    quote.total_cost, quote.missing_hull, repair_quote.module_cost
                ))
            } else {
                Err(format!(
                    "{} requires {} credits ({detail})",
                    plan.label(),
                    quote.total_cost
                ))
            };
        }
        let restores_hull = matches!(plan, ServicePlan::Full | ServicePlan::Hull);
        let restores_systems = matches!(plan, ServicePlan::Full | ServicePlan::Systems);
        let restored: Vec<String> = if restores_systems {
            self.damaged_modules
                .iter()
                .filter_map(|id| {
                    data.modules
                        .get(id)
                        .map(|module| module.display_name.clone())
                })
                .collect()
        } else {
            Vec::new()
        };
        self.economy.credits -= quote.total_cost;
        if restores_systems {
            self.damaged_modules.clear();
            self.ship_wear = 0;
        }
        if restores_hull {
            self.hull = self.max_hull_with_modules(data);
        }
        self.career.record_repair(quote.total_cost, restored.len());
        if plan == ServicePlan::Systems {
            Ok(format!(
                "Serviced ship systems for {} credits",
                quote.total_cost
            ))
        } else if plan == ServicePlan::Hull {
            Ok(format!("Repaired hull for {} credits", quote.total_cost))
        } else if restored.is_empty() {
            if quote.missing_hull == 0 {
                Ok(format!(
                    "Serviced ship systems for {} credits",
                    quote.total_cost
                ))
            } else {
                Ok(format!("Repaired hull for {} credits", quote.total_cost))
            }
        } else {
            Ok(format!(
                "Repaired hull and systems for {} credits. Restored: {}.",
                quote.total_cost,
                restored.join(", ")
            ))
        }
    }
}

#[cfg(test)]
mod tests;
