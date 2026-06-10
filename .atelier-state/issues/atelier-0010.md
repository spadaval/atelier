---
acceptance: []
blocks:
- "atelier-0011"
- "atelier-0012"
created_at: "2026-06-08T19:39:22+00:00"
depends_on:
- "atelier-000p"
evidence_required: []
id: "atelier-0010"
issue_type: "task"
labels:
- "agent-factory"
- "migration"
- "mission"
- "spec"
- "task"
- "tracker"
links: []
parent: "atelier-000z"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Define Atelier replacement MVP for Beads users"
updated_at: "2026-06-08T21:43:06+00:00"
---


Define the minimum product contract required before Atelier can replace Beads for this repo and Agent Factory. Scope includes required commands, JSON shapes, persistence guarantees, import/export behavior, readiness semantics, dependency semantics, close/update flows, validation checks, and failure modes.

This should distinguish replacement-critical features from later Atelier differentiators such as Mission Control richness, workflow policy, and worktree ergonomics.

## Acceptance Criteria

A documented MVP matrix maps Beads commands and Agent Factory operations to Atelier equivalents; each required command names text and JSON behavior; cutover blockers are listed with owning beads; non-blocking later features are explicitly deferred; the matrix is linked from AGENTFACTORY.md or docs so future agents know the replacement bar.
