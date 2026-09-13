//! Seeded generation from authored pools, committed once at discovery.

use crate::data::discovery::{starter_target, WreckPool};
use crate::data::GameData;
use crate::state::wrecks::{item_id, wreck_id, SalvageInstance, WreckInstance};
use macroquad_toolkit::rng::SeededRng;

pub fn generate_wreck(
    pool: &WreckPool,
    serial: u64,
    seed: u64,
    data: &GameData,
) -> Result<WreckInstance, String> {
    let tuning = &data.discovery;
    let mut rng = SeededRng::new(seed);
    let mut site = data
        .sites
        .get(&pool.template_id)
        .ok_or_else(|| format!("missing wreck template '{}'", pool.template_id))?
        .clone();
    site.id = wreck_id(&pool.template_id, serial);
    site.display_name = tuning
        .name_pattern
        .replace("{class}", &site.display_name)
        .replace("{number}", &format!("{serial:04}"));
    site.condition = rng.range_i32(tuning.condition_min, tuning.condition_max + 1);
    site.danger = (site.danger
        + rng.range_i32(-tuning.danger_variation, tuning.danger_variation + 1))
    .clamp(0, 100);
    site.fuel_cost += rng.range_i32(0, 3);
    site.candidate_salvage.clear();
    let mut targets = Vec::new();
    for (section_index, section) in site.sections.iter_mut().enumerate() {
        section.candidate_targets.clear();
        let count =
            tuning.minimum_targets + rng.below(tuning.maximum_targets - tuning.minimum_targets + 1);
        for slot in 0..count {
            // Every wreck includes an accessible objective in its entry section.
            let accessible = section_index == 0 && slot == 0;
            let template_id = choose_target(pool, accessible, &mut rng, data)?;
            let mut object = data
                .salvage_objects
                .get(&template_id)
                .ok_or_else(|| format!("missing salvage template '{template_id}'"))?
                .clone();
            object.id = item_id(&site.id, section_index, slot);
            let old_integrity = object.integrity.max(1);
            object.integrity = (object.integrity
                + rng.range_i32(-tuning.integrity_variation, tuning.integrity_variation + 1))
            .clamp(30, 100);
            object.sale_value =
                (object.sale_value * i64::from(object.integrity) / i64::from(old_integrity)).max(1);
            object.extraction_difficulty = (object.extraction_difficulty
                + (old_integrity - object.integrity) / 3)
                .clamp(0, 100);
            section.candidate_targets.push(object.id.clone());
            site.candidate_salvage.push(object.id.clone());
            targets.push(SalvageInstance {
                template_id,
                section_index,
                slot,
                object,
            });
        }
    }
    let target = targets
        .first()
        .ok_or_else(|| "wreck has no contract target".to_owned())?;
    site.contract_target = Some(target.object.id.clone());
    site.contract_brief.clone_from(&tuning.contract_brief);
    // Cover the round-trip fuel before pricing danger and extraction work.
    site.contract_reward = tuning.reward_base
        + i64::from(site.fuel_cost + data.config.safe_return_buffer)
            * i64::from(data.config.refuel_price_per_unit)
        + i64::from(site.danger) * tuning.reward_per_danger
        + i64::from(target.object.extraction_difficulty) * tuning.reward_per_difficulty;
    Ok(WreckInstance {
        serial,
        template_id: pool.template_id.clone(),
        seed,
        site,
        targets,
    })
}

fn choose_target(
    pool: &WreckPool,
    accessible: bool,
    rng: &mut SeededRng,
    data: &GameData,
) -> Result<String, String> {
    let choices: Vec<_> = pool
        .loot
        .iter()
        .filter(|entry| {
            !accessible
                || data
                    .salvage_objects
                    .get(&entry.object_id)
                    .is_some_and(starter_target)
        })
        .collect();
    let total: usize = choices.iter().map(|entry| entry.weight).sum();
    if total == 0 {
        return Err("wreck pool has no eligible salvage".to_owned());
    }
    let mut roll = rng.below(total);
    for choice in choices {
        if roll < choice.weight {
            return Ok(choice.object_id.clone());
        }
        roll -= choice.weight;
    }
    Err("wreck pool weights could not be resolved".to_owned())
}
