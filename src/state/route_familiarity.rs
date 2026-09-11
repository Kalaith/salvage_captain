//! Persistent route familiarity earned by working the same wreck repeatedly.

use super::GameSession;

pub const MAX_ROUTE_FAMILIARITY: u8 = 3;
pub const ROUTE_FAMILIARITY_DANGER_REDUCTION: i32 = 4;

impl GameSession {
    pub fn route_familiarity(&self, site_id: &str) -> u8 {
        self.site_progress.get(site_id).map_or(0, |progress| {
            progress.visits.min(u32::from(MAX_ROUTE_FAMILIARITY)) as u8
        })
    }

    pub fn route_familiarity_danger_reduction(&self, site_id: &str) -> i32 {
        i32::from(self.route_familiarity(site_id)) * ROUTE_FAMILIARITY_DANGER_REDUCTION
    }

    pub fn route_familiarity_label(&self, site_id: &str) -> &'static str {
        match self.route_familiarity(site_id) {
            0 => "NEW",
            1 => "KNOWN",
            2 => "WORKED",
            _ => "FAMILIAR",
        }
    }

    pub fn route_familiarity_readout(&self, site_id: &str) -> String {
        let reduction = self.route_familiarity_danger_reduction(site_id);
        if reduction == 0 {
            format!(
                "ROUTE {} // FIRST PASS",
                self.route_familiarity_label(site_id)
            )
        } else {
            format!(
                "ROUTE {} // DANGER -{}",
                self.route_familiarity_label(site_id),
                reduction
            )
        }
    }
}

#[cfg(test)]
mod tests;
