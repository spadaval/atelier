---
acceptance: []
blocks:
- "atelier-000y"
- "atelier-0011"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000k"
- "atelier-000x"
evidence_required: []
id: "atelier-000w"
issue_type: "task"
labels:
- "projection"
- "spec"
- "storage"
- "task"
links: []
parent: "atelier-0009"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define `.atelier-state` manifest and record layout"
updated_at: "2026-06-08T21:16:58+00:00"
---


Define the canonical exported layout for manifest, issues, missions, milestones, plans, evidence, graph, and Mission Control projection. Include ID formats, file naming, metadata, ordering, and versioning rules.

## Acceptance Criteria

A documented layout covers every path listed in SPEC.md or explicitly defers it; deterministic ordering and schema/version fields are defined; fixture examples exist for at least issue, graph, and manifest records.
