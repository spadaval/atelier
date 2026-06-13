---
created_at: "2026-06-10T00:34:04.827838719+00:00"
id: "atelier-0026"
issue_type: "task"
labels:
- "assignee:root"
- "graph"
- "links"
- "markdown"
- "storage"
- "task"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0023"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T01:14:14.182337625+00:00"
status: "done"
title: "Move graph relationships into record files"
updated_at: "2026-06-10T01:14:14.182337625+00:00"
---

## Description

Remove graph.json as a canonical source-of-truth file by storing relationships with the owning Markdown records. Define and implement deterministic front matter for dependencies, parent/child ownership, and typed links so Git merges happen per record rather than through one aggregate graph file.

## Outcome

graph.json is no longer required in .atelier-state; issue dependencies and typed links rebuild from record front matter; export/check/lint detect dangling links, duplicate links, invalid relation types, and asymmetric compatibility cases; tests cover multi-record links and merge-friendly per-record changes; docs and fixtures no longer describe graph.json as canonical.

## Evidence

Evidence was not specified in the legacy issue record.
