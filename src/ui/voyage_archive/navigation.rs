//! Journal navigation independent of rendering and persistent save data.

use crate::state::VoyageRecord;

pub const ARCHIVE_PAGE_SIZE: usize = 5;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ArchiveTab {
    #[default]
    Voyages,
    Career,
    Ledger,
    Awards,
}

impl ArchiveTab {
    pub const ALL: [Self; 4] = [Self::Voyages, Self::Career, Self::Ledger, Self::Awards];

    pub const fn page_count(self) -> usize {
        match self {
            Self::Voyages | Self::Career => 2,
            Self::Ledger => 3,
            Self::Awards => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ArchiveFilter {
    #[default]
    All,
    Merchant,
    Military,
    Research,
}

impl ArchiveFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Merchant, Self::Military, Self::Research];

    pub fn matches(self, site_id: &str) -> bool {
        match self {
            Self::All => true,
            Self::Merchant => site_id == "merchant_wreck",
            Self::Military => site_id == "military_wreck",
            Self::Research => site_id == "research_vessel",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveAction {
    Tab(ArchiveTab),
    Filter(ArchiveFilter),
    Select(usize),
    Older,
    Newer,
    Page(usize),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArchiveState {
    pub tab: ArchiveTab,
    pub filter: ArchiveFilter,
    pub offset: usize,
    pub selected: Option<usize>,
    pub page: usize,
}

impl ArchiveState {
    pub fn matching_indices(&self, records: &[VoyageRecord]) -> Vec<usize> {
        records
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, record)| self.filter.matches(&record.site_id))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn page_start(&self, total: usize) -> usize {
        (self.offset / ARCHIVE_PAGE_SIZE * ARCHIVE_PAGE_SIZE)
            .min(total.saturating_sub(1) / ARCHIVE_PAGE_SIZE * ARCHIVE_PAGE_SIZE)
    }

    pub fn visible_indices(&self, records: &[VoyageRecord]) -> Vec<usize> {
        let indices = self.matching_indices(records);
        let start = self.page_start(indices.len());
        indices
            .into_iter()
            .skip(start)
            .take(ARCHIVE_PAGE_SIZE)
            .collect()
    }

    pub fn selected_index(&self, records: &[VoyageRecord]) -> Option<usize> {
        let visible = self.visible_indices(records);
        self.selected
            .filter(|index| visible.contains(index))
            .or_else(|| visible.first().copied())
    }

    pub fn apply(&mut self, action: ArchiveAction, records: &[VoyageRecord]) {
        match action {
            ArchiveAction::Tab(tab) => {
                self.tab = tab;
                self.page = 0;
            }
            ArchiveAction::Filter(filter) => {
                self.filter = filter;
                self.offset = 0;
                self.selected = None;
            }
            ArchiveAction::Select(index) => {
                if self.visible_indices(records).contains(&index) {
                    self.selected = Some(index);
                }
            }
            ArchiveAction::Older | ArchiveAction::Newer => {
                let total = self.matching_indices(records).len();
                let start = self.page_start(total);
                self.offset = if action == ArchiveAction::Older {
                    (start.saturating_add(ARCHIVE_PAGE_SIZE))
                        .min(total.saturating_sub(1) / ARCHIVE_PAGE_SIZE * ARCHIVE_PAGE_SIZE)
                } else {
                    start.saturating_sub(ARCHIVE_PAGE_SIZE)
                };
                self.selected = None;
            }
            ArchiveAction::Page(page) => self.page = page.min(self.tab.page_count() - 1),
        }
    }
}
