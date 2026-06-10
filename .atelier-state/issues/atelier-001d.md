---
acceptance: []
blocks:
- "atelier-001f"
created_at: "2026-06-09T17:30:35.641914916+00:00"
depends_on:
- "atelier-001b"
evidence_required: []
id: "atelier-001d"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:feature"
- "cli"
- "domain-model"
- "links"
links: []
parent: "atelier-001a"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Replace assumption-specific relation commands with generic link impact"
updated_at: "2026-06-10T02:44:12.362979445+00:00"
---

Replace assumption-specific relation commands with generic typed-link impact concepts. `cascade` and `falsify` should be removed, hidden, or superseded by typed-link impact commands once `atelier link` exists.

## Acceptance

`cascade` and `falsify` are no longer the preferred public relation workflow; typed-link impact behavior covers the useful capability; docs explain migration from assumption/falsification-specific commands.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
