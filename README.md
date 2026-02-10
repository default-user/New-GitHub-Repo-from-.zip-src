# zip2repo

Turn a source `.zip` into a GitHub repository wired to GitHub-native scaffolds (Actions CI, security docs, templates, Dependabot, checklists).

## Quickstart

```bash
zip2repo ./source.zip --owner YOUR_OWNER --repo YOUR_REPO --private
# or
zip2repo https://example.com/source.zip --owner YOUR_OWNER --repo YOUR_REPO --public
```

## What it does

1. Validates and safely extracts the zip (fail-closed).
2. Auto-detects the project stack (Rust, Node, Python, Go, or generic).
3. Adds repo scaffolding (`.github` workflows, templates, docs).
4. Creates the repo on GitHub (only if you have permission).
5. Pushes the initial commit.
6. Optionally applies repo settings (branch protection, status checks) with verification.

## CLI

```
zip2repo <zip_path_or_https_url> --owner <gh_owner> --repo <repo_name> [flags]
```

### Required flags

| Flag | Description |
|------|-------------|
| `--owner` | GitHub owner (user or org) |
| `--repo` | Repository name |
| `--private` or `--public` | Visibility (exactly one required) |

### Optional flags

| Flag | Default | Description |
|------|---------|-------------|
| `--apply-settings` | `false` | Apply branch protection and verify |
| `--dry-run` | `false` | Do everything locally, print plan, no GitHub mutation |
| `--stack` | `auto` | Stack: `auto`, `rust`, `node`, `python`, `go`, `multi`, `other` |
| `--auth` | `auto` | Auth mode: `auto` (gh then PAT), `gh`, `pat`, `app` |
| `--verbose` | `false` | Enable debug logging |

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Full success (settings applied and verified) |
| 10 | Artifacts only (repo created/pushed; settings pending) |
| 20 | Permission denied |
| 21 | GitHub API error |
| 22 | Git push error |
| 30 | Zip validation failed |
| 31 | Network fetch failed |
| 32 | IO error |
| 40 | Config/usage error |

## Authentication

Token resolution order (with `--auth auto`):

1. `gh auth token` (GitHub CLI)
2. `GITHUB_TOKEN` environment variable (fine-grained PAT)

GitHub App auth (`--auth app`) is stubbed for future implementation.

## Safety

- All zip entries are validated against path traversal, absolute paths, and symlinks.
- Total uncompressed size capped at 512 MB; file count capped at 50,000.
- zip2repo only operates within the permissions you already have. No escalation, ever.

## License

MIT
