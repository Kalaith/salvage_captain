//! Deterministic salvage manifest generation rules.

use crate::data::GameData;
use crate::engine::expedition::generate_salvage;

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
