---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000p"
issue_type: "task"
labels:
- "closeout"
- "spec"
- "task"
- "validation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000f"
  - kind: "issue"
    id: "atelier-000u"
  - kind: "issue"
    id: "atelier-0010"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "[CLOSEOUT] Validate Milestone 2 storage contract"
updated_at: "2026-06-08T21:38:57+00:00"
---


Close out canonical export/rebuild by proving the runtime-store and repo-state contract works end to end.

## Acceptance Criteria

Classify each Milestone 2 criterion from SPEC.md; provide evidence for deterministic export, stale export --check, rebuild from clean checkout-like state, and normal test coverage; name any deferred record families explicitly.
