---
created_at: "2026-10-17T19:00:36.355034861+00:00"
id: "atelier-81e5"
issue_type: "feature"
labels:
- "retro-2026-10-17"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Warn when another live session holds the item being started"
updated_at: "2026-10-17T19:00:36.355034861+00:00"
---

## Goal

Two sessions worked the same items: another session ran `start atelier-92mz` in the main checkout (10-16 18:10), and a resumed session edited the 37uq worktree, forcing a handoff (8 cross-session messages).

## Done when

`atelier start` and `atelier status` report when another live session or worktree holds the item (by a runtime lease under `.atelier/runtime`), and `start` refuses without `--take-over`. Tests cover a live and a stale lease.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
