use super::*;

#[test]
fn embedded_content_loads_and_cross_references_validate() {
    let data = GameData::load().unwrap();
    assert_eq!(data.config.grid_width, 5);
    assert_eq!(data.sites.len(), 3);
    assert!(data.salvage_objects.contains("damaged_reactor"));
    assert!(data.modules.contains("nav_module"));
}

#[test]
fn authored_wrecks_keep_distinct_visual_profiles() {
    let data = GameData::load().unwrap();
    let themes: Vec<_> = data
        .ordered_sites()
        .into_iter()
        .map(|site| site.visual_theme.as_str())
        .collect();

    assert_eq!(themes, vec!["merchant", "military", "research"]);
}

#[test]
fn sites_reject_unknown_visual_themes() {
    let mut data = GameData::load().unwrap();
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.visual_theme = "industrial_blue".to_owned();
    data.sites.insert("merchant_wreck".to_owned(), site);

    let error = data.validate().unwrap_err();

    assert!(error.contains("unknown visual theme"));
}

#[test]
fn rotations_swap_rectangular_dimensions() {
    let footprint = Footprint {
        width: 1,
        height: 2,
    };
    assert_eq!(footprint.rotated(0), footprint);
    assert_eq!(
        footprint.rotated(1),
        Footprint {
            width: 2,
            height: 1
        }
    );
}

#[test]
fn section_connections_must_be_reciprocal() {
    let mut data = GameData::load().unwrap();
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.sections[1].connected_sections.clear();
    data.sites.insert("merchant_wreck".to_owned(), site);
    let error = data.validate().unwrap_err();
    assert!(error.contains("not reciprocal"));
}

#[test]
fn salvage_transfer_modes_are_explicit() {
    let mut data = GameData::load().unwrap();
    let mut object = data.salvage_objects.remove("industrial_battery").unwrap();
    object.transfer_mode = "teleport".to_owned();
    data.salvage_objects
        .insert("industrial_battery".to_owned(), object);
    let error = data.validate().unwrap_err();
    assert!(error.contains("unknown transfer mode"));
}

#[test]
fn every_wreck_has_a_valid_contract_objective() {
    let data = GameData::load().unwrap();
    for site in data.ordered_sites() {
        let target = site.contract_target.as_ref().unwrap();
        assert!(site.contract_reward > 0);
        assert!(site.candidate_salvage.contains(target));
        assert!(data.salvage_objects.contains(target));
    }
}

#[test]
fn contract_targets_cannot_have_zero_payouts() {
    let mut data = GameData::load().unwrap();
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.contract_reward = 0;
    data.sites.insert("merchant_wreck".to_owned(), site);

    let error = data.validate().unwrap_err();

    assert!(error.contains("contract target needs a positive reward"));
}

#[test]
fn contract_targets_cannot_have_empty_briefs() {
    let mut data = GameData::load().unwrap();
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.contract_brief.clear();
    data.sites.insert("merchant_wreck".to_owned(), site);

    let error = data.validate().unwrap_err();

    assert!(error.contains("contract target needs a briefing"));
}

#[test]
fn drone_support_cannot_be_negative() {
    let mut data = GameData::load().unwrap();
    let mut module = data.modules.remove("drone_bay").unwrap();
    module.drone_support = -1;
    data.modules.insert("drone_bay".to_owned(), module);

    let error = data.validate().unwrap_err();

    assert!(error.contains("negative module cost or capacity"));
}

#[test]
fn section_target_rosters_cannot_repeat_a_target() {
    let mut data = GameData::load().unwrap();
    let mut site = data.sites.remove("merchant_wreck").unwrap();
    site.sections[0]
        .candidate_targets
        .push("industrial_battery".to_owned());
    data.sites.insert("merchant_wreck".to_owned(), site);

    let error = data.validate().unwrap_err();

    assert!(error.contains("duplicate target 'industrial_battery'"));
}

#[test]
fn external_cargo_penalty_cannot_be_negative() {
    let mut data = GameData::load().unwrap();
    data.config.risk.external_cargo_risk_per_item = -1;

    let error = data.validate().unwrap_err();

    assert!(error.contains("negative external cargo penalty"));
}

#[test]
fn workspace_scan_cost_cannot_be_negative() {
    let mut data = GameData::load().unwrap();
    data.config.workspace_scan_energy_cost = -1;

    let error = data.validate().unwrap_err();

    assert!(error.contains("workspace scan energy cost"));
}

#[test]
fn risk_weights_must_form_a_complete_distribution() {
    let mut data = GameData::load().unwrap();
    data.config.risk.ordinary_return_weight = 101;

    let error = data.validate().unwrap_err();

    assert!(error.contains("weights must be non-negative and total 100"));
}

#[test]
fn non_starting_modules_cannot_be_free() {
    let mut data = GameData::load().unwrap();
    let mut module = data.modules.remove("scanner_module").unwrap();
    module.purchase_cost = 0;
    data.modules.insert("scanner_module".to_owned(), module);

    let error = data.validate().unwrap_err();

    assert!(error.contains("non-starting modules need a positive purchase cost"));
}
