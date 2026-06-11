---
acceptance: []
created_at: "2026-06-11T20:10:53.593745161+00:00"
evidence_required: []
id: "atelier-ru15"
issue_type: "task"
labels:
- "compatibility"
- "migration"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Keep legacy .atelier-state discovery read-only during the migration window"
updated_at: "2026-06-11T20:10:53.593745161+00:00"
---

Support old .atelier-state discovery only for read/migrate flows during the explicit compatibility window. Acceptance: no command silently writes durable records back to .atelier-state once markdown-first migration is available.
