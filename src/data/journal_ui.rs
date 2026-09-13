//! Authored journal labels grouped to match bounded, readable pages.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JournalUiCopy {
    pub title: String,
    pub subtitle: String,
    pub tabs: [String; 4],
    pub filters: [String; 4],
    pub voyage_pages: [String; 2],
    pub career_pages: [String; 2],
    pub ledger_pages: [String; 3],
    pub hints: [String; 4],
    pub close: String,
    pub newer: String,
    pub older: String,
    pub run: String,
    pub page: String,
    pub of: String,
    pub empty_title: String,
    pub empty_hint: String,
    pub filtered_title: String,
    pub filtered_hint: String,
    pub recovery: [String; 12],
    pub preparation: [String; 8],
    pub career: [String; 12],
    pub archive: [String; 12],
    pub service: [String; 9],
    pub supplies: [String; 6],
    pub income: [String; 6],
    pub contracts: [String; 4],
    pub coverage: [String; 2],
    pub sections: String,
    pub none: String,
    pub earned: String,
    pub pending: String,
    pub award_requirements: [String; 7],
}
