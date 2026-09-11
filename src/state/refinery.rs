//! Safe-checkpoint material refining for the station shipyard.

use super::{GameData, GameSession};
use crate::engine::refinery::{quote_for, RefineryQuote, RefineryResource};

impl GameSession {
    pub fn refinery_quote(&self, resource: RefineryResource, data: &GameData) -> RefineryQuote {
        quote_for(resource, self.economy, &data.config.refinery)
    }

    pub fn refine_resource(
        &mut self,
        resource: RefineryResource,
        data: &GameData,
    ) -> Result<String, String> {
        let quote = self.refinery_quote(resource, data);
        if !quote.can_refine() {
            return Err(format!(
                "Need {} {} for one refinery batch",
                quote.batch_size,
                resource.label()
            ));
        }
        match resource {
            RefineryResource::Alloy => self.economy.alloy -= quote.batch_size,
            RefineryResource::Electronics => self.economy.electronics -= quote.batch_size,
        }
        self.economy.credits += quote.payout;
        let remaining = self.refinery_quote(resource, data).available;
        Ok(format!(
            "Refined {} {} for {} credits. {} {} remain.",
            quote.batch_size,
            resource.label(),
            quote.payout,
            remaining,
            resource.label()
        ))
    }
}

#[cfg(test)]
mod tests;
