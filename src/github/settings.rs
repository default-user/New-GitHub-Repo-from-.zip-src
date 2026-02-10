use crate::error::Zip2RepoError;
use crate::github::models::SettingsResult;
use octocrab::Octocrab;
use serde_json::json;

/// Apply branch protection and repo settings, then verify via readback.
/// Returns a SettingsResult indicating what was applied and whether verification passed.
pub async fn apply_and_verify(
    gh: &Octocrab,
    owner: &str,
    repo: &str,
    check_names: &[&str],
) -> Result<SettingsResult, Zip2RepoError> {
    // Build the branch protection payload
    let payload = json!({
        "required_status_checks": {
            "strict": true,
            "contexts": check_names,
        },
        "enforce_admins": false,
        "required_pull_request_reviews": {
            "dismiss_stale_reviews": true,
            "required_approving_review_count": 1,
        },
        "restrictions": null,
        "allow_force_pushes": false,
    });

    // Apply branch protection on main
    let route = format!("/repos/{owner}/{repo}/branches/main/protection");
    let apply_result: Result<serde_json::Value, _> = gh
        .put(&route, Some(&payload))
        .await;

    let branch_protection_applied = match apply_result {
        Ok(_) => true,
        Err(e) => {
            tracing::warn!("Failed to apply branch protection: {e}");
            false
        }
    };

    // Readback verification
    let verified = if branch_protection_applied {
        let readback: Result<serde_json::Value, _> = gh
            .get(&route, None::<&()>)
            .await;
        match readback {
            Ok(val) => {
                // Verify key fields exist
                val.get("required_pull_request_reviews").is_some()
                    && val.get("required_status_checks").is_some()
            }
            Err(e) => {
                tracing::warn!("Settings readback failed: {e}");
                false
            }
        }
    } else {
        false
    };

    Ok(SettingsResult {
        branch_protection_applied,
        status_checks_applied: branch_protection_applied,
        verified,
    })
}
