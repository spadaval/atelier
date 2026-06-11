---
acceptance: []
created_at: "2026-06-11T18:22:52.821916422+00:00"
evidence_required: []
id: "atelier-p6hv"
issue_type: "epic"
labels:
- "assignee:root"
- "markdown"
- "projection"
- "recordstore"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7dj5"
  - kind: "issue"
    id: "atelier-mszu"
  - kind: "issue"
    id: "atelier-yiiz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define projection refresh contract after canonical writes"
updated_at: "2026-06-11T18:33:28.877502115+00:00"
---

Define and implement the post-RecordStore projection refresh contract used by mutation commands. Scope includes choosing full rebuild versus targeted refresh for this mission, making the refresh boundary explicit after successful canonical writes, preserving RuntimeState, and documenting which helpers mutation paths must call. Out of scope: migrating every command family in this epic. Acceptance: commands have a shared projection refresh API after canonical writes; stale projection metadata remains correct; rebuild/export/lint/doctor stay green; downstream mutation epics have a clear API to use.
