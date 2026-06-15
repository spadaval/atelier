---
created_at: "2026-06-15T05:13:43.698977551+00:00"
id: "atelier-rjua"
issue_type: "task"
labels:
- "cleanup"
- "runtime"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove runtime active-work and claim source-of-truth from SQLite"
updated_at: "2026-06-15T05:13:43.698977551+00:00"
---

## Description

Remove hidden claim and runtime active-work database state from the rewrite path. Current work should be derived from canonical issue status and checkout context.

## Outcome

- Runtime active issue/work association is not a source of truth in the replacement SQLite schema or application flow.
- Hidden claim behavior is removed rather than shimmed.
- Status, man, help, docs, and tests teach status-derived current-work sets.
- Cache deletion or projection rebuild does not erase meaningful current-work state.

## Evidence

- Evidence record or closeout review from the absorbed active-work removal epic maps implementation proof to this issue.
- Recovery transcript proves current-work orientation survives runtime DB deletion or rebuild.
- Help and search transcripts show obsolete claim or active-pointer cleanup guidance is gone.
