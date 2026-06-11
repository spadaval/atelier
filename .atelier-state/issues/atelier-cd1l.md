---
acceptance: []
created_at: "2026-06-11T20:10:54.434763072+00:00"
evidence_required: []
id: "atelier-cd1l"
issue_type: "task"
labels:
- "cache"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-22z4"
  - kind: "issue"
    id: "atelier-eprw"
  - kind: "issue"
    id: "atelier-unma"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Centralize safe auto-refresh from canonical Markdown"
updated_at: "2026-06-11T20:10:54.434763072+00:00"
---

Implement one freshness policy that checks source hash/mtime/size metadata and refreshes the ProjectionIndex from Markdown when safe. Acceptance: query commands do not use stale SQLite when canonical Markdown changed.
