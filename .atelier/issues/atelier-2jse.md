---
created_at: "2026-06-23T16:21:20.077491339+00:00"
id: "atelier-2jse"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-5m81"
  - kind: "issue"
    id: "atelier-5yd6"
  - kind: "issue"
    id: "atelier-ckca"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Introduce CacheManager get-cache boundary"
updated_at: "2026-06-23T16:21:20.077491339+00:00"
---

## Description

Introduce the cache access boundary that opens existing cache state, checks source freshness when cache data is needed, and rebuilds or repairs stale cache on demand.

## Outcome

- Most cache-using command paths call a single cache access boundary instead of choosing projection storage modes directly.

## Evidence

Evidence was not specified in the bundle.
