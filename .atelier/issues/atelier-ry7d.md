---
created_at: "2026-10-17T09:10:55.596980155+00:00"
id: "atelier-ry7d"
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
    id: "atelier-2mky"
    type: "advances"
  - kind: "review"
    id: "atelier-94ku"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Report an invalid status or issue type once in check"
updated_at: "2026-10-17T09:10:55.596980155+00:00"
---

## Goal

`atelier check` shows an issue's invalid status or issue_type twice: once under `records:` from record loading and once as the issue's own finding (`crates/atelier-app/src/lint.rs`, about line 191). Both name the issue's file since atelier-f4ko. Found in its review (finding atelier-94ku); already true on master.

## Done when

- `check` reports an invalid status or issue_type once.
- A test pins the count.
