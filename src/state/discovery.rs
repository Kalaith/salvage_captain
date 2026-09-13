//! Port discovery, automatic replenishment, and wreck lifecycle queries.

use super::{GameSession, SiteProgress};
use crate::data::discovery::LeadKind;
use crate::data::{GameData, SiteData};
use macroquad_toolkit::rng::SeededRng;

impl GameSession {
    pub fn wreck_depleted(&self, site_id: &str, data: &GameData) -> bool {
        data.sites.contains(site_id)
            && self.site_recovery_status(site_id, data).remaining_targets == 0
    }

    pub fn listed_wrecks<'a>(&self, data: &'a GameData, archived: bool) -> Vec<&'a SiteData> {
        let mut sites: Vec<_> = data
            .ordered_sites()
            .into_iter()
            .filter(|site| self.wreck_depleted(&site.id, data) == archived)
            .collect();
        sites.sort_by(|a, b| {
            let a_serial = self
                .wrecks
                .iter()
                .find(|w| w.site.id == a.id)
                .map_or(0, |w| w.serial);
            let b_serial = self
                .wrecks
                .iter()
                .find(|w| w.site.id == b.id)
                .map_or(0, |w| w.serial);
            b_serial.cmp(&a_serial).then_with(|| a.id.cmp(&b.id))
        });
        sites
    }

    pub fn discovery_block_reason(&self, kind: LeadKind, data: &GameData) -> Option<String> {
        let copy = &data.discovery.copy;
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Some(copy.busy.clone());
        }
        if self.listed_wrecks(data, false).len() >= data.discovery.active_limit {
            return Some(copy.full.clone());
        }
        let specialist = kind == LeadKind::Specialist;
        if !data
            .discovery
            .pools
            .iter()
            .any(|pool| pool.specialist == specialist && pool.minimum_reputation <= self.reputation)
        {
            return Some(copy.locked.clone());
        }
        if specialist && self.economy.credits < data.discovery.specialist_cost {
            return Some(
                copy.unaffordable
                    .replace("{cost}", &data.discovery.specialist_cost.to_string()),
            );
        }
        None
    }

    /// Generate and validate first. Commit the debit, sequence and registry together.
    pub fn discover_wreck(
        &mut self,
        kind: LeadKind,
        data: &mut GameData,
    ) -> Result<String, String> {
        if let Some(reason) = self.discovery_block_reason(kind, data) {
            return Err(reason);
        }
        let mut rng = SeededRng::new(self.discovery_seed);
        let pools: Vec<_> = data
            .discovery
            .pools
            .iter()
            .filter(|pool| {
                pool.specialist == (kind == LeadKind::Specialist)
                    && pool.minimum_reputation <= self.reputation
            })
            .collect();
        let pool = rng
            .choose(&pools)
            .ok_or_else(|| data.discovery.copy.locked.clone())?;
        let serial = self
            .discovery_serial
            .checked_add(1)
            .ok_or_else(|| "wreck discovery sequence exhausted".to_owned())?;
        let wreck = crate::engine::discovery::generate_wreck(pool, serial, rng.next_u64(), data)?;
        let id = wreck.site.id.clone();
        let mut wrecks = self.wrecks.clone();
        wrecks.push(wreck.clone());
        let resolved = data.with_wreck_instances(&wrecks)?;
        if kind == LeadKind::Specialist {
            self.economy.credits -= data.discovery.specialist_cost;
        }
        self.discovery_serial = serial;
        self.discovery_seed = rng.next_u64();
        self.site_progress
            .insert(id.clone(), SiteProgress::fresh(wreck.site.condition));
        self.wrecks = wrecks;
        self.selected_site = Some(id.clone());
        *data = resolved;
        Ok(id)
    }

    /// Refill the local lead after a voyage without replacing unfinished jobs.
    pub fn replenish_wrecks(&mut self, data: &mut GameData) -> Result<Option<String>, String> {
        if self.expedition.is_some() || !self.returned.is_empty() {
            return Ok(None);
        }
        let local_available = self.listed_wrecks(data, false).iter().any(|site| {
            site.visual_theme == "merchant"
                && site.contract_target.as_ref().is_some_and(|target| {
                    !self
                        .site_progress
                        .get(&site.id)
                        .is_some_and(|p| p.removed_targets.contains(target))
                })
        });
        if local_available || self.discovery_block_reason(LeadKind::Local, data).is_some() {
            return Ok(None);
        }
        self.discover_wreck(LeadKind::Local, data).map(Some)
    }

    pub fn wreck_equipment(&self, site: &SiteData, data: &GameData) -> String {
        let mut capabilities: Vec<&str> = site
            .sections
            .iter()
            .filter_map(|s| s.required_capability.as_deref())
            .chain(
                site.candidate_salvage
                    .iter()
                    .filter_map(|id| data.salvage_objects.get(id))
                    .filter_map(|target| target.required_capability.as_deref()),
            )
            .collect();
        capabilities.sort_unstable();
        capabilities.dedup();
        let names: Vec<_> = capabilities
            .iter()
            .map(|capability| {
                data.modules
                    .iter()
                    .find(|(_, module)| module.capability.as_deref() == Some(*capability))
                    .map_or_else(
                        || capability.replace('_', " "),
                        |(_, module)| module.display_name.clone(),
                    )
            })
            .collect();
        if names.is_empty() {
            data.discovery.copy.starter_ready.clone()
        } else {
            names.join(", ")
        }
    }
}
