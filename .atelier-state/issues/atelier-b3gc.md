---
acceptance: []
created_at: "2026-06-11T16:14:26.191466697+00:00"
evidence_required: []
id: "atelier-b3gc"
issue_type: "feature"
labels:
- "markdown"
- "recordstore"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Move public durable mutations onto RecordStore"
updated_at: "2026-06-11T16:14:26.191466697+00:00"
---

Normal durable mutation commands are recoverable because they write .atelier-state through the compatibility export path, but most public mutations still use SQLite as the mutation engine. Move issue lifecycle, labels/dependencies, typed links, missions, plans, and evidence onto RecordStore-owned Markdown writes before projection refresh; retire export-as-normal-writer once covered.
