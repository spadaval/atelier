---
created_at: "2026-06-09T17:30:35.587295808+00:00"
id: "atelier-001b"
issue_type: "task"
labels:
- "audit"
- "beads:type:task"
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000f"
  - kind: "issue"
    id: "atelier-001c"
  - kind: "issue"
    id: "atelier-001d"
  - kind: "issue"
    id: "atelier-001e"
  - kind: "issue"
    id: "atelier-001g"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Classify current CLI commands into surface tiers"
updated_at: "2026-06-09T18:55:47.712538891+00:00"
---

## Description

Classify the current CLI surface into core, compatibility, integration, and removal buckets. Cover `timer`, `usage`, `cpitd`, `daemon`, `archive`, `cascade`, `falsify`, backup import/export, changelog-on-close, flat aliases, locks, sessions, and milestones.

## Outcome

A durable doc or tracker note records the final disposition for each command family, names the intended replacement where one exists, and identifies any compatibility period or migration note required before implementation.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
