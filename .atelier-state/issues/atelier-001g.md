---
acceptance: []
blocks:
- "atelier-001f"
created_at: "2026-06-09T17:30:35.790126736+00:00"
depends_on:
- "atelier-001b"
evidence_required: []
id: "atelier-001g"
issue_type: "task"
labels:
- "beads:type:task"
- "cli"
- "compatibility"
- "docs"
links: []
parent: "atelier-001a"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Document CLI surface tiers and compatibility policy"
updated_at: "2026-06-09T18:55:47.763872390+00:00"
---

Document the CLI surface tiers and compatibility policy created by the cleanup work.

## Acceptance

Docs define core commands agents should use, hidden compatibility aliases, experimental or integration commands, removed Chainlink baggage, and migration notes for old command users.
## Validation

- `cargo fmt -- --check`
- `cargo test` or a named focused substitute
- `git diff --check`
- `atelier lint`
- `atelier export --check`
- `atelier doctor`
