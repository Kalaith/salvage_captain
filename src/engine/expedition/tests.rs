use super::*;
use crate::data::GameData;

#[test]
fn contract_objective_is_always_in_the_generated_manifest() {
    let data = GameData::load().unwrap();
    for site in data.ordered_sites() {
        let contract_target = site.contract_target.as_ref().unwrap();
        for seed in 0..12 {
            assert!(generate_salvage(site, seed).contains(contract_target));
        }
    }
}
