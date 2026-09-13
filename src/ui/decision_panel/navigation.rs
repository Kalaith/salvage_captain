//! Bounded pagination for cargo lists, including after resolving the last row.

pub const PAGE_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ManifestPage {
    pub index: usize,
}

impl ManifestPage {
    pub fn page_count(count: usize) -> usize {
        count.div_ceil(PAGE_SIZE).max(1)
    }

    pub fn current(self, count: usize) -> usize {
        self.index.min(Self::page_count(count) - 1)
    }

    pub fn turn(&mut self, next: bool, count: usize) {
        let current = self.current(count);
        self.index = if next {
            (current + 1).min(Self::page_count(count) - 1)
        } else {
            current.saturating_sub(1)
        };
    }
}
