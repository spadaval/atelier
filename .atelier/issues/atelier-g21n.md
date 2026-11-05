---
created_at: "2026-11-05T04:47:42.479935642+00:00"
id: "atelier-g21n"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mgl0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Leave out a report Decisions part that only points at the item's decision notes"
updated_at: "2026-11-05T04:47:42.479935642+00:00"
---

## Goal

Since atelier-mgl0 the debrief shows each report's Decisions part, so a part that only points at the notes ("See the decision notes on this item.") shows the same decision twice next to the notes it names (about 10 reports in this repo; vt8q's l7gl entry).

## Done when

A Decisions part that only points at the entry's decision notes is left out when the entry has notes, with a fixture test; parts with their own content still show.
