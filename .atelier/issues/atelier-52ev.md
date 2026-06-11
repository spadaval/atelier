---
acceptance: []
created_at: "2026-06-11T16:16:00.177356290+00:00"
evidence_required: []
id: "atelier-52ev"
issue_type: "task"
labels:
- "markdown"
- "projection"
- "search"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Retire issue description and legacy comment text from SQLite search projection"
updated_at: "2026-06-11T16:26:29.793290391+00:00"
---

Replace issue search's dependence on issues.description and comments.content with a metadata/search index that is rebuilt from RecordStore and activity sidecars. Keep issue list and ready queries on small metadata rows, and load Markdown only for selected detail results.
