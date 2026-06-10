---
acceptance: []
blocks:
- "atelier-000f"
- "atelier-001c"
- "atelier-001d"
- "atelier-001e"
- "atelier-001g"
created_at: "2026-06-09T17:30:35.587295808+00:00"
depends_on: []
evidence_required: []
id: "atelier-001b"
issue_type: "task"
labels:
- "audit"
- "beads:type:task"
- "cli"
- "docs"
links: []
parent: "atelier-001a"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Classify current CLI commands into surface tiers"
updated_at: "2026-06-09T18:55:47.712538891+00:00"
---

Classify the current CLI surface into core, compatibility, integration, and removal buckets. Cover `timer`, `usage`, `cpitd`, `daemon`, `archive`, `cascade`, `falsify`, backup import/export, changelog-on-close, flat aliases, locks, sessions, and milestones.

## Acceptance

A durable doc or tracker note records the final disposition for each command family, names the intended replacement where one exists, and identifies any compatibility period or migration note required before implementation.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
