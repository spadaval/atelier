---
created_at: "2026-06-11T20:10:57.980157137+00:00"
id: "atelier-g9fd"
issue_type: "validation"
labels:
- "cache"
- "recovery"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-gye2"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-dmx2"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T00:21:52.672319566+00:00"
status: "done"
title: "Prove recovery when state.db is missing or stale"
updated_at: "2026-06-12T00:21:52.672319566+00:00"
---

## Description

Delete state.db, modify Markdown, and simulate checkout-like clean state. Acceptance: commands rebuild the ProjectionIndex when safe or fail clearly when canonical Markdown is invalid.
Outcome was not specified in the legacy issue record.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
