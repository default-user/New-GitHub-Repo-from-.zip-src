pub const README_MD: &str = r#"# zip2repo

Turn a source .zip into a GitHub repository wired to GitHub-native scaffolds (Actions CI, security docs, templates, Dependabot, checklists).

## Quickstart

```bash
zip2repo ./source.zip --owner YOUR_OWNER --repo YOUR_REPO --private
# or
zip2repo https://example.com/source.zip --owner YOUR_OWNER --repo YOUR_REPO --public
```

## What it does

- Validates and safely extracts the zip (fail-closed).
- Adds repo scaffolding (.github workflows, templates, docs).
- Creates the repo (only if you have permission).
- Pushes initial commit.
- Optionally applies repo settings with verification.

## Safety & authority

zip2repo only operates within the permissions you already have. No escalation, ever.
"#;

pub const CI_YML_RUST: &str = r#"name: CI

on:
  pull_request:
  push:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --release

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Test
        run: cargo test

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - name: Clippy
        run: cargo clippy -- -D warnings
      - name: Fmt check
        run: cargo fmt --check
"#;

pub const CI_YML_NODE: &str = r#"name: CI

on:
  pull_request:
  push:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install
        run: npm ci
      - name: Build
        run: npm run build --if-present

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install
        run: npm ci
      - name: Test
        run: npm test

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install
        run: npm ci
      - name: Lint
        run: npm run lint --if-present
"#;

pub const CI_YML_PYTHON: &str = r#"name: CI

on:
  pull_request:
  push:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      - name: Install
        run: pip install -e ".[dev]" || pip install -r requirements.txt

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      - name: Install
        run: pip install -e ".[dev]" || pip install -r requirements.txt
      - name: Test
        run: pytest

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      - name: Install
        run: pip install ruff
      - name: Lint
        run: ruff check .
"#;

pub const CI_YML_GO: &str = r#"name: CI

on:
  pull_request:
  push:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      - name: Build
        run: go build ./...

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      - name: Test
        run: go test ./...

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      - name: Vet
        run: go vet ./...
"#;

pub const CI_YML_GENERIC: &str = r#"name: CI

on:
  pull_request:
  push:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: echo "TODO: build steps (auto-detected by zip2repo if stack known)"

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - name: Test
        run: echo "TODO: test steps (auto-detected by zip2repo if stack known)"
"#;

pub const SECURITY_CODEQL_YML: &str = r#"name: "CodeQL"

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
  schedule:
    - cron: '0 6 * * 1'

jobs:
  analyze:
    name: Analyze
    runs-on: ubuntu-latest
    permissions:
      actions: read
      contents: read
      security-events: write
    strategy:
      fail-fast: false
      matrix:
        # CodeQL supports: cpp, csharp, go, java, javascript, python, ruby, swift
        # Adjust to your repo's languages
        language: [ 'javascript' ]
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
      - name: Initialize CodeQL
        uses: github/codeql-action/init@v3
        with:
          languages: ${{ matrix.language }}
      - name: Autobuild
        uses: github/codeql-action/autobuild@v3
      - name: Perform CodeQL Analysis
        uses: github/codeql-action/analyze@v3
        with:
          category: "/language:${{ matrix.language }}"
"#;

pub const DEPENDABOT_YML_HEADER: &str = "version: 2\nupdates:\n  - package-ecosystem: \"github-actions\"\n    directory: \"/\"\n    schedule:\n      interval: \"weekly\"\n";

pub const DEPENDABOT_CARGO: &str = "  - package-ecosystem: \"cargo\"\n    directory: \"/\"\n    schedule:\n      interval: \"weekly\"\n";

pub const DEPENDABOT_NPM: &str = "  - package-ecosystem: \"npm\"\n    directory: \"/\"\n    schedule:\n      interval: \"weekly\"\n";

pub const DEPENDABOT_PIP: &str = "  - package-ecosystem: \"pip\"\n    directory: \"/\"\n    schedule:\n      interval: \"weekly\"\n";

pub const DEPENDABOT_GOMOD: &str = "  - package-ecosystem: \"gomod\"\n    directory: \"/\"\n    schedule:\n      interval: \"weekly\"\n";

pub const PR_TEMPLATE: &str = r#"## What changed

## Checklist
- [ ] Tests added/updated (or not applicable)
- [ ] Docs updated (if needed)
- [ ] Security impact considered
"#;

pub const BUG_TEMPLATE: &str = r#"name: Bug report
description: Report a bug
title: "[bug] "
labels: ["bug"]
body:
  - type: textarea
    id: what
    attributes:
      label: What happened?
    validations:
      required: true
  - type: textarea
    id: expected
    attributes:
      label: What did you expect?
    validations:
      required: true
"#;

pub const FEATURE_TEMPLATE: &str = r#"name: Feature request
description: Suggest an idea
title: "[feat] "
labels: ["enhancement"]
body:
  - type: textarea
    id: why
    attributes:
      label: Why?
    validations:
      required: true
  - type: textarea
    id: what
    attributes:
      label: What should it do?
    validations:
      required: true
"#;

pub const ISSUE_CONFIG: &str = r#"blank_issues_enabled: false
contact_links:
  - name: Security issues
    url: ./SECURITY.md
    about: Please follow the security policy for disclosures
"#;

pub const SECURITY_MD: &str = r#"# Security Policy

## Reporting a Vulnerability

Please do not open public issues for security reports.
Instead, contact the maintainers via the channel described in this repository, or use GitHub Security Advisories if enabled.
"#;

pub const CONTRIBUTING_MD: &str = r#"# Contributing

- Keep changes small and reviewable.
- Prefer explicit, testable behavior.
- No secrets in commits.
"#;

pub const CODE_OF_CONDUCT_MD: &str = r#"# Code of Conduct

This project uses the Contributor Covenant Code of Conduct.
"#;

pub const INTEGRATIONS_MD: &str = r#"# Integrations

## Webhooks

Webhooks are optional and must be configured explicitly in repo settings.
Recommended events: push, pull_request, issues, release.

## GitHub Apps vs Webhooks vs Actions

- **Actions**: in-repo automation.
- **Webhooks**: outbound event delivery to your service.
- **Apps**: fine-grained, install-based permissions and better long-term auth posture.
"#;

pub const SETTINGS_CHECKLIST_MD: &str = r#"# Repo settings checklist

## Branch protection (main)

- [ ] Require PR reviews: 1
- [ ] Require status checks:
  - [ ] build
  - [ ] test
  - [ ] lint (if present)
- [ ] Disable force-push

## Security

- [ ] Enable Dependabot alerts (if available)
- [ ] Enable Code scanning alerts (if available)
- [ ] Enable Secret scanning (if available)

## Optional

- [ ] Discussions
- [ ] Projects
- [ ] Pages (if using docs site)

zip2repo will only apply settings when `--apply-settings` is used and permissions allow, and will verify readback.
"#;
