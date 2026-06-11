---
acceptance: []
created_at: "2026-06-09T19:47:26.315366600+00:00"
evidence_required: []
id: "atelier-001u"
issue_type: "task"
labels:
- "bulk"
- "issue"
- "json"
- "task"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-001t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement atomic issue graph bulk apply"
updated_at: "2026-06-10T15:05:38.571560376+00:00"
---

Implement the first bulk apply slice for issue records: create multiple issues with parent-child hierarchy, dependencies, labels, priorities, issue types, descriptions, acceptance criteria, and notes from one JSON file. This slice must use the project-scoped random record ID allocator and must not create numeric or typed-prefix issue IDs.

Acceptance: apply is atomic where practical, emits a client_ref-to-ID mapping using project-scoped random IDs, preserves deterministic export/rebuild, and includes rollback or recovery tests for failed validation.
