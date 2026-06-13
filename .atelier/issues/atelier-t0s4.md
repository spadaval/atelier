---
created_at: "2026-06-11T20:10:54.046258349+00:00"
id: "atelier-t0s4"
issue_type: "task"
labels:
- "architecture"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-cd1l"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-zei2"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:42:14.026415256+00:00"
status: "done"
title: "Introduce explicit command storage access modes"
updated_at: "2026-06-11T23:42:14.026415256+00:00"
---

## Description

Replace scattered get_db/get_fresh_projection_db decisions with explicit access modes: projection query, canonical mutation, runtime-only, and health/repair. Acceptance: command entrypoints declare their mode and share common setup/failure behavior.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
