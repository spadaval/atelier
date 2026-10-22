---
created_at: "2026-10-16T23:04:58.948224135+00:00"
id: "atelier-majk"
issue_type: "feature"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show recent tracker activity at session start"
updated_at: "2026-10-16T23:04:58.948224135+00:00"
---

## Goal

A fresh session has no summary of what closed, merged, or changed in the tracker recently and reconstructs it from `git log`.

## Done when

`status` shows a short recent-activity section derived from Git (items closed, merged owners, new findings since a reference point), at most 8 lines. The reference point is fixed when this leaves backlog.

## Unknowns

What "recent" means without session records (ADR 0014/0019): the last N base commits, the upstream, or a time window.
