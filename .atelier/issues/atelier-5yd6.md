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
closed_at: "2026-07-06T18:32:11.378544150+00:00"
status: "done"
title: "Route query commands through lazy cache access"
updated_at: "2026-07-06T18:32:11.378544150+00:00"
---

## Description

Route list, status, workflow, evidence, and graph commands that need indexed data through the lazy cache boundary.

## Outcome

- Cache-dependent commands repair or rebuild stale cache at access time and then return correct query results.

## Evidence

- Test evidence `atelier-q6d9` records 19 passing CacheManager, command-route
  inventory, missing/stale cache, decision-safety, and degraded-orientation
  scenarios across `atelier-app` and `atelier-cli`.
