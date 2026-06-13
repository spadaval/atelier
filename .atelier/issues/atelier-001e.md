---
created_at: "2026-06-09T17:30:35.664345209+00:00"
id: "atelier-001e"
issue_type: "task"
labels:
- "beads:type:task"
- "cleanup"
- "cli"
- "issue"
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
closed_at: "2026-06-09T18:55:47.741705917+00:00"
status: "done"
title: "Remove changelog behavior from issue closure"
updated_at: "2026-06-09T18:55:47.741705917+00:00"
---

## Description

Decouple work-item closure from product changelog mutation. `issue close` should represent tracker state only; release-note or changelog generation should be a separate explicit workflow if retained later.

## Outcome

`issue close` no longer exposes or depends on changelog behavior. Existing tests and docs are updated so closing work records tracker closure reason only.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`
