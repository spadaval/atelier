---
created_at: "2026-06-23T16:21:20.078243688+00:00"
id: "atelier-5m81"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xa9s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove eager cache refresh from write paths"
updated_at: "2026-06-23T16:21:20.078243688+00:00"
---

## Description

Remove refresh-after-write calls and the canonical mutation storage mode that forces cache rebuilds before and after record-file mutations.

## Outcome

- Write commands no longer eagerly rebuild SQLite cache unless their own output explicitly needs cache-derived data.

## Evidence

Evidence was not specified in the bundle.
