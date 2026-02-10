use crate::cli::Stack;
use crate::error::Zip2RepoError;
use crate::scaffold::templates as t;
use std::{fs, path::Path};

pub fn ensure_file(path: &Path, contents: &str) -> Result<(), Zip2RepoError> {
    if path.exists() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Zip2RepoError::Io(e.to_string()))?;
    }
    fs::write(path, contents).map_err(|e| Zip2RepoError::Io(e.to_string()))
}

/// Select the appropriate CI workflow template based on detected stack.
fn ci_yml_for_stack(stack: &Stack) -> &'static str {
    match stack {
        Stack::Rust => t::CI_YML_RUST,
        Stack::Node => t::CI_YML_NODE,
        Stack::Python => t::CI_YML_PYTHON,
        Stack::Go => t::CI_YML_GO,
        _ => t::CI_YML_GENERIC,
    }
}

/// Build the dependabot.yml content with ecosystem-specific entries.
fn dependabot_yml_for_stack(stack: &Stack) -> String {
    let mut content = t::DEPENDABOT_YML_HEADER.to_string();
    match stack {
        Stack::Rust => content.push_str(t::DEPENDABOT_CARGO),
        Stack::Node => content.push_str(t::DEPENDABOT_NPM),
        Stack::Python => content.push_str(t::DEPENDABOT_PIP),
        Stack::Go => content.push_str(t::DEPENDABOT_GOMOD),
        Stack::Multi => {
            content.push_str(t::DEPENDABOT_CARGO);
            content.push_str(t::DEPENDABOT_NPM);
            content.push_str(t::DEPENDABOT_PIP);
            content.push_str(t::DEPENDABOT_GOMOD);
        }
        _ => {}
    }
    content
}

/// Write the minimum scaffold into the repo root. Idempotent: will not overwrite existing files.
pub fn scaffold_minimum(repo_root: &Path, stack: &Stack) -> Result<(), Zip2RepoError> {
    ensure_file(&repo_root.join("README.md"), t::README_MD)?;
    ensure_file(&repo_root.join("SECURITY.md"), t::SECURITY_MD)?;
    ensure_file(&repo_root.join("CONTRIBUTING.md"), t::CONTRIBUTING_MD)?;
    ensure_file(&repo_root.join("CODE_OF_CONDUCT.md"), t::CODE_OF_CONDUCT_MD)?;

    ensure_file(&repo_root.join("docs/integrations.md"), t::INTEGRATIONS_MD)?;
    ensure_file(
        &repo_root.join("docs/repo-settings-checklist.md"),
        t::SETTINGS_CHECKLIST_MD,
    )?;

    ensure_file(
        &repo_root.join(".github/PULL_REQUEST_TEMPLATE.md"),
        t::PR_TEMPLATE,
    )?;
    ensure_file(
        &repo_root.join(".github/ISSUE_TEMPLATE/bug_report.yml"),
        t::BUG_TEMPLATE,
    )?;
    ensure_file(
        &repo_root.join(".github/ISSUE_TEMPLATE/feature_request.yml"),
        t::FEATURE_TEMPLATE,
    )?;
    ensure_file(
        &repo_root.join(".github/ISSUE_TEMPLATE/config.yml"),
        t::ISSUE_CONFIG,
    )?;

    ensure_file(
        &repo_root.join(".github/workflows/ci.yml"),
        ci_yml_for_stack(stack),
    )?;
    ensure_file(
        &repo_root.join(".github/workflows/security-codeql.yml"),
        t::SECURITY_CODEQL_YML,
    )?;

    let dependabot = dependabot_yml_for_stack(stack);
    ensure_file(&repo_root.join(".github/dependabot.yml"), &dependabot)?;

    Ok(())
}
