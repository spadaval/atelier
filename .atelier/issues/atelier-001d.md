---
created_at: "2026-06-09T17:30:35.641914916+00:00"
id: "atelier-001d"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:feature"
- "cli"
- "domain-model"
- "links"
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
status: "closed"
title: "Replace assumption-specific relation commands with generic link impact"
updated_at: "2026-06-10T02:44:12.362979445+00:00"
---

## Description

Replace assumption-specific relation commands with generic typed-link impact concepts. `cascade` and `falsify` should be removed, hidden, or superseded by typed-link impact commands once `atelier link` exists.

## Outcome

`cascade` and `falsify` are no longer the preferred public relation workflow; typed-link impact behavior covers the useful capability; docs explain migration from assumption/falsification-specific commands.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
