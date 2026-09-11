//! Port-side coverage quotes and departure affordability checks.

use super::GameSession;
use crate::data::GameData;
use crate::engine::{insurance_quote_for, InsuranceQuote, VoyagePlan};

impl GameSession {
    pub fn insurance_quote(&self, site_id: &str, data: &GameData) -> Option<InsuranceQuote> {
        data.sites
            .get(site_id)
            .map(|site| insurance_quote_for(site, &data.config.insurance))
    }

    pub fn can_depart_insured(&self, site_id: &str, data: &GameData) -> bool {
        self.can_depart(site_id, data)
            && self
                .insurance_quote(site_id, data)
                .is_some_and(|quote| self.economy.credits >= quote.premium)
    }

    pub fn can_depart_insured_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> bool {
        if plan == VoyagePlan::Standard {
            return self.can_depart_insured(site_id, data);
        }
        self.can_depart_with_plan(site_id, data, plan)
            && self
                .insurance_quote(site_id, data)
                .is_some_and(|quote| self.economy.credits >= quote.premium)
    }
}

#[cfg(test)]
mod tests;
