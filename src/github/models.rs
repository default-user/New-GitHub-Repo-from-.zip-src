use serde::{Deserialize, Serialize};

/// Represents the result of a settings apply operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResult {
    pub branch_protection_applied: bool,
    pub status_checks_applied: bool,
    pub verified: bool,
}
