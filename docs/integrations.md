# Integrations

## Webhooks

Webhooks are optional and must be configured explicitly in repo settings.
Recommended events: push, pull_request, issues, release.

## GitHub Apps vs Webhooks vs Actions

- **Actions**: in-repo automation.
- **Webhooks**: outbound event delivery to your service.
- **Apps**: fine-grained, install-based permissions and better long-term auth posture.
