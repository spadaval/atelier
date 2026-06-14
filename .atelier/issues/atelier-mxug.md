---
created_at: "2026-06-14T02:52:43.165440069+00:00"
id: "atelier-mxug"
issue_type: "task"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make projection rebuild lock-aware and atomic"
updated_at: "2026-06-14T02:52:43.165440069+00:00"
---

## Description

Harden projection rebuild behavior so parallel commands avoid partial refresh, fixed temp-file collisions, and confusing contention failures.

## Outcome

Concurrent readers and rebuilds either complete safely or report a clear retry/recovery message.

## Evidence

- Focused test or validation transcript covers parallel rebuild/read behavior.
- `atelier lint`, `atelier export --check`, and the focused concurrency or scenario test pass.
