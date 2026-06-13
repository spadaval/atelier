---
created_at: "2026-06-11T20:28:37.395693161+00:00"
id: "atelier-1tq7"
issue_type: "task"
labels:
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1uxw"
  - kind: "issue"
    id: "atelier-qd2t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T20:35:35.900161156+00:00"
status: "done"
title: "Define active mission focus and lifecycle rules"
updated_at: "2026-06-11T20:35:35.900161156+00:00"
---

## Description

Define the repo-scoped active mission model for Atelier. Specify lifecycle states, the one-active-mission invariant, switch/deactivate behavior, how multiple worktrees inherit active mission context, how non-mission issue tasks stay outside the rule, and what lint/merge validation catches when branches diverge. Acceptance: docs or tracker artifact names the lifecycle states and invariant; command behavior for mission start/switch/finish/status is specified; lint behavior for multiple active missions is specified; strict versus advisory behavior is risk-scaled.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Resolution

Atelier has one active mission per checkout/worktree.

Active mission state is stored in canonical in-tree Markdown, so the enforceable
scope is the current checkout's durable tracker state. That means a given
checkout/worktree can have at most one active mission. Parallel worktrees inherit
whatever active mission exists in their checked-out Markdown state; if branches
diverge and produce multiple active missions, `atelier lint` must report that as
invalid canonical state.

### Command Semantics

- `atelier mission start <id>` marks that mission active for the current
  checkout and fails if another mission is active unless an explicit switch
  operation is requested.
- `atelier mission status` without an ID uses the active mission when one
  exists.
- `atelier work start <issue-id>` uses the active mission as orchestration
  context for mission-linked work.
- Non-mission issue tasks remain outside the active mission requirement.

### Rationale

The product goal is orchestration focus, not global process supervision. Since
Atelier's durable state lives in Markdown inside the repository, repo-local
canonical state is the only place this invariant can be reliably enforced.
Keeping one active mission per checkout makes default commands obvious while
still allowing normal Git branch/worktree isolation.
