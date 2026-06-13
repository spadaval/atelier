---
created_at: "2026-06-11T20:10:54.434763072+00:00"
id: "atelier-cd1l"
issue_type: "task"
labels:
- "cache"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-22z4"
  - kind: "issue"
    id: "atelier-eprw"
  - kind: "issue"
    id: "atelier-unma"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-9m68"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:49:02.590937457+00:00"
status: "done"
title: "Centralize safe auto-refresh from canonical Markdown"
updated_at: "2026-06-11T23:49:02.590937457+00:00"
---

## Description

Implement one freshness policy that checks source hash/mtime/size metadata and refreshes the ProjectionIndex from Markdown when safe. Acceptance: query commands do not use stale SQLite when canonical Markdown changed.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
