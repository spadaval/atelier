---
created_at: "2026-10-16T23:04:58.820768684+00:00"
id: "atelier-fdxj"
issue_type: "feature"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show what to do next: blocker chains and the work each item unblocks"
updated_at: "2026-10-16T23:04:58.820768684+00:00"
---

## Goal

Ready work is ordered only by priority and ID. Nothing shows which item unblocks the most work or how blockers chain, so "what next" across a mission needs manual graph walking.

## Done when

A mission or parent view shows, for each open item, the open items it transitively unblocks, and ranks ready items by that count after priority. Concrete form (a `work next` command or a section in `work mission`) is fixed when this leaves backlog.

## Unknowns

Whether this belongs in `work ready`, `work mission`, or its own view; whether a blocker tree reads better than counts.
