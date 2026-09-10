//! Deterministic salvage selection from a site's data-defined candidate pool.

use crate::data::SiteData;

pub fn generate_salvage(site: &SiteData, seed: u64) -> Vec<String> {
    let count = site.candidate_salvage.len().min(5);
    let start = (seed as usize) % site.candidate_salvage.len();
    (0..count)
        .map(|index| site.candidate_salvage[(start + index) % site.candidate_salvage.len()].clone())
        .collect()
}
