---
created_at: "2026-06-08T19:39:22+00:00"
id: "atelier-0010"
issue_type: "task"
labels:
- "agent-factory"
- "migration"
- "mission"
- "spec"
- "task"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0011"
  - kind: "issue"
    id: "atelier-0012"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-08T21:43:06+00:00"
status: "done"
title: "Define Atelier replacement MVP for Beads users"
updated_at: "2026-06-08T21:43:06+00:00"
---

## Description

Define the minimum product contract required before Atelier can replace Beads for this repo and Agent Factory. Scope includes required commands, JSON shapes, persistence guarantees, import/export behavior, readiness semantics, dependency semantics, close/update flows, validation checks, and failure modes.

This should distinguish replacement-critical features from later Atelier differentiators such as Mission Control richness, workflow policy, and worktree ergonomics.
A documented MVP matrix maps Beads commands and Agent Factory operations to Atelier equivalents; each required command names text and JSON behavior; cutover blockers are listed with owning beads; non-blocking later features are explicitly deferred; the matrix is linked from AGENTFACTORY.md or docs so future agents know the replacement bar.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
