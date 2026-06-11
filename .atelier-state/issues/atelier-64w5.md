---
acceptance: []
created_at: "2026-06-11T15:56:47.184687838+00:00"
evidence_required: []
id: "atelier-64w5"
issue_type: "epic"
labels:
- "assignee:root"
- "markdown"
- "recordstore"
- "storage"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-b3gc"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Finish Markdown-first durable writes"
updated_at: "2026-06-11T16:26:56.053247071+00:00"
---

Migrate remaining durable command mutations so canonical Markdown records are written through RecordStore before SQLite projection refresh.

## Scope

- Audit durable mutation paths across issues, missions, plans, evidence, typed links, labels, dependencies, and workflow-adjacent records.
- Classify each path as Markdown-first, compatibility SQLite-first, or runtime-only.
- Move remaining non-runtime durable mutations toward RecordStore-owned Markdown writes.
- Keep projection refresh/rebuild behavior explicit after successful canonical writes.
- Preserve compatibility boundaries only where they are narrow, documented, and tracked as follow-up work.

## Acceptance Criteria

- Command audit is recorded in docs or tracker notes and covers all durable mutation commands.
- Missions, plans, evidence, typed links, labels/dependencies, and issue lifecycle gaps either write Markdown first or have explicit child issues for the remaining boundary.
- Successful durable mutations do not rely on a later export step to become recoverable from `.atelier-state/`.
- `atelier rebuild`, `atelier export --check`, `atelier lint`, `atelier doctor`, and agent-facing issue/mission workflows remain green.
- Tests cover representative Markdown-first writes for issue and first-class record mutations.
