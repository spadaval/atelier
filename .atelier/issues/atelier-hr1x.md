---
created_at: "2026-06-25T16:23:02.656523106+00:00"
id: "atelier-hr1x"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement work queue as mission-organized replacement for issue list"
updated_at: "2026-06-25T16:23:02.656523106+00:00"
---

## Description

Move the current grouped queue behavior from `issue list` to `work queue`, adding mission-level grouping via `advances` links above epic/root-parent grouping. Preserve useful filters such as status, category, priority, label, and quiet IDs; add state filters for ready, active, blocked, and backlog.

## Outcome

`work queue` renders the repo-wide operational queue with mission -> epic/root -> issue grouping, blocker-aware ordering, useful metadata filters, state filters, quiet ID output, and orphan/unassigned work handling. It preserves the useful current `issue list` behavior while making mission context the default organization.
