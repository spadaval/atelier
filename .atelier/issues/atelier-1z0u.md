---
created_at: "2026-06-10T16:00:59.288881607+00:00"
id: "atelier-1z0u"
issue_type: "task"
labels:
- "activity"
- "cli"
- "history"
- "json"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-krhk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:35:56.806271575+00:00"
status: "done"
title: "Add atelier history command"
updated_at: "2026-06-10T17:35:56.806271575+00:00"
---

## Description

Add the central history query surface.

What:
- Add `atelier history [--issue <id>] [--since <date-or-rfc3339>] [--until <date-or-rfc3339>] [--type <event_type>] [--limit <n>] [--json]`.
- Default to newest-first recent global history across issue activity folders.
- Default limit is 50.
- `--issue` narrows to a single issue activity folder.
- Render `evidence_attached` references inline with enough information to follow up via `atelier evidence show`.
- JSON output must be stable enough for downstream projection and validation tests.

Out of scope:
- Non-issue history domains.
- Migrating legacy comments.

## Outcome

- Global and issue-scoped history return newest-first streams.
- `--type`, `--since`, `--until`, `--limit`, and `--json` behave as documented.
- Human output is readable and includes event type, timestamp, issue, actor/summary, body/detail, and evidence references where applicable.
- Tests cover filtering, ordering, limits, JSON shape, invalid arguments, and empty history.

Recommended subskill: agent-factory implement.

## Evidence

Evidence was not specified in the legacy issue record.
