---
acceptance: []
created_at: "2026-06-11T20:10:53.196497184+00:00"
evidence_required: []
id: "atelier-kxko"
issue_type: "task"
labels:
- "config"
- "gitignore"
- "init"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Update init and gitignore behavior for tracked .atelier records"
updated_at: "2026-06-11T20:10:53.196497184+00:00"
---

Change init/export scaffolding so .atelier/ canonical records and project config are tracked while runtime/cache subpaths are ignored. Acceptance: fresh init and migrated repos do not ignore all of .atelier/ and do not commit state.db or runtime diagnostics.
