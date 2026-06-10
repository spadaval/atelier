---
acceptance: []
blocks:
- "atelier-0013"
- "atelier-0014"
created_at: "2026-06-08T19:39:41+00:00"
depends_on:
- "atelier-000e"
- "atelier-000y"
- "atelier-0010"
- "atelier-0011"
evidence_required: []
id: "atelier-0012"
issue_type: "task"
labels:
- "agent-factory"
- "cli"
- "feature"
- "json"
- "migration"
- "mission"
- "tracker"
links: []
parent: "atelier-000z"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Implement Agent Factory tracker command parity in Atelier"
updated_at: "2026-06-08T22:10:56+00:00"
---


Implement or verify the Atelier command surface required by Agent Factory. This is not full Beads compatibility; it is the subset agents actually need for durable planning and execution: ready/list/search/show/create/update/close/dependency operations, stable JSON, lint/doctor-style health checks, and explicit export/check behavior.

Where command names differ from Beads, provide documented mappings or temporary aliases until Agent Factory is migrated.

## Acceptance Criteria

Agent-facing commands cover ready, list, search, show, create, update, close, dep add/remove/list, lint, doctor, export, export --check, and rebuild or documented equivalents; each has stable JSON output; error messages are actionable for missing IDs, invalid deps, stale exports, and dirty state; focused CLI tests cover the Agent Factory command subset.
