---
created_at: "2026-06-09T17:30:35.769115263+00:00"
id: "atelier-001f"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:validation"
- "cli"
- "validation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000c"
  - kind: "issue"
    id: "atelier-000j"
  - kind: "issue"
    id: "atelier-000m"
  - kind: "issue"
    id: "atelier-000r"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate streamlined CLI surface"
updated_at: "2026-06-10T02:51:27.254204467+00:00"
---

## Description

Validate the streamlined public CLI surface after cleanup implementation lands.

## Outcome

Help snapshots, CLI tests, or command transcript evidence prove the intended public surface. `cargo test`, `atelier lint`, `atelier export --check`, and `atelier doctor` are recorded in notes or linked evidence.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
