//! Integration suite for authored data loading and semantic validation.

pub mod data {
    pub use salvage_captain::data::*;
}

pub use data::*;

#[path = "data/content.rs"]
mod content_tests;
