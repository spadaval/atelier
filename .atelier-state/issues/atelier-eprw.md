---
acceptance: []
created_at: "2026-06-11T20:10:54.799132612+00:00"
evidence_required: []
id: "atelier-eprw"
issue_type: "task"
labels:
- "errors"
- "lint"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g9fd"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Fail projection queries clearly when canonical Markdown is invalid"
updated_at: "2026-06-11T20:10:54.799132612+00:00"
---

Ensure invalid canonical Markdown blocks query refresh and produces actionable guidance to run atelier lint, without falling back to stale projection rows. Acceptance: tests cover invalid records with existing stale state.db.
