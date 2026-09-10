//! Deterministic salvage selection from a site's data-defined candidate pool.

use crate::data::SiteData;

pub fn generate_salvage(site: &SiteData, seed: u64, removed_targets: &[String]) -> Vec<String> {
    let available: Vec<&String> = site
        .candidate_salvage
        .iter()
        .filter(|target| !removed_targets.iter().any(|removed| removed == *target))
        .collect();
    if available.is_empty() {
        return Vec::new();
    }
    let count = available.len().min(5);
    let start = (seed as usize) % available.len();
    let mut generated: Vec<String> = (0..count)
        .map(|index| available[(start + index) % available.len()].clone())
        .collect();
    if let Some(contract_target) = &site.contract_target {
        if removed_targets
            .iter()
            .any(|removed| removed == contract_target)
        {
            return generated;
        }
        if !generated.iter().any(|target| target == contract_target) {
            generated.pop();
            generated.push(contract_target.clone());
        }
    }
    generated
}

#[cfg(test)]
mod tests;
