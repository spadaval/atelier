---
created_at: "2026-06-15T03:16:30.321491648+00:00"
id: "atelier-k95l"
issue_type: "task"
labels:
- "cli"
- "implementation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T03:26:15.833732866+00:00"
status: "done"
title: "Implement role-specific man command and remove prime"
updated_at: "2026-06-15T03:26:15.833732866+00:00"
---

## Description

Add the visible `atelier man [role]` CLI guide surface and remove
`atelier prime` without compatibility aliases.

## Outcome

- `atelier man` accepts `worker`, `reviewer`, `manager`, and `admin` only.
- `atelier man` with no role lists those roles.
- Unknown roles fail with exact valid role names.
- Worker, reviewer, and manager guides require valid tracker state; admin
  degrades gracefully before init or broken state.
- Root help shows `man` and does not show `prime`.
- Prime command code, dispatch, module wiring, docs/help references, and
  prime-focused tests are removed or rewritten.

## Evidence

- Focused CLI integration tests prove `man` output, invalid role behavior,
  pre-init behavior, and root help behavior.
- Command output: `cargo fmt -- --check`, focused `cargo test` filters,
  `atelier lint`, `atelier export --check`, and `git diff --check` pass.
