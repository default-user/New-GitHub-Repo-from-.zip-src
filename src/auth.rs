use crate::cli::AuthMode;
use crate::error::Zip2RepoError;
use std::process::Command;

pub fn acquire_token(mode: AuthMode) -> Result<String, Zip2RepoError> {
    match mode {
        AuthMode::Gh => token_from_gh(),
        AuthMode::Pat => token_from_env(),
        AuthMode::App => Err(Zip2RepoError::Usage(
            "GitHub App auth not implemented in v0.1".into(),
        )),
        AuthMode::Auto => token_from_gh().or_else(|_| token_from_env()),
    }
}

fn token_from_env() -> Result<String, Zip2RepoError> {
    let t = std::env::var("GITHUB_TOKEN").map_err(|_| {
        Zip2RepoError::Usage(
            "Set GITHUB_TOKEN (fine-grained PAT) or use --auth gh".into(),
        )
    })?;
    if t.trim().is_empty() {
        return Err(Zip2RepoError::Usage("GITHUB_TOKEN is empty".into()));
    }
    Ok(t)
}

fn token_from_gh() -> Result<String, Zip2RepoError> {
    let out = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|e| Zip2RepoError::Usage(format!("Failed to run gh: {e}")))?;
    if !out.status.success() {
        return Err(Zip2RepoError::Usage(
            "gh auth token failed; run `gh auth login` or use GITHUB_TOKEN".into(),
        ));
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        return Err(Zip2RepoError::Usage("gh returned empty token".into()));
    }
    Ok(s)
}
