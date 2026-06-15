---
created_at: "2026-06-15T05:13:41.925589382+00:00"
id: "atelier-xmvz"
issue_type: "task"
labels:
- "projection"
- "sqlite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-wng0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement atelier-sqlite rebuild freshness and query APIs"
updated_at: "2026-06-15T05:13:41.925589382+00:00"
---

## Description

Implement the `atelier-sqlite` rebuild, freshness, and query APIs against the replacement schema.

## Outcome

- Projection rebuild recreates query state from canonical Markdown records.
- Freshness checks prevent stale query answers or repair safely before answering.
- Ready, blocked, search, graph, workflow lookup, and mission status queries use the new projection APIs.

## Evidence

- Focused SQLite tests cover rebuild from fixtures, source freshness changes, and representative query APIs.
- Missing or stale database transcript proves recovery through normal product surfaces.
- Representative CLI tests for ready/list/search/graph/mission status pass.
