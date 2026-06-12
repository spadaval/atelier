---
created_at: "2026-06-09T19:46:18.103412445+00:00"
id: "atelier-001m"
issue_type: "task"
labels:
- "diagnostics"
- "feature"
- "mission-control"
- "performance"
- "telemetry"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-001p"
  - kind: "issue"
    id: "atelier-001q"
  - kind: "issue"
    id: "atelier-001r"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add local command telemetry and performance diagnostics"
updated_at: "2026-06-11T13:15:09.034591115+00:00"
---

## Description

Record command interactions and timing in a global, local-only Atelier diagnostics store so slow commands can be identified without making run/session records part of committed durable state.

## Outcome

Telemetry is captured for mutating and read-only commands, can be disabled, survives across workspaces in a global local folder, has stable JSON query output, documents privacy/redaction behavior, and does not make `.atelier-state/` nondeterministic. Tests cover successful commands, failed commands, disabled telemetry, redaction, and slow-command queries.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`

## Notes

### Scope

- Define a global diagnostics directory, expected to be outside `.atelier-state/`, with documented override and disable controls.
- Record command name, started_at, finished_at, duration, exit status, workspace identity, agent identity when known, and structured phase timings where available.
- Redact or avoid sensitive command arguments by default; provide an explicit verbose mode for local debugging.
- Add `atelier diagnostics slow` or equivalent command JSON that summarizes slow commands by workspace, command, and time window.
- Keep the data model compatible with future run/session metadata without requiring export of raw interaction logs.
