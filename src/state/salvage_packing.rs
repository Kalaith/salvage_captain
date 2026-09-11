//! Expedition packing screen identity.

use super::{cargo_layout_id, CargoStatus, GameSession, ReturnPolicy, ReturnedItem};
use crate::data::GameData;
use crate::engine::{claim_payout, danger_after_intel, resolve_risk, RiskOutcome, RiskResult};
use crate::state::return_policy::select_casualty;
use crate::state::workspace::TransferMode;

pub const TITLE: &str = "PACK THE HAUL";

impl GameSession {
    pub fn external_capacity(&self, data: &GameData) -> i32 {
        self.module_stats(data).external_capacity + self.crew_external_capacity()
    }

    pub fn external_cargo_count(&self, data: &GameData, excluding: Option<&str>) -> i32 {
        let in_expedition = self.expedition.as_ref().map_or(0, |expedition| {
            expedition
                .cargo
                .iter()
                .filter(|cargo| cargo.status == CargoStatus::Packed)
                .filter(|cargo| Some(cargo.object_id.as_str()) != excluding)
                .filter(|cargo| {
                    data.salvage_objects
                        .get(&cargo.object_id)
                        .is_some_and(|object| TransferMode::from_target(object).uses_external_rig())
                })
                .count() as i32
        });
        let returned = self.returned.iter().filter(|cargo| {
            Some(cargo.object_id.as_str()) != excluding
                && data
                    .salvage_objects
                    .get(&cargo.object_id)
                    .is_some_and(|object| TransferMode::from_target(object).uses_external_rig())
        });
        in_expedition + returned.count() as i32
    }

    pub fn expedition_risk_preview(&self, data: &GameData) -> Option<RiskResult> {
        let expedition = self.expedition.as_ref()?;
        let site = data.sites.get(&expedition.site_id)?;
        let condition = self
            .site_progress
            .get(&expedition.site_id)
            .map_or(site.condition, |progress| progress.condition);
        let condition_penalty = (100 - condition).max(0) / 4;
        let external_penalty =
            self.external_cargo_count(data, None) * data.config.risk.external_cargo_risk_per_item;
        let route_danger = danger_after_intel(
            site.danger + condition_penalty + external_penalty,
            self.reconnaissance_level(&expedition.site_id),
            &data.config.reconnaissance,
        );
        Some(resolve_risk(
            expedition.seed,
            self.maintenance_adjusted_danger(
                self.crew_adjusted_danger(
                    expedition
                        .voyage_plan
                        .adjust_danger(route_danger, &data.config.voyage_plan),
                ),
                data,
            ),
            self.hull,
            self.module_stats(data),
            &data.config.risk,
        ))
    }

    pub fn finish_packing(&mut self, data: &GameData) -> Result<String, String> {
        if self.pending_count() > 0 {
            return Err("leave or discard every unplaced object first".to_owned());
        }
        let return_fuel = data.config.safe_return_buffer.max(0);
        if self.economy.fuel < return_fuel {
            return Err(format!(
                "the return burn requires {} fuel after packing",
                return_fuel
            ));
        }
        let external_load = self.external_cargo_count(data, None);
        let return_policy = self
            .expedition
            .as_ref()
            .map_or(ReturnPolicy::Standard, |expedition| {
                expedition.return_policy
            });
        let protected_objective = self.expedition.as_ref().and_then(|expedition| {
            self.contract_objective_status(&expedition.site_id, data)
                .filter(|status| {
                    matches!(
                        status.state,
                        crate::state::contracts::ContractObjectiveState::Open
                            | crate::state::contracts::ContractObjectiveState::Recovered
                    )
                })
                .map(|status| status.target_id)
        });
        let awards_before = self.career.earned_awards();
        if let Some(risk) = self.expedition_risk_preview(data) {
            if let Some(expedition) = self.expedition.as_mut() {
                expedition.risk = risk;
            }
        }
        let mut expedition = self
            .expedition
            .take()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        self.last_return_policy = return_policy;
        let fatigue_gain = self.register_crew_fatigue(
            expedition.risk.outcome,
            external_load,
            expedition.power_cycles_used,
        );
        let wear_gain = self.register_ship_wear(
            expedition.risk.outcome,
            external_load,
            expedition.power_cycles_used,
            &data.config.maintenance,
        );
        let reconnaissance_level = self.reconnaissance_level(&expedition.site_id);
        let insured = expedition.insured;
        let insurance_premium = if insured {
            self.insurance_quote_with_plan(&expedition.site_id, data, expedition.voyage_plan)
                .map_or(0, |quote| quote.premium)
        } else {
            0
        };
        let scan_profile = expedition.scan_profile;
        let mut message = expedition.risk.explanation.clone();
        let mut emergency_bill = 0;
        if external_load > 0 {
            let strain = external_load * data.config.risk.external_cargo_risk_per_item;
            message.push_str(&format!(" External load added +{strain} risk."));
        }
        message.push_str(&format!(" Return policy: {}.", return_policy.label()));
        message.push_str(&format!(
            " Crew fatigue +{fatigue_gain}; readiness {}%.",
            self.crew_readiness()
        ));
        message.push_str(&format!(
            " Ship wear +{wear_gain}; systems {}% worn.",
            self.ship_wear()
        ));
        match expedition.risk.outcome {
            RiskOutcome::OrdinaryReturn => {}
            RiskOutcome::DamagedModule => {
                message.push_str(&self.apply_workspace_damage(data));
            }
            RiskOutcome::LostSalvage => {
                let lost = select_casualty(
                    &mut expedition.cargo,
                    data,
                    return_policy,
                    protected_objective.as_deref(),
                    RiskOutcome::LostSalvage,
                );
                if let Some(item) = lost {
                    self.ship_layout.remove(&cargo_layout_id(&item.object_id));
                    item.status = CargoStatus::Lost;
                    item.position = None;
                    message.push_str(&format!(" Lost {}.", item.object_id));
                }
            }
            RiskOutcome::EmergencyRepair => {
                let bill = i64::from((data.config.repair_price_per_hull * 3).max(60));
                emergency_bill = bill;
                self.economy.credits = (self.economy.credits - bill).max(0);
                message.push_str(&format!(" Emergency bill: {bill} credits."));
            }
            RiskOutcome::ForcedAbandon => {
                let abandoned = select_casualty(
                    &mut expedition.cargo,
                    data,
                    return_policy,
                    protected_objective.as_deref(),
                    RiskOutcome::ForcedAbandon,
                );
                if let Some(item) = abandoned {
                    self.ship_layout.remove(&cargo_layout_id(&item.object_id));
                    item.status = CargoStatus::LeftBehind;
                    item.position = None;
                    message.push_str(&format!(" Abandoned {}.", item.object_id));
                }
            }
        }
        let impacted_value = expedition
            .cargo
            .iter()
            .find(|item| matches!(item.status, CargoStatus::Lost | CargoStatus::LeftBehind))
            .and_then(|item| data.salvage_objects.get(&item.object_id))
            .map(|object| {
                crate::engine::market::quote_for(object, self.market_cycle, &data.config.market)
                    .sale_value
            })
            .unwrap_or(0);
        let insurance_payout = if insured {
            claim_payout(
                expedition.risk.outcome,
                impacted_value,
                emergency_bill,
                &data.config.insurance,
            )
        } else {
            0
        };
        if insurance_payout > 0 {
            self.economy.credits += insurance_payout;
            self.career.record_insurance_claim(insurance_payout);
            message.push_str(&format!(" Insurance claim: +{insurance_payout} credits."));
        }
        if let Some(contract_message) =
            self.complete_site_contract(&expedition.site_id, &expedition.cargo, data)
        {
            message.push_str(&contract_message);
        }
        self.returned = expedition
            .cargo
            .iter()
            .filter_map(|item| {
                (item.status == CargoStatus::Packed).then_some(ReturnedItem {
                    object_id: item.object_id.clone(),
                    position: item.position?,
                    rotation: item.rotation,
                    market_cycle: self.market_cycle,
                })
            })
            .collect();
        let condition_after =
            if let Some(progress) = self.site_progress.get_mut(&expedition.site_id) {
                progress.visits += 1;
                progress.condition = (progress.condition - 18).max(0);
                progress.condition
            } else {
                0
            };
        let clearance = self.resolve_section_clearance(&expedition.site_id, data);
        message.push_str(&clearance.message(data));
        let (contract_completed, contract_failed) = self
            .site_progress
            .get(&expedition.site_id)
            .map_or((false, false), |progress| {
                (progress.contract_completed, progress.contract_failed)
            });
        let crew_experience_before = self.crew_experience();
        let crew_expertise_before = self.crew_expertise_level();
        self.record_voyage(
            &expedition.site_id,
            &expedition.risk,
            expedition.voyage_plan,
            return_policy,
            reconnaissance_level,
            external_load,
            contract_completed,
            contract_failed,
            scan_profile,
            expedition.drone_directive,
            condition_after,
            return_fuel,
            &clearance.section_ids,
            clearance.payout,
            insured,
            insurance_premium,
            insurance_payout,
            data,
        );
        let crew_experience_gain = self
            .crew_experience()
            .saturating_sub(crew_experience_before);
        message.push(' ');
        message.push_str(&self.crew_experience_report(crew_experience_gain, crew_expertise_before));
        let new_awards: Vec<_> = self
            .career
            .earned_awards()
            .into_iter()
            .filter(|award| !awards_before.contains(award))
            .collect();
        if !new_awards.is_empty() {
            message.push_str(&format!(
                " Commendation filed: {}.",
                new_awards
                    .iter()
                    .map(|award| award.label())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        self.economy.fuel -= return_fuel;
        self.last_risk = Some(expedition.risk);
        self.market_cycle = self.market_cycle.wrapping_add(1);
        Ok(format!(
            "{message} Return burn: {return_fuel} fuel. {remaining} fuel remains.",
            remaining = self.economy.fuel
        ))
    }
}
