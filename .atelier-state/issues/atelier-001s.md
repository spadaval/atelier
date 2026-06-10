---
acceptance: []
blocks:
- "atelier-001u"
created_at: "2026-06-09T19:47:26.227646644+00:00"
depends_on: []
evidence_required: []
id: "atelier-001s"
issue_type: "task"
labels:
- "assignee:root"
- "bulk"
- "json"
- "plan"
- "task"
links: []
parent: "atelier-001n"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define bulk plan JSON schema and validation"
updated_at: "2026-06-09T20:27:37.504138712+00:00"
---

Define the versioned JSON format for bulk authored plans, including client_ref references, records, issue hierarchy, blockers, typed links, labels, priorities, notes, and apply options.

Acceptance: schema docs and fixtures exist, validation errors identify JSON paths and client_refs, and dry-run output can be generated without mutating state.
