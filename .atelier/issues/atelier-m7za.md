---
created_at: "2026-06-23T16:21:20.083529820+00:00"
id: "atelier-m7za"
issue_type: "validation"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate lazy cache behavior for batch writes"
updated_at: "2026-06-23T16:21:20.083529820+00:00"
---

## Description

Add regression coverage showing relationship-heavy batch writes do not rebuild the full cache after each mutation and the next cache query returns correct graph results.

## Outcome

- Tests or scripted evidence prove lazy cache behavior for the original slow batch scenario.

## Evidence

Evidence was not specified in the bundle.
