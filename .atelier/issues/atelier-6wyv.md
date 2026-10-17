---
created_at: "2026-10-17T08:19:41.002938778+00:00"
id: "atelier-6wyv"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Offer start only for work mission rows that pass the readiness gates"
updated_at: "2026-10-17T08:41:50.487189145+00:00"
---

## Goal

`work mission`'s Ready Work skips the readiness gates `work ready` applies, so the `atelier start <top ready>` step atelier-ksx6 added can name an item `start` then refuses. Found while implementing atelier-ksx6.

## Done when

- `work mission` and `work parent` classify Ready Work with the same gates as `work ready`, or offer start only for an item that passes them.
- A test covers an item with an unmet readiness gate.
