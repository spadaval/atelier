---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000w"
issue_type: "task"
labels:
- "projection"
- "spec"
- "storage"
- "task"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000y"
  - kind: "issue"
    id: "atelier-0011"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define `.atelier-state` manifest and record layout"
updated_at: "2026-06-08T21:16:58+00:00"
---


Define the canonical exported layout for manifest, issues, missions, milestones, plans, evidence, graph, and Mission Control projection. Include ID formats, file naming, metadata, ordering, and versioning rules.

## Acceptance Criteria

A documented layout covers every path listed in SPEC.md or explicitly defers it; deterministic ordering and schema/version fields are defined; fixture examples exist for at least issue, graph, and manifest records.
