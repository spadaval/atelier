---
created_at: "2026-06-10T16:00:59.368552201+00:00"
id: "atelier-krhk"
issue_type: "validation"
labels:
- "activity"
- "closeout"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate issue activity history end to end"
updated_at: "2026-06-10T17:43:46.476755383+00:00"
---

## Description

Validate issue-centered activity history end to end.

What:
- Exercise comments/notes, close/reopen, start/finish work, and evidence attachment.
- Verify `atelier history` shows a unified newest-first stream.
- Verify `atelier history --issue <id>`, `--type`, `--since`, `--until`, `--limit`, and `--json`.
- Delete `.atelier/state.db`, run `atelier rebuild`, and confirm activity survives and history output is unchanged.
- Run the migration script on a temp repo with SQLite comments and close reasons; verify generated sidecars and no duplicate output on a second run.
- Run the agreed validation commands.

## Outcome

- `cargo fmt -- --check` passes.
- Focused history/activity integration tests pass.
- `cargo test` passes.
- `atelier export --check` passes.
- `atelier lint` passes.
- `atelier doctor` passes.
- Any residual risk or intentionally deferred non-issue history support is documented in the issue close reason.

Recommended subskill: agent-factory validate.

## Evidence

Evidence was not specified in the legacy issue record.
