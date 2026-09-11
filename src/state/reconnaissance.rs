//! Persistent site intelligence purchases and departure affordability.

use super::GameSession;
use crate::data::GameData;
use crate::engine::{reconnaissance_quote_for, ReconnaissanceQuote};

impl GameSession {
    pub fn reconnaissance_level(&self, site_id: &str) -> u8 {
        self.site_progress
            .get(site_id)
            .map_or(0, |progress| progress.reconnaissance_level)
    }

    pub fn reconnaissance_quote(
        &self,
        site_id: &str,
        data: &GameData,
    ) -> Option<ReconnaissanceQuote> {
        data.sites.get(site_id)?;
        reconnaissance_quote_for(
            self.reconnaissance_level(site_id),
            &data.config.reconnaissance,
        )
    }

    pub fn can_buy_reconnaissance(&self, site_id: &str, data: &GameData) -> bool {
        self.expedition.is_none()
            && self.returned.is_empty()
            && self
                .reconnaissance_quote(site_id, data)
                .is_some_and(|quote| self.economy.credits >= quote.cost)
    }

    pub fn buy_reconnaissance(&mut self, site_id: &str, data: &GameData) -> Result<String, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Err("route intelligence is available at the port before departure".to_owned());
        }
        let site = data
            .sites
            .get(site_id)
            .ok_or_else(|| format!("unknown salvage site '{site_id}'"))?;
        let quote = self
            .reconnaissance_quote(site_id, data)
            .ok_or_else(|| format!("{} route is fully briefed", site.display_name))?;
        if self.economy.credits < quote.cost {
            return Err(format!(
                "route intelligence requires {} credits",
                quote.cost
            ));
        }
        let progress = self
            .site_progress
            .get_mut(site_id)
            .ok_or_else(|| format!("missing progress for '{}'", site.display_name))?;
        progress.reconnaissance_level = quote.next_level;
        self.economy.credits -= quote.cost;
        Ok(format!(
            "Route brief bought for {} credits. {} intel level {}/{}; departure danger -{}.",
            quote.cost,
            site.display_name,
            quote.next_level,
            quote.max_level,
            quote.danger_reduction
        ))
    }
}

#[cfg(test)]
mod tests;
