use super::*;
use crate::data::{Footprint, GameData, GridPosition};
use crate::engine::{Disposition, RiskOutcome, VoyagePlan};

#[test]
fn new_game_has_a_valid_starter_layout_and_safe_economy() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    assert_eq!(session.ship_layout.occupied_cells(), 5);
    assert_eq!(session.economy.credits, data.config.starting_credits);
    assert_eq!(session.economy.fuel, data.config.starting_fuel);
}

#[test]
fn expedition_spends_fuel_and_generates_five_objects() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let message = session.begin_expedition("merchant_wreck", &data).unwrap();
    assert_eq!(session.economy.fuel, 8);
    assert_eq!(session.expedition.as_ref().unwrap().cargo.len(), 5);
    assert!(message.contains("Manifest: 5 target(s) remain"));
}

#[test]
fn save_round_trip_preserves_layout_and_resources() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.briefing_voyage_plan = VoyagePlan::Expedited;
    let save = session.to_save(&data.config.version);
    let json = serde_json::to_value(save).unwrap();
    let restored: SaveData = serde_json::from_value(json).unwrap();
    let restored = GameSession::from_save(restored, &data).unwrap();
    assert_eq!(restored.ship_layout, session.ship_layout);
    assert_eq!(restored.economy, session.economy);
    assert_eq!(restored.briefing_voyage_plan, VoyagePlan::Expedited);
}

#[test]
fn legacy_save_defaults_the_briefing_plan_to_standard() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let mut value = serde_json::to_value(session.to_save("2.17.0")).unwrap();
    value["session"]
        .as_object_mut()
        .unwrap()
        .remove("briefing_voyage_plan");

    let migrated =
        crate::state::migrate_save_value(Some("2.17.0".to_owned()), value, &data).unwrap();
    assert_eq!(migrated.session.briefing_voyage_plan, VoyagePlan::Standard);
}

#[test]
fn selling_returns_credits() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let first = session.expedition.as_ref().unwrap().cargo[0]
        .object_id
        .clone();
    session.auto_place(&first, &data).unwrap();
    let remaining: Vec<String> = session
        .expedition
        .as_ref()
        .unwrap()
        .cargo
        .iter()
        .skip(1)
        .map(|item| item.object_id.clone())
        .collect();
    for item in remaining {
        session
            .set_cargo_status(&item, CargoStatus::LeftBehind)
            .unwrap();
    }
    session.finish_packing(&data).unwrap();
    session.dispose(&first, Disposition::Sell, &data).unwrap();
    assert!(session.economy.credits > data.config.starting_credits);
}

#[test]
fn finishing_packing_burns_the_reserved_return_fuel() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.leave_all_pending().unwrap();
    let fuel_before_return = session.economy.fuel;

    let message = session.finish_packing(&data).unwrap();

    assert_eq!(
        session.economy.fuel,
        fuel_before_return - data.config.safe_return_buffer
    );
    assert!(message.contains("Return burn: 2 fuel"));
}

#[test]
fn invalid_saved_layout_is_rejected() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.ship_layout.placements[0].position = crate::data::GridPosition::new(4, 4);
    let save = session.to_save(&data.config.version);
    assert!(GameSession::from_save(save, &data).is_err());
}

#[test]
fn invalid_saved_cargo_phase_is_rejected() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let object_id = session.expedition.as_ref().unwrap().cargo[0]
        .object_id
        .clone();
    session.auto_place(&object_id, &data).unwrap();
    session
        .expedition
        .as_mut()
        .unwrap()
        .cargo
        .iter_mut()
        .find(|item| item.object_id == object_id)
        .unwrap()
        .status = CargoStatus::Pending;
    let save = session.to_save(&data.config.version);
    assert!(GameSession::from_save(save, &data).is_err());
}

#[test]
fn save_rejects_invalid_workspace_power_reserve() {
    let data = GameData::load().unwrap();
    for (energy, capacity) in [(-1, 12), (13, 12), (1, 0)] {
        let mut session = GameSession::new(&data);
        session.begin_expedition("merchant_wreck", &data).unwrap();
        let expedition = session.expedition.as_mut().unwrap();
        expedition.workspace_energy = energy;
        expedition.workspace_energy_capacity = capacity;

        let error =
            GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

        assert!(error.contains("workspace power reserve"));
    }
}

#[test]
fn installed_module_can_be_removed_at_port() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    assert_eq!(session.max_fuel(&data), 24);
    session.remove_module("fuel_tank", &data).unwrap();
    assert_eq!(session.max_fuel(&data), 20);
    assert!(!session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.id == "fuel_tank"));
}

#[test]
fn starter_engine_provides_one_external_clamp() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    assert_eq!(session.external_capacity(&data), 1);
    assert_eq!(session.external_cargo_count(&data, None), 0);
}

#[test]
fn returned_external_cargo_count_matches_transfer_modes() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.returned = vec![
        ReturnedItem {
            object_id: "sealed_container".to_owned(),
            position: GridPosition::new(0, 0),
            rotation: 0,
            market_cycle: 0,
        },
        ReturnedItem {
            object_id: "medical_supplies".to_owned(),
            position: GridPosition::new(2, 0),
            rotation: 0,
            market_cycle: 0,
        },
    ];

    assert_eq!(session.external_cargo_count(&data, None), 1);
    assert_eq!(
        session.external_cargo_count(&data, Some("sealed_container")),
        0
    );
}

#[test]
fn external_cargo_respects_clamp_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("sealed_container", &data).unwrap();
    let error = session.auto_place("trade_crate", &data).unwrap_err();
    assert!(error.contains("No external clamp is free"));
}

#[test]
fn heavy_salvage_modules_expand_the_external_rig() {
    let data = GameData::load().unwrap();
    assert_eq!(
        data.modules
            .get("reactor_module")
            .unwrap()
            .external_capacity,
        2
    );
    assert_eq!(
        data.modules.get("shield_module").unwrap().external_capacity,
        1
    );
}

#[test]
fn yard_can_buy_a_scanner_into_open_ship_space() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let scanner = data.modules.get("scanner_module").unwrap();
    session.economy.credits = scanner.unlock_credits + scanner.purchase_cost;
    session.refresh_module_unlocks(&data);
    let before = session.economy.credits;
    let message = session.purchase_module("scanner_module", &data).unwrap();
    assert_eq!(session.economy.credits, before - 360);
    assert!(message.contains("Clamp capacity is now 2"));
    assert!(session.has_capability("scanner_array", &data));
    assert!(session
        .ship_layout
        .placements
        .iter()
        .any(|item| item.permanent && item.id == "scanner_module"));
}

#[test]
fn yard_rejects_unaffordable_purchase_without_mutating_ship() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let scanner = data.modules.get("scanner_module").unwrap();
    session.economy.credits = scanner.unlock_credits;
    session.refresh_module_unlocks(&data);
    session.economy.credits = 0;
    let layout_before = session.ship_layout.clone();
    let unlocked_before = session.unlocked_modules.clone();

    let error = session
        .purchase_module("scanner_module", &data)
        .unwrap_err();

    assert!(error.contains("requires 360 credits"));
    assert_eq!(session.ship_layout, layout_before);
    assert_eq!(session.unlocked_modules, unlocked_before);
    assert_eq!(session.economy.credits, 0);
}

#[test]
fn yard_rejects_purchase_when_the_grid_has_no_fit() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let scanner = data.modules.get("scanner_module").unwrap();
    session.economy.credits = scanner.unlock_credits + scanner.purchase_cost;
    session.refresh_module_unlocks(&data);
    session.ship_layout.placements.clear();
    session
        .ship_layout
        .place(
            "test_blocker",
            Footprint {
                width: 5,
                height: 5,
            },
            GridPosition::new(0, 0),
            0,
            true,
        )
        .unwrap();
    let credits_before = session.economy.credits;

    let error = session
        .purchase_module("scanner_module", &data)
        .unwrap_err();

    assert!(error.contains("no open fit"));
    assert_eq!(session.economy.credits, credits_before);
    assert_eq!(session.ship_layout.placements.len(), 1);
}

#[test]
fn damaged_engine_goes_offline_until_repaired() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());

    assert_eq!(session.module_stats(&data).power, 0);
    assert_eq!(session.tractor_capacity_tons(&data), 0.0);
    assert!(!session.has_capability("basic_tractor", &data));
    let expected_cost = session.repair_quote(&data).total_cost;

    let message = session.repair(&data).unwrap();

    assert!(session.damaged_modules.is_empty());
    assert!(message.contains("Restored: Engine Core"));
    assert_eq!(session.module_stats(&data).power, 2);
    assert!(session.has_capability("basic_tractor", &data));
    assert_eq!(session.career.repairs_completed, 1);
    assert_eq!(session.career.systems_restored, 1);
    assert_eq!(session.career.repair_spend, expected_cost);
}

#[test]
fn offline_module_service_has_a_quote_even_when_hull_is_full() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());
    session.hull = session.max_hull_with_modules(&data);

    let quote = session.repair_quote(&data);

    assert_eq!(quote.missing_hull, 0);
    assert_eq!(quote.offline_modules, 1);
    assert_eq!(quote.hull_cost, 0);
    assert_eq!(quote.module_cost, 55);
    assert_eq!(quote.total_cost, 55);
}

#[test]
fn unaffordable_service_reports_the_hull_and_module_cost_split() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());
    session.hull = session.max_hull_with_modules(&data);
    session.economy.credits = 20;

    let error = session.repair(&data).unwrap_err();

    assert_eq!(error, "repairs require 55 credits (hull 0 + modules 55)");
}

#[test]
fn repairing_hull_plating_restores_its_bonus_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let hull_plating = data.modules.get("hull_plating").unwrap();
    session
        .ship_layout
        .place(
            hull_plating.id.clone(),
            hull_plating.footprint,
            GridPosition::new(2, 0),
            0,
            true,
        )
        .unwrap();
    session.damaged_modules.push(hull_plating.id.clone());
    session.hull = session.max_hull_with_modules(&data);

    session.repair(&data).unwrap();

    assert_eq!(session.hull, data.config.max_hull + 1);
    assert_eq!(
        session.max_hull_with_modules(&data),
        data.config.max_hull + 1
    );
    assert!(session.damaged_modules.is_empty());
}

#[test]
fn damaged_fuel_tank_clamps_fuel_to_new_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules.push("engine_core".to_owned());
    session.economy.fuel = session.max_fuel(&data);

    session.apply_workspace_damage(&data);

    assert!(session.damaged_modules.contains(&"fuel_tank".to_owned()));
    assert_eq!(session.max_fuel(&data), 20);
    assert_eq!(session.economy.fuel, 20);
}

#[test]
fn save_rejects_duplicate_damaged_modules() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.damaged_modules = vec!["engine_core".to_owned(), "engine_core".to_owned()];

    let error = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

    assert!(error.contains("invalid damaged module 'engine_core'"));
}

#[test]
fn return_damage_uses_the_same_offline_system_rules() {
    let mut data = GameData::load().unwrap();
    data.config.risk.safe_danger_threshold = 0;
    data.config.risk.ordinary_return_weight = 0;
    data.config.risk.damaged_module_weight = 100;
    data.config.risk.lost_salvage_weight = 0;
    data.config.risk.emergency_repair_weight = 0;
    data.config.risk.forced_abandon_weight = 0;
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.danger = 100;
    data.sites.insert("merchant_wreck".to_owned(), site);
    let mut session = GameSession::new(&data);
    session.hull = 1;
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let expedition = session.expedition.as_mut().unwrap();
    for cargo in &mut expedition.cargo {
        cargo.status = CargoStatus::LeftBehind;
    }

    let message = session.finish_packing(&data).unwrap();

    assert!(session.damaged_modules.contains(&"engine_core".to_owned()));
    assert!(message.contains("Engine Core is marked damaged"));
    assert_eq!(session.module_stats(&data).power, 0);
}

#[test]
fn packed_contract_is_paid_when_the_run_is_closed() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let before = session.economy.credits;
    let target_id = data
        .sites
        .get("merchant_wreck")
        .unwrap()
        .contract_target
        .as_ref()
        .unwrap()
        .clone();
    session.auto_place(&target_id, &data).unwrap();
    let expedition = session.expedition.as_mut().unwrap();
    expedition.risk.outcome = RiskOutcome::OrdinaryReturn;
    for cargo in &mut expedition.cargo {
        if cargo.object_id != target_id {
            cargo.status = CargoStatus::LeftBehind;
        }
    }

    let message = session.finish_packing(&data).unwrap();

    assert_eq!(session.economy.credits, before + 180);
    assert!(message.contains("Contract complete"));
    assert!(
        session
            .site_progress
            .get("merchant_wreck")
            .unwrap()
            .contract_completed
    );
}

#[test]
fn revisiting_a_wreck_does_not_regenerate_removed_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.scan_workspace(&data).unwrap();
    session
        .recover_workspace_target("industrial_battery", &data)
        .unwrap();
    for cargo in &mut session.expedition.as_mut().unwrap().cargo {
        cargo.status = CargoStatus::LeftBehind;
    }
    session.finish_packing(&data).unwrap();
    session.economy.fuel = data.config.starting_fuel;

    session.begin_expedition("merchant_wreck", &data).unwrap();

    assert!(!session
        .expedition
        .as_ref()
        .unwrap()
        .cargo
        .iter()
        .any(|cargo| cargo.object_id == "industrial_battery"));
    assert_eq!(session.site_recovery_summary("merchant_wreck", &data).0, 1);
}

#[test]
fn external_haul_raises_the_return_risk_preview() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    let before = session.expedition_risk_preview(&data).unwrap();

    session.auto_place("sealed_container", &data).unwrap();

    let after = session.expedition_risk_preview(&data).unwrap();
    assert_eq!(session.external_cargo_count(&data, None), 1);
    assert_eq!(
        after.danger_score - before.danger_score,
        data.config.risk.external_cargo_risk_per_item
    );
}

#[test]
fn return_notice_reports_external_haul_strain() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("sealed_container", &data).unwrap();
    for cargo in &mut session.expedition.as_mut().unwrap().cargo {
        if cargo.object_id != "sealed_container" {
            cargo.status = CargoStatus::LeftBehind;
        }
    }

    let message = session.finish_packing(&data).unwrap();

    assert!(message.contains("External load added +8 risk"));
}

#[test]
fn save_rejects_an_external_haul_over_clamp_capacity() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.begin_expedition("merchant_wreck", &data).unwrap();
    session.auto_place("sealed_container", &data).unwrap();
    let trade_index = session
        .expedition
        .as_ref()
        .unwrap()
        .cargo
        .iter()
        .position(|cargo| cargo.object_id == "trade_crate")
        .unwrap();
    session
        .ship_layout
        .place(
            "cargo:trade_crate",
            data.salvage_objects.get("trade_crate").unwrap().footprint,
            GridPosition::new(2, 1),
            0,
            false,
        )
        .unwrap();
    let trade = &mut session.expedition.as_mut().unwrap().cargo[trade_index];
    trade.status = CargoStatus::Packed;
    trade.position = Some(GridPosition::new(2, 1));

    let error = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

    assert!(error.contains("external clamp capacity"));
}

#[test]
fn save_rejects_duplicate_removed_targets() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .removed_targets = vec![
        "industrial_battery".to_owned(),
        "industrial_battery".to_owned(),
    ];

    let error = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

    assert!(error.contains("unknown removed target"));
}

#[test]
fn save_rejects_duplicate_discovered_sections() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session
        .site_progress
        .get_mut("merchant_wreck")
        .unwrap()
        .discovered_sections = vec!["cargo_bay".to_owned(), "cargo_bay".to_owned()];

    let error = GameSession::from_save(session.to_save(&data.config.version), &data).unwrap_err();

    assert!(error.contains("duplicate discovered section"));
}
