use crate::cli::Stack;
use std::path::Path;

/// Auto-detect the project stack from the extracted source tree.
/// Returns the detected stack, or `Stack::Other` if nothing recognized.
pub fn detect_stack(root: &Path) -> Stack {
    if root.join("Cargo.toml").exists() {
        return Stack::Rust;
    }
    if root.join("package.json").exists() {
        return Stack::Node;
    }
    if root.join("pyproject.toml").exists()
        || root.join("setup.py").exists()
        || root.join("requirements.txt").exists()
    {
        return Stack::Python;
    }
    if root.join("go.mod").exists() {
        return Stack::Go;
    }
    Stack::Other
}

/// Resolve the effective stack: if `Auto`, run detection; otherwise use the explicit choice.
pub fn resolve_stack(requested: &Stack, root: &Path) -> Stack {
    match requested {
        Stack::Auto => detect_stack(root),
        other => other.clone(),
    }
}
