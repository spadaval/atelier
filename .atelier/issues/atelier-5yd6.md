---
created_at: "2026-06-23T16:21:20.078980297+00:00"
id: "atelier-5yd6"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0p7e"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Route query commands through lazy cache access"
updated_at: "2026-06-23T16:21:20.078980297+00:00"
---

## Description

Route list, status, workflow, evidence, and graph commands that need indexed data through the lazy cache boundary.

## Outcome

- Cache-dependent commands repair or rebuild stale cache at access time and then return correct query results.

## Evidence

Evidence was not specified in the bundle.
