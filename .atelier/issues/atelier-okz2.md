---
created_at: "2026-06-15T03:54:39.166582212+00:00"
id: "atelier-okz2"
issue_type: "task"
labels:
- "cleanup"
- "cli"
- "runtime"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T15:50:56.847814562+00:00"
status: "done"
title: "Remove runtime active issue association behavior"
updated_at: "2026-06-15T15:50:56.847814562+00:00"
---

## Description

Remove runtime active issue/work association behavior from normal status and start flows. Runtime cache deletion must not erase current-work orientation, and commands should derive current work from canonical issue status plus checkout context.

## Outcome

- `atelier status` reports current work from local canonical in_progress issues rather than `.atelier/runtime/state.db` work_associations.
- `atelier start <issue-id>` remains a workflow/status operation and no longer records runtime active-work rows as the current-work source of truth.
- Runtime `sessions` and `work_associations` are not used as durable
  current-work sources; remaining internal uses are deleted or explicitly
  classified as local diagnostics only.
- Runtime rebuild may recreate cache/projection tables without preserving meaningful current-work state outside Markdown.

## Evidence

- Failing-before/passing-after CLI tests or transcripts show deleting `.atelier/runtime/state.db` and rebuilding does not lose current-work status output.
- Command transcript proves stale runtime work_associations do not affect `atelier status`.
- Focused tests for `status`, `start`, rebuild/cache deletion, and runtime association cleanup pass.
