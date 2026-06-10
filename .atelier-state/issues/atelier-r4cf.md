---
acceptance: []
blocks: []
created_at: "2026-06-10T16:00:00.692912198+00:00"
depends_on: []
evidence_required: []
id: "atelier-r4cf"
issue_type: "epic"
labels:
- "activity"
- "assignee:root"
- "epic"
- "history"
- "migration"
- "record-store"
links: []
parent: "atelier-zd4d"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Epic: Issue-centered activity history"
updated_at: "2026-06-10T17:43:57.667530036+00:00"
---

Create the canonical issue-centered activity history system from the supplied plan.

Scope:
- Activity is V1 issue-first and stored in per-issue sidecar folders under `.atelier-state/issues/<issue-id>.activity/`.
- Activity entries use `schema: atelier.activity`, `schema_version: 1`, typed metadata, timestamp-based IDs, and markdown bodies.
- `atelier history` provides newest-first global and issue-scoped views with filters and JSON output.
- Evidence remains a rich record/artifact concept; issue history stores only lightweight `evidence_attached` references.
- Export/rebuild treats activity sidecars as canonical state once present.
- Existing SQLite comments and close reasons are migrated by one explicit script, not a normal command.

Out of scope:
- Mission, plan, or non-issue activity folders.
- Replacing rich evidence records with activity entries.
- Implementing the work in this planning pass.

Acceptance criteria:
- Child issues cover schema/storage, issue mutation writes, evidence references, history CLI, rebuild/export projection, one-off migration, docs, and end-to-end validation.
- Dependencies keep validation and UI/query work behind the canonical activity format and persistence work.
- Future agents can execute each child without needing the original chat plan.
