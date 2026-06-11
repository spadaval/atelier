---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000c"
issue_type: "epic"
labels:
- "epic"
- "milestone"
- "mission-control"
- "spec"
- "validator"
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0003"
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-001m"
  - kind: "issue"
    id: "atelier-001o"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Milestone 6: Mission Control projection"
updated_at: "2026-06-11T14:41:58.428447610+00:00"
---

Expose enough mechanical state through deterministic JSON projections to support a future dashboard or local UI before building any rich interface. The projection depends on worktree/branch state, first-class records, typed links, workflow policy, workflow validators, and evidence.

Direct live agent-run rows, retry queues, and session metrics are deferred; the initial projection focuses on durable work state, workflow/config health, evidence, validator failures, branches, worktrees, and plan drift.

Acceptance:
JSON projections cover active missions, milestone checkpoint progress, blockers, workflow/config health, branches/worktrees, stale durable-state projections, evidence, workflow validator failures, plan drift, recent decisions, and review/validation queues. Projection schema is documented and deterministic. Rich UI remains explicitly deferred until projections are useful. Live run management is out of scope for the first projection milestone.
