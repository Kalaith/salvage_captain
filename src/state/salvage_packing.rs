//! Expedition packing screen identity.

use super::{
    cargo_layout_id, CareerAward, CargoStatus, ExpeditionState, GameSession, ReturnPolicy,
    ReturnedItem,
};
use crate::data::GameData;
use crate::engine::{claim_payout, danger_after_intel, resolve_risk, RiskOutcome, RiskResult};
use crate::state::return_policy::select_casualty;
use crate::state::workspace::TransferMode;

pub const TITLE: &str = "PACK THE HAUL";

struct PackingContext {
    return_fuel: i32,
    external_load: i32,
    return_policy: ReturnPolicy,
    protected_objective: Option<String>,
    awards_before: Vec<CareerAward>,
}

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
        )
        .saturating_sub(self.route_familiarity_danger_reduction(&expedition.site_id));
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
        let context = self.prepare_packing(data)?;
        self.refresh_expedition_risk(data);
        let mut expedition = self
            .expedition
            .take()
            .ok_or_else(|| "there is no active expedition".to_owned())?;
        self.last_return_policy = context.return_policy;
        let scan_profile = expedition.scan_profile;
        let insured = expedition.insured;
        let insurance_premium = self.insurance_premium(&expedition, insured, data);
        let reconnaissance_level = self.reconnaissance_level(&expedition.site_id);
        let voyage_plan = expedition.voyage_plan;
        let (mut message, emergency_bill) =
            self.resolve_return_consequences(&mut expedition, data, &context);
        let impacted_value = self.impacted_value(&expedition, data);
        let insurance_payout =
            self.insurance_payout(&expedition, insured, impacted_value, emergency_bill, data);
        self.append_insurance_claim(&mut message, insurance_payout);
        self.append_contract_result(&mut message, &expedition, data);
        self.returned = returned_cargo(&expedition, self.market_cycle);
        let condition_after = self.update_site_after_return(&expedition.site_id);
        let clearance = self.resolve_section_clearance(&expedition.site_id, data);
        message.push_str(&clearance.message(data));
        let (contract_completed, contract_failed) = self.contract_result(&expedition);
        let crew_experience_before = self.crew_experience();
        let crew_expertise_before = self.crew_expertise_level();
        self.record_voyage(crate::state::VoyageRecordInput {
            site_id: &expedition.site_id,
            risk: &expedition.risk,
            voyage_plan,
            return_policy: context.return_policy,
            reconnaissance_level,
            external_load: context.external_load,
            contract_completed,
            contract_failed,
            contract_accepted: expedition.contract_accepted,
            scan_profile,
            drone_directive: expedition.drone_directive,
            condition_after,
            return_fuel: context.return_fuel,
            cleared_sections: &clearance.section_ids,
            clearance_payout: clearance.payout,
            insured,
            insurance_premium,
            insurance_payout,
            data,
        });
        self.append_experience_report(
            &mut message,
            crew_experience_before,
            crew_expertise_before,
            &context.awards_before,
        );
        self.economy.fuel -= context.return_fuel;
        self.last_risk = Some(expedition.risk);
        self.market_cycle = self.market_cycle.wrapping_add(1);
        Ok(format!(
            "{message} Return burn: {} fuel. {remaining} fuel remains.",
            context.return_fuel,
            remaining = self.economy.fuel
        ))
    }

    fn prepare_packing(&self, data: &GameData) -> Result<PackingContext, String> {
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
        Ok(PackingContext {
            return_fuel,
            external_load: self.external_cargo_count(data, None),
            return_policy,
            protected_objective,
            awards_before: self.career.earned_awards(),
        })
    }

    fn refresh_expedition_risk(&mut self, data: &GameData) {
        if let Some(risk) = self.expedition_risk_preview(data) {
            if let Some(expedition) = self.expedition.as_mut() {
                expedition.risk = risk;
            }
        }
    }

    fn resolve_return_consequences(
        &mut self,
        expedition: &mut ExpeditionState,
        data: &GameData,
        context: &PackingContext,
    ) -> (String, i64) {
        let fatigue_gain = self.register_crew_fatigue(
            expedition.risk.outcome,
            context.external_load,
            expedition.power_cycles_used,
        );
        let wear_gain = self.register_ship_wear(
            expedition.risk.outcome,
            context.external_load,
            expedition.power_cycles_used,
            &data.config.maintenance,
        );
        let mut message = expedition.risk.explanation.clone();
        if context.external_load > 0 {
            let strain = context.external_load * data.config.risk.external_cargo_risk_per_item;
            message.push_str(&format!(" External load added +{strain} risk."));
        }
        message.push_str(&format!(
            " Return policy: {}.",
            context.return_policy.label()
        ));
        message.push_str(&format!(
            " Crew fatigue +{fatigue_gain}; readiness {}%.",
            self.crew_readiness()
        ));
        message.push_str(&format!(
            " Ship wear +{wear_gain}; systems {}% worn.",
            self.ship_wear()
        ));
        let emergency_bill = match expedition.risk.outcome {
            RiskOutcome::OrdinaryReturn => 0,
            RiskOutcome::DamagedModule => {
                message.push_str(&self.apply_workspace_damage(data));
                0
            }
            RiskOutcome::LostSalvage => {
                message.push_str(&self.lose_return_cargo(
                    expedition,
                    data,
                    context.return_policy,
                    context.protected_objective.as_deref(),
                    RiskOutcome::LostSalvage,
                ));
                0
            }
            RiskOutcome::EmergencyRepair => self.apply_emergency_repair(&mut message, data),
            RiskOutcome::ForcedAbandon => {
                message.push_str(&self.lose_return_cargo(
                    expedition,
                    data,
                    context.return_policy,
                    context.protected_objective.as_deref(),
                    RiskOutcome::ForcedAbandon,
                ));
                0
            }
        };
        (message, emergency_bill)
    }

    fn lose_return_cargo(
        &mut self,
        expedition: &mut ExpeditionState,
        data: &GameData,
        return_policy: ReturnPolicy,
        protected_objective: Option<&str>,
        outcome: RiskOutcome,
    ) -> String {
        let Some(item) = select_casualty(
            &mut expedition.cargo,
            data,
            return_policy,
            protected_objective,
            outcome,
        ) else {
            return String::new();
        };
        self.ship_layout.remove(&cargo_layout_id(&item.object_id));
        item.position = None;
        match outcome {
            RiskOutcome::LostSalvage => {
                item.status = CargoStatus::Lost;
                format!(" Lost {}.", item.object_id)
            }
            RiskOutcome::ForcedAbandon => {
                item.status = CargoStatus::LeftBehind;
                format!(" Abandoned {}.", item.object_id)
            }
            _ => String::new(),
        }
    }

    fn apply_emergency_repair(&mut self, message: &mut String, data: &GameData) -> i64 {
        let bill = i64::from((data.config.repair_price_per_hull * 3).max(60));
        self.economy.credits = (self.economy.credits - bill).max(0);
        message.push_str(&format!(" Emergency bill: {bill} credits."));
        bill
    }

    fn insurance_premium(
        &self,
        expedition: &ExpeditionState,
        insured: bool,
        data: &GameData,
    ) -> i64 {
        if insured {
            self.insurance_quote_with_plan(&expedition.site_id, data, expedition.voyage_plan)
                .map_or(0, |quote| quote.premium)
        } else {
            0
        }
    }

    fn impacted_value(&self, expedition: &ExpeditionState, data: &GameData) -> i64 {
        expedition
            .cargo
            .iter()
            .find(|item| matches!(item.status, CargoStatus::Lost | CargoStatus::LeftBehind))
            .and_then(|item| data.salvage_objects.get(&item.object_id))
            .map(|object| {
                crate::engine::market::quote_for(object, self.market_cycle, &data.config.market)
                    .sale_value
            })
            .unwrap_or(0)
    }

    fn insurance_payout(
        &self,
        expedition: &ExpeditionState,
        insured: bool,
        impacted_value: i64,
        emergency_bill: i64,
        data: &GameData,
    ) -> i64 {
        if insured {
            claim_payout(
                expedition.risk.outcome,
                impacted_value,
                emergency_bill,
                &data.config.insurance,
            )
        } else {
            0
        }
    }

    fn append_insurance_claim(&mut self, message: &mut String, insurance_payout: i64) {
        if insurance_payout > 0 {
            self.economy.credits += insurance_payout;
            self.career.record_insurance_claim(insurance_payout);
            message.push_str(&format!(" Insurance claim: +{insurance_payout} credits."));
        }
    }

    fn append_contract_result(
        &mut self,
        message: &mut String,
        expedition: &ExpeditionState,
        data: &GameData,
    ) {
        if let Some(contract_message) = self.complete_site_contract_for_run(
            &expedition.site_id,
            &expedition.cargo,
            data,
            expedition.contract_accepted,
        ) {
            message.push_str(&contract_message);
        } else if !expedition.contract_accepted {
            message.push_str(" Private haul; client contract declined. Contract standing held.");
        }
    }

    fn update_site_after_return(&mut self, site_id: &str) -> i32 {
        if let Some(progress) = self.site_progress.get_mut(site_id) {
            progress.visits += 1;
            progress.condition = (progress.condition - 18).max(0);
            progress.condition
        } else {
            0
        }
    }

    fn contract_result(&self, expedition: &ExpeditionState) -> (bool, bool) {
        if expedition.contract_accepted {
            self.site_progress
                .get(&expedition.site_id)
                .map_or((false, false), |progress| {
                    (progress.contract_completed, progress.contract_failed)
                })
        } else {
            (false, false)
        }
    }

    fn append_experience_report(
        &mut self,
        message: &mut String,
        experience_before: u16,
        expertise_before: u8,
        awards_before: &[CareerAward],
    ) {
        let experience_gain = self.crew_experience().saturating_sub(experience_before);
        message.push(' ');
        message.push_str(&self.crew_experience_report(experience_gain, expertise_before));
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
    }
}

fn returned_cargo(expedition: &ExpeditionState, market_cycle: u32) -> Vec<ReturnedItem> {
    expedition
        .cargo
        .iter()
        .filter_map(|item| {
            (item.status == CargoStatus::Packed).then_some(ReturnedItem {
                object_id: item.object_id.clone(),
                position: item.position?,
                rotation: item.rotation,
                market_cycle,
            })
        })
        .collect()
}
