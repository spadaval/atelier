---
acceptance: []
created_at: "2026-06-11T18:22:54.885691411+00:00"
evidence_required: []
id: "atelier-ybt6"
issue_type: "epic"
labels:
- "assignee:root"
- "storage"
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
title: "Validate RecordStore-owned mutation cutover"
updated_at: "2026-06-11T18:59:47.721995696+00:00"
---

Close out the RecordStore-owned mutation cutover with scenario-centered proof. Scope includes residue searches for export-as-writer and SQLite-first durable paths, fresh checkout rebuild, manual Markdown edit followed by query, invalid Markdown failure behavior, concurrent mutation/query behavior, mission/issue Agent Factory workflows, export freshness, lint, doctor, cargo nextest, cargo test, and extended ignored/property tests. Acceptance: all mission validation criteria are either proven by linked evidence or explicitly deferred with a follow-up; mission can close with no linked ready/blocking work.
