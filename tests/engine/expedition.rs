//! Deterministic salvage manifest generation rules.

use crate::data::GameData;
use crate::engine::expedition::generate_salvage;
use std::collections::HashSet;

#[test]
fn contract_objective_is_always_in_the_generated_manifest() {
    let data = GameData::load().unwrap();
    for site in data.ordered_sites() {
        let contract_target = site.contract_target.as_ref().unwrap();
        for seed in 0..12 {
            assert!(generate_salvage(site, seed, &[]).contains(contract_target));
        }
    }
}

#[test]
fn removed_targets_stay_out_of_future_manifests() {
    let data = GameData::load().unwrap();
    let site = data.sites.get("merchant_wreck").unwrap();

    let manifest = generate_salvage(site, 7, &["industrial_battery".to_owned()]);

    assert!(!manifest.contains(&"industrial_battery".to_owned()));
}

#[test]
fn generated_manifest_matches_every_physical_section_target() {
    let data = GameData::load().unwrap();
    for site in data.ordered_sites() {
        let physical_targets: HashSet<&str> = site
            .sections
            .iter()
            .flat_map(|section| section.candidate_targets.iter().map(String::as_str))
            .collect();
        let generated = generate_salvage(site, 7, &[]);
        let manifest_targets: HashSet<&str> = generated.iter().map(String::as_str).collect();

        assert_eq!(manifest_targets, physical_targets, "site {}", site.id);
    }
}
