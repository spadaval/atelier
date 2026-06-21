---
created_at: '2026-06-11T19:53:21.659407023+00:00'
id: atelier-n8ag
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: evidence
    id: atelier-kpp5
    type: validates
  - kind: evidence
    id: atelier-rodj
    type: validates
  - kind: issue
    id: atelier-0001
    type: advances
  - kind: issue
    id: atelier-0006
    type: advances
  - kind: issue
    id: atelier-0007
    type: advances
  - kind: issue
    id: atelier-0008
    type: advances
  - kind: issue
    id: atelier-000c
    type: advances
  - kind: issue
    id: atelier-000i
    type: advances
  - kind: issue
    id: atelier-000l
    type: advances
  - kind: issue
    id: atelier-000r
    type: advances
  - kind: issue
    id: atelier-001m
    type: advances
  - kind: issue
    id: atelier-001p
    type: advances
  - kind: issue
    id: atelier-001q
    type: advances
  - kind: issue
    id: atelier-001r
    type: advances
  - kind: issue
    id: atelier-zh61
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T21:20:37.872775104+00:00'
status: closed
title: Autonomous mission operations
updated_at: '2026-06-11T21:20:37.872775104+00:00'
---

## Description

Make `atelier` the orchestrator's best friend for highly autonomous,
goal-directed agent work. An orchestrator should be able to focus one active
mission, see what should happen next, delegate ready issue slices, create or
locate isolated branches/worktrees, run and enforce validations, ensure required
evidence is present, see blockers and closeout gaps, and finish with durable
proof. The system should enforce the gates humans and agents commonly skip while
keeping ordinary non-mission issue work lightweight.

## Outcome

### Constraints

- Atelier is the durable coordination substrate for Agent Factory mission work.
- Mission guidance lives in CLI workflows and Agent Factory procedures, not a separate Mission Control UI yet.
- The repository has at most one active mission focus; parallel worktrees inherit or explicitly switch that focus.
- High-consequence product, architecture, persistence, security, migration, data-retention, validation-policy, and public-contract choices are resolved before mission start; artifact-update tasks capture any durable follow-up.
- Non-mission issue tasks stay lightweight; high-risk mission work can require evidence, validators, and waivers.
- Human-first command output remains the agent interface; do not restore command-result JSON.
- Existing Markdown-first state and rebuildable projection boundaries remain intact.

### Risks

- Over-strict workflow policy could make ordinary agent work feel like red tape, while under-specified policy could let closeout quality drift.
- Guidance-only process can be skipped by humans or agents unless important gates are enforced by workflow validators and lint.
- Agent Factory guidance can drift from Atelier CLI behavior unless the skill and CLI are validated together.
- Missions can lose autonomy if high-consequence product, architecture, persistence, security, or public-contract choices are left unresolved inside the mission graph.
- A separate dashboard or projection built too early could duplicate state logic before CLI mission status proves the needed control surface.

## Evidence

- Manual check: Linked work proves lightweight and strict workflows, validator-backed transitions, waivers, action-aware guidance, mission status CLI, evidence gaps, blocker visibility, active mission focus, Agent Factory guidance, branch/worktree ergonomics, artifact readiness, and closeout proof.

## Notes

Migrated from `.atelier/missions/atelier-n8ag.md` as a declared mission objective issue.
