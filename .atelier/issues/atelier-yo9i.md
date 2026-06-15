---
created_at: "2026-06-15T15:16:51.648772613+00:00"
id: "atelier-yo9i"
issue_type: "task"
labels:
- "activity"
- "deletion"
- "sqlite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Delete SQLite comments sessions and work-association schema"
updated_at: "2026-06-15T15:16:51.648772613+00:00"
---

## Description

Remove inherited SQLite comments, sessions, and work_associations schema and migration behavior. Beads/import preservation comments and close reasons should persist as canonical activity sidecars, not SQLite-only rows.

## Outcome

- SQLite `comments`, `sessions`, and `work_associations` schema, migrations, and query paths are removed.
- Beads/import preservation comments and close reasons are stored as canonical activity sidecars, not SQLite-only rows.
- Status/worktree orientation no longer depends on runtime association rows after rebuild or cache deletion.

## Evidence

- File-change review artifact proves the removed SQLite tables, migrations, and callers are gone.
- Import/close scenario transcript proves preservation comments and close reasons appear in canonical activity sidecars after rebuild.
- Missing or freshly rebuilt runtime database transcript proves status/worktree orientation still works without `work_associations`.
