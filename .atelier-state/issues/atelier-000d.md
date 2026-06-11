---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000d"
issue_type: "task"
labels:
- "docs"
- "fork"
- "spec"
- "task"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000k"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Document Chainlink provenance and inherited architecture boundaries"
updated_at: "2026-06-08T19:51:35+00:00"
---


Make the starting point explicit: this repo begins from `dollspace-gay/chainlink` and inherits Rust CLI structure, SQLite persistence, sessions, locks, hooks, token usage, and JSON output. Document what is baseline, what is target Atelier design, and what is intentionally deferred.

## Acceptance Criteria

Docs name Chainlink provenance, inherited modules, target Atelier changes, and preservation expectations; docs link back to SPEC.md and CONTEXT.md; no current target-state doc relies only on historical prose.
