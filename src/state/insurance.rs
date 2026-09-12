//! Port-side coverage quotes and departure affordability checks.

use super::GameSession;
use crate::data::GameData;
use crate::engine::{
    danger_after_intel, insurance_quote_for, insurance_quote_for_danger, InsuranceQuote, VoyagePlan,
};

impl GameSession {
    pub fn insurance_quote(&self, site_id: &str, data: &GameData) -> Option<InsuranceQuote> {
        if self.ship_wear() > 0
            || self.reconnaissance_level(site_id) > 0
            || self.crew_danger_delta() != 0
            || self.route_familiarity(site_id) > 0
        {
            return self.insurance_quote_with_plan(site_id, data, VoyagePlan::Standard);
        }
        data.sites
            .get(site_id)
            .map(|site| insurance_quote_for(site, &data.config.insurance))
    }

    pub fn insurance_quote_with_plan(
        &self,
        site_id: &str,
        data: &GameData,
        plan: VoyagePlan,
    ) -> Option<InsuranceQuote> {
        if plan == VoyagePlan::Standard
            && self.reconnaissance_level(site_id) == 0
            && self.crew_danger_delta() == 0
            && self.ship_wear() == 0
            && self.route_familiarity(site_id) == 0
        {
            return self.insurance_quote(site_id, data);
        }
        data.sites.get(site_id).map(|site| {
            let route_danger = danger_after_intel(
                site.danger,
                self.reconnaissance_level(site_id),
                &data.config.reconnaissance,
            )
            .saturating_sub(self.route_familiarity_danger_reduction(site_id));
            insurance_quote_for_danger(
                self.maintenance_adjusted_danger(
                    self.crew_adjusted_danger(
                        plan.adjust_danger(route_danger, &data.config.voyage_plan),
                    ),
                    data,
                ),
                &data.config.insurance,
            )
        })
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
        if plan == VoyagePlan::Standard
            && self.reconnaissance_level(site_id) == 0
            && self.crew_danger_delta() == 0
            && self.ship_wear() == 0
            && self.route_familiarity(site_id) == 0
        {
            return self.can_depart_insured(site_id, data);
        }
        self.can_depart_with_plan(site_id, data, plan)
            && self
                .insurance_quote_with_plan(site_id, data, plan)
                .is_some_and(|quote| self.economy.credits >= quote.premium)
    }
}
