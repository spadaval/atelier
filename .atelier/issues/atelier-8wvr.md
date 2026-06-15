---
created_at: "2026-06-15T05:13:34.692791811+00:00"
id: "atelier-8wvr"
issue_type: "task"
labels:
- "markdown"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-y3ur"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Split canonical record parsing and rendering into atelier-records"
updated_at: "2026-06-15T07:23:18.996959705+00:00"
---

## Description

Move canonical record parsing, rendering, kind registration, and activity sidecar handling into `atelier-records`.

## Outcome

- `atelier-records` owns canonical path derivation, front matter/body parsing, deterministic rendering, record-kind registry, and activity sidecar parsing.
- Current authored `.atelier/` Markdown layout remains readable and renderable.
- The old oversized `record_store.rs` is split into cohesive modules by durable ownership.

## Evidence

- Focused record parsing/rendering tests pass for current committed fixture records.
- File review shows record-store code split by responsibility, not only renamed.
- `atelier lint` can still validate canonical records.
