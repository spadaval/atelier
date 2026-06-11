---
acceptance: []
created_at: "2026-06-11T18:22:53.718926187+00:00"
evidence_required: []
id: "atelier-7dj5"
issue_type: "epic"
labels:
- "assignee:root"
- "graph"
- "markdown"
- "recordstore"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-afir"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Move issue relationship writes onto RecordStore"
updated_at: "2026-06-11T18:42:15.705152044+00:00"
---

Move issue relationship mutations onto RecordStore-owned Markdown writes before projection refresh. Scope includes labels, dependencies/blockers, typed relations, hierarchy relationship updates, and top-level dep aliases. Out of scope: issue field lifecycle and first-class record links. Acceptance: relationship front matter is canonical; SQLite relationship tables are projection rows only; ready/list/impact/tree workflows remain correct after rebuild; tests cover label/unlabel, block/unblock, relate/unrelate, dep add/remove, and invalid/cyclic relationship failure behavior.
