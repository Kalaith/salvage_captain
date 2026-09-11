//! Typed scan profiles retained by an active expedition.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceScanProfile {
    Standard,
    Array,
}

impl Default for WorkspaceScanProfile {
    fn default() -> Self {
        Self::Standard
    }
}

impl WorkspaceScanProfile {
    pub const fn from_capability(has_scanner_array: bool) -> Self {
        if has_scanner_array {
            Self::Array
        } else {
            Self::Standard
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD",
            Self::Array => "ARRAY",
        }
    }

    pub const fn result_label(self) -> &'static str {
        match self {
            Self::Standard => "STANDARD SCAN",
            Self::Array => "ARRAY SCAN // DEEP RESOLVE",
        }
    }

    pub const fn pulse_count(self) -> usize {
        match self {
            Self::Standard => 1,
            Self::Array => 2,
        }
    }
}

#[cfg(test)]
mod tests;
