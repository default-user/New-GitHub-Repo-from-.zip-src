use std::fs;
use tempfile::TempDir;
use zip2repo::cli::Stack;
use zip2repo::scaffold::write::scaffold_minimum;

#[test]
fn scaffold_creates_expected_files() {
    let td = TempDir::new().unwrap();
    let root = td.path();

    scaffold_minimum(root, &Stack::Other).unwrap();

    assert!(root.join("README.md").exists());
    assert!(root.join("SECURITY.md").exists());
    assert!(root.join("CONTRIBUTING.md").exists());
    assert!(root.join("CODE_OF_CONDUCT.md").exists());
    assert!(root.join("docs/integrations.md").exists());
    assert!(root.join("docs/repo-settings-checklist.md").exists());
    assert!(root.join(".github/PULL_REQUEST_TEMPLATE.md").exists());
    assert!(root.join(".github/ISSUE_TEMPLATE/bug_report.yml").exists());
    assert!(root
        .join(".github/ISSUE_TEMPLATE/feature_request.yml")
        .exists());
    assert!(root.join(".github/ISSUE_TEMPLATE/config.yml").exists());
    assert!(root.join(".github/workflows/ci.yml").exists());
    assert!(root.join(".github/workflows/security-codeql.yml").exists());
    assert!(root.join(".github/dependabot.yml").exists());
}

#[test]
fn scaffold_is_idempotent() {
    let td = TempDir::new().unwrap();
    let root = td.path();

    scaffold_minimum(root, &Stack::Rust).unwrap();

    // Read content of a file
    let content_before = fs::read_to_string(root.join("README.md")).unwrap();

    // Run scaffold again
    scaffold_minimum(root, &Stack::Rust).unwrap();

    // Content should be identical (not duplicated)
    let content_after = fs::read_to_string(root.join("README.md")).unwrap();
    assert_eq!(content_before, content_after);
}

#[test]
fn scaffold_does_not_overwrite_existing() {
    let td = TempDir::new().unwrap();
    let root = td.path();

    // Create a README before scaffolding
    fs::write(root.join("README.md"), "My custom readme").unwrap();

    scaffold_minimum(root, &Stack::Other).unwrap();

    // The original content should be preserved
    let content = fs::read_to_string(root.join("README.md")).unwrap();
    assert_eq!(content, "My custom readme");
}

#[test]
fn scaffold_rust_stack_uses_cargo_dependabot() {
    let td = TempDir::new().unwrap();
    let root = td.path();

    scaffold_minimum(root, &Stack::Rust).unwrap();

    let dependabot = fs::read_to_string(root.join(".github/dependabot.yml")).unwrap();
    assert!(dependabot.contains("cargo"));
    assert!(dependabot.contains("github-actions"));
}

#[test]
fn scaffold_node_stack_uses_npm_dependabot() {
    let td = TempDir::new().unwrap();
    let root = td.path();

    scaffold_minimum(root, &Stack::Node).unwrap();

    let dependabot = fs::read_to_string(root.join(".github/dependabot.yml")).unwrap();
    assert!(dependabot.contains("npm"));
    assert!(dependabot.contains("github-actions"));
}
