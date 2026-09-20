//! Paged active and archived wreck choices with visible discovery controls.

use super::*;
use crate::data::discovery::LeadKind;
use crate::data::{GameData, SiteData};

pub const BOARD_PAGE_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardAction {
    Previous,
    Next,
    ToggleArchive,
}

impl WreckSelection {
    pub fn page_count(&self, session: &GameSession, data: &GameData) -> usize {
        session
            .listed_wrecks(data, self.archived)
            .len()
            .div_ceil(BOARD_PAGE_SIZE)
            .max(1)
    }

    pub fn visible_sites<'a>(
        &self,
        session: &GameSession,
        data: &'a GameData,
    ) -> Vec<&'a SiteData> {
        session
            .listed_wrecks(data, self.archived)
            .into_iter()
            .skip(self.page.min(self.page_count(session, data) - 1) * BOARD_PAGE_SIZE)
            .take(BOARD_PAGE_SIZE)
            .collect()
    }

    pub fn selected_on_board<'a>(
        &self,
        session: &GameSession,
        data: &'a GameData,
    ) -> Option<&'a SiteData> {
        let visible = self.visible_sites(session, data);
        visible
            .iter()
            .copied()
            .find(|site| Some(&site.id) == self.site_id.as_ref())
            .or_else(|| visible.first().copied())
    }

    pub fn normalize_board(&mut self, session: &GameSession, data: &GameData) {
        self.page = self.page.min(self.page_count(session, data) - 1);
        self.site_id = self
            .selected_on_board(session, data)
            .map(|site| site.id.clone());
    }

    pub fn apply_board(&mut self, action: BoardAction, session: &GameSession, data: &GameData) {
        match action {
            BoardAction::Previous => self.page = self.page.saturating_sub(1),
            BoardAction::Next => {
                self.page = (self.page + 1).min(self.page_count(session, data) - 1)
            }
            BoardAction::ToggleArchive => {
                self.archived = !self.archived;
                self.page = 0;
            }
        }
        self.details_open = false;
        self.normalize_board(session, data);
    }
}

pub(super) fn draw_toolbar(ctx: &UiContext<'_>, actions: &mut Vec<UiAction>) {
    let copy = &ctx.data.discovery.copy;
    let specialist = copy
        .specialist
        .replace("{cost}", &ctx.data.discovery.specialist_cost.to_string());
    let page = ctx
        .wreck_selection
        .page
        .min(ctx.wreck_selection.page_count(ctx.session, ctx.data) - 1);
    let pages = ctx.wreck_selection.page_count(ctx.session, ctx.data);
    for (rect, label, action, enabled) in [
        (
            Rect::new(24.0, 126.0, 210.0, 46.0),
            copy.find.as_str(),
            UiAction::DiscoverWreck(LeadKind::Local),
            ctx.session
                .discovery_block_reason(LeadKind::Local, ctx.data)
                .is_none(),
        ),
        (
            Rect::new(244.0, 126.0, 234.0, 46.0),
            specialist.as_str(),
            UiAction::DiscoverWreck(LeadKind::Specialist),
            ctx.session
                .discovery_block_reason(LeadKind::Specialist, ctx.data)
                .is_none(),
        ),
        (
            Rect::new(488.0, 126.0, 166.0, 46.0),
            if ctx.wreck_selection.archived {
                copy.active.as_str()
            } else {
                copy.archive.as_str()
            },
            UiAction::WreckBoard(BoardAction::ToggleArchive),
            true,
        ),
    ] {
        if button(ctx, rect, label, enabled, ButtonTone::Secondary) {
            actions.push(action);
        }
    }
    if pages > 1 {
        for (rect, label, action, enabled) in [
            (
                Rect::new(664.0, 126.0, 140.0, 46.0),
                copy.previous.as_str(),
                UiAction::WreckBoard(BoardAction::Previous),
                page > 0,
            ),
            (
                Rect::new(814.0, 126.0, 140.0, 46.0),
                copy.next.as_str(),
                UiAction::WreckBoard(BoardAction::Next),
                page + 1 < pages,
            ),
        ] {
            if button(ctx, rect, label, enabled, ButtonTone::Secondary) {
                actions.push(action);
            }
        }
    }
    let count = ctx
        .session
        .listed_wrecks(ctx.data, ctx.wreck_selection.archived)
        .len();
    text(
        &format!(
            "{} {count} · {}/{pages}",
            if ctx.wreck_selection.archived {
                &copy.archive
            } else {
                &copy.active
            },
            page + 1
        ),
        974.0,
        134.0,
        282.0,
        32.0,
        22.0,
        visual_theme::text_dim(),
    );
}

pub(super) fn draw_empty(ctx: &UiContext<'_>) {
    let copy = &ctx.data.discovery.copy;
    visual_theme::surface(Rect::new(24.0, 184.0, 1232.0, 512.0));
    text(
        if ctx.wreck_selection.archived {
            &copy.archive_empty
        } else {
            &copy.empty
        },
        64.0,
        266.0,
        1140.0,
        90.0,
        32.0,
        visual_theme::text(),
    );
    text(
        &copy.tutorial,
        64.0,
        398.0,
        1100.0,
        170.0,
        27.0,
        visual_theme::text_dim(),
    );
}
