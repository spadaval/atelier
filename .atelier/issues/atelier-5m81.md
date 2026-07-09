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
closed_at: "2026-07-06T18:43:57.464122460+00:00"
status: "done"
title: "Remove eager cache refresh from write paths"
updated_at: "2026-07-06T18:43:57.464122460+00:00"
---

## Description

Remove refresh-after-write calls and the canonical mutation storage mode that forces cache rebuilds before and after record-file mutations.

## Outcome

- Write commands no longer eagerly rebuild SQLite cache unless their own output explicitly needs cache-derived data.

## Evidence

- Command transcript: `cargo nextest run -p atelier-app -p atelier-cli` passes seven focused eager cache refresh removal tests; evidence record `atelier-zgah` captures the result.
