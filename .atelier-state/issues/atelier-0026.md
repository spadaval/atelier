---
acceptance: []
blocks:
- "atelier-0023"
created_at: "2026-06-10T00:34:04.827838719+00:00"
depends_on: []
evidence_required: []
id: "atelier-0026"
issue_type: "task"
labels:
- "assignee:root"
- "graph"
- "links"
- "markdown"
- "storage"
- "task"
links: []
parent: "atelier-0024"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Move graph relationships into record files"
updated_at: "2026-06-10T01:14:14.182337625+00:00"
---

Remove graph.json as a canonical source-of-truth file by storing relationships with the owning Markdown records. Define and implement deterministic front matter for dependencies, parent/child ownership, and typed links so Git merges happen per record rather than through one aggregate graph file.

## Acceptance

graph.json is no longer required in .atelier-state; issue dependencies and typed links rebuild from record front matter; export/check/lint detect dangling links, duplicate links, invalid relation types, and asymmetric compatibility cases; tests cover multi-record links and merge-friendly per-record changes; docs and fixtures no longer describe graph.json as canonical.
