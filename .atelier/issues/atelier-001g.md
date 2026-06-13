---
created_at: "2026-06-09T17:30:35.790126736+00:00"
id: "atelier-001g"
issue_type: "task"
labels:
- "beads:type:task"
- "cli"
- "compatibility"
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-09T18:55:47.763872390+00:00"
status: "done"
title: "Document CLI surface tiers and compatibility policy"
updated_at: "2026-06-09T18:55:47.763872390+00:00"
---

## Description

Document the CLI surface tiers and compatibility policy created by the cleanup work.
Docs define core commands agents should use, hidden compatibility aliases, experimental or integration commands, removed Chainlink baggage, and migration notes for old command users.
- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
