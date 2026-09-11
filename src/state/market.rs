//! Persistent market-cycle quotes for returned salvage.

use super::{GameData, GameSession, ReturnedItem};
use crate::engine::market::{quote_for, MarketQuote};

impl GameSession {
    pub fn market_quote(&self, object_id: &str, data: &GameData) -> Option<MarketQuote> {
        let object = data.salvage_objects.get(object_id)?;
        Some(quote_for(object, self.market_cycle, &data.config.market))
    }

    pub fn returned_market_quote(
        &self,
        returned: &ReturnedItem,
        data: &GameData,
    ) -> Option<MarketQuote> {
        let object = data.salvage_objects.get(&returned.object_id)?;
        Some(quote_for(
            object,
            returned.market_cycle,
            &data.config.market,
        ))
    }

    pub fn market_cycle_label(&self) -> String {
        format!("CYCLE {:02}", self.market_cycle)
    }
}

#[cfg(test)]
mod tests;
