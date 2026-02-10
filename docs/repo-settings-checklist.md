# Repo settings checklist

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
