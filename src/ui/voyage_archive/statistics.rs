//! Readable lifetime, archive, operating ledger and commendation pages.

use super::*;
use crate::state::{CareerAward, CareerStats};

pub(super) fn draw(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.journal_ui;
    let state = ctx.voyage_archive;
    let labels = if state.tab == ArchiveTab::Career {
        copy.career_pages.as_slice()
    } else {
        copy.ledger_pages.as_slice()
    };
    let width = 1224.0 / labels.len() as f32;
    for (page, label) in labels.iter().enumerate() {
        control(
            ctx,
            actions,
            Rect::new(28.0 + page as f32 * width, 220.0, width - 12.0, 46.0),
            label,
            state.page == page,
            ArchiveAction::Page(page),
        );
    }
    let (fields, values) = if state.tab == ArchiveTab::Career {
        if state.page == 0 {
            (copy.career.as_slice(), career_values(ctx))
        } else {
            (copy.archive.as_slice(), archive_values(ctx))
        }
    } else {
        match state.page {
            0 => (copy.service.as_slice(), service_values(&ctx.session.career)),
            1 => (copy.supplies.as_slice(), supply_values(&ctx.session.career)),
            _ => (copy.income.as_slice(), income_values(&ctx.session.career)),
        }
    };
    metric_grid(Rect::new(28.0, 282.0, 1224.0, 378.0), fields, &values, 3);
}

fn career_values(ctx: &UiContext<'_>) -> Vec<String> {
    let stats = &ctx.session.career;
    vec![
        stats.rank_label().to_owned(),
        stats.voyages_completed.to_string(),
        stats.safe_returns.to_string(),
        stats.targets_recovered.to_string(),
        money(stats.gross_haul_value),
        money(stats.highest_haul_value),
        stats.contracts_completed.to_string(),
        stats.contract_streak.to_string(),
        stats.best_contract_streak.to_string(),
        stats.sections_cleared.to_string(),
        format!("{} / 5", stats.crew_qualified_count()),
        format!(
            "{} / {}",
            ctx.session.unlocked_module_count(ctx.data),
            ctx.data.modules.iter().count()
        ),
    ]
}

fn archive_values(ctx: &UiContext<'_>) -> Vec<String> {
    let records = &ctx.session.voyage_log;
    vec![
        records.len().to_string(),
        money(records.iter().map(|r| r.recovered_value).sum()),
        money(records.iter().map(|r| r.recovered_value).max().unwrap_or(0)),
        records
            .iter()
            .filter(|r| r.risk_outcome == RiskOutcome::OrdinaryReturn)
            .count()
            .to_string(),
        records
            .iter()
            .map(|r| u64::from(r.recovered_count))
            .sum::<u64>()
            .to_string(),
        records
            .iter()
            .map(|r| u64::from(r.external_load))
            .sum::<u64>()
            .to_string(),
        records
            .iter()
            .map(|r| i64::from(r.return_fuel))
            .sum::<i64>()
            .to_string(),
        records
            .iter()
            .map(|r| i64::from(r.recovered_alloy))
            .sum::<i64>()
            .to_string(),
        records
            .iter()
            .map(|r| i64::from(r.recovered_electronics))
            .sum::<i64>()
            .to_string(),
        money(records.iter().map(|r| r.insurance_premium).sum()),
        money(records.iter().map(|r| r.insurance_payout).sum()),
        money(records.iter().map(|r| r.clearance_payout).sum()),
    ]
}

fn service_values(stats: &CareerStats) -> Vec<String> {
    vec![
        stats.repairs_completed.to_string(),
        stats.full_overhauls.to_string(),
        stats.hull_patches.to_string(),
        stats.systems_services.to_string(),
        stats.systems_restored.to_string(),
        money(stats.repair_spend),
        stats.fuel_units_bought.to_string(),
        money(stats.refuel_spend),
        stats.cargo_bay_upgrades.to_string(),
    ]
}

fn supply_values(stats: &CareerStats) -> Vec<String> {
    vec![
        stats.field_power_cells_bought.to_string(),
        stats.field_power_cells_fabricated.to_string(),
        stats.field_power_cells_used.to_string(),
        money(stats.field_power_spend),
        stats.field_power_alloy_used.to_string(),
        stats.field_power_electronics_used.to_string(),
    ]
}

fn income_values(stats: &CareerStats) -> Vec<String> {
    vec![
        money(stats.contract_income),
        money(stats.insurance_claims),
        money(stats.sale_income),
        stats.module_changes.to_string(),
        money(stats.module_spend),
        money(
            stats
                .contract_income
                .saturating_add(stats.insurance_claims)
                .saturating_add(stats.sale_income),
        ),
    ]
}

pub(super) fn draw_awards(ctx: &UiContext<'_>) {
    let copy = &ctx.data.journal_ui;
    let stats = &ctx.session.career;
    text(
        &format!(
            "{} / {}  {}",
            stats.earned_awards().len(),
            CareerAward::ALL.len(),
            copy.earned
        ),
        Rect::new(30.0, 226.0, 1200.0, 34.0),
        24.0,
        visual_theme::cyan(),
    );
    for (index, award) in CareerAward::ALL.into_iter().enumerate() {
        let rect = Rect::new(
            28.0 + (index % 2) as f32 * 618.0,
            280.0 + (index / 2) as f32 * 96.0,
            606.0,
            84.0,
        );
        let earned = award.is_earned(stats);
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, visual_theme::panel());
        draw_rectangle(
            rect.x,
            rect.y,
            4.0,
            rect.h,
            if earned {
                visual_theme::safe()
            } else {
                visual_theme::structure()
            },
        );
        text(
            award.label(),
            Rect::new(rect.x + 18.0, rect.y + 10.0, 368.0, 30.0),
            24.0,
            visual_theme::text(),
        );
        text(
            if earned { &copy.earned } else { &copy.pending },
            Rect::new(rect.x + 420.0, rect.y + 14.0, 175.0, 27.0),
            18.0,
            if earned {
                visual_theme::safe()
            } else {
                visual_theme::text()
            },
        );
        text(
            &copy.award_requirements[index],
            Rect::new(rect.x + 18.0, rect.y + 48.0, 570.0, 29.0),
            20.0,
            visual_theme::text(),
        );
    }
}
