---
created_at: "2026-06-11T15:57:00.695787358+00:00"
id: "atelier-ncog"
issue_type: "epic"
labels:
- "assignee:root"
- "cache"
- "projection"
- "rebuild"
- "sqlite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T16:26:29.833981292+00:00"
status: "done"
title: "Add transparent stale projection rebuild"
updated_at: "2026-06-11T16:26:29.833981292+00:00"
---

## Description

Make SQLite projection staleness self-healing for ordinary read paths. Projection-backed commands should validate canonical Markdown, rebuild the local SQLite projection when safe, then answer without requiring operators to manually run `atelier rebuild`.

## Outcome

- Projection-backed read commands automatically rebuild stale SQLite when `.atelier-state/` validates.

- Tests cover changed, deleted, and newly added canonical Markdown sources before a read command.

- Invalid Markdown and record conflicts produce actionable errors and do not silently repair from bad data.

- Concurrency behavior is documented and tested for mutation/query overlap or lock contention.

- `atelier doctor` and `atelier export --check` continue to distinguish projection freshness, rebuild readiness, and runtime-state health.

- Fresh checkout, manual Markdown edit followed by query, and stale-cache scenarios are covered by integration or smoke tests.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Scope

- Replace ordinary query-path stale projection failures with safe automatic rebuild from `.atelier-state/`.
- Keep explicit failures for invalid Markdown, conflicting canonical records, missing required state, unsafe concurrent mutation windows, and rebuild errors.
- Define which commands may transparently repair and which must still fail closed.
- Preserve RuntimeState boundaries so automatic projection rebuild does not erase or reinterpret local-only state.
- Make diagnostics clear when automatic rebuild happens or cannot happen.
