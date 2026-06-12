---
acceptance: []
created_at: "2026-06-11T20:54:34.276202555+00:00"
evidence_required: []
id: "atelier-t24d"
issue_type: "task"
labels:
- "agent-factory"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000l"
  - kind: "issue"
    id: "atelier-1uxw"
  - kind: "issue"
    id: "atelier-8ig1"
  - kind: "issue"
    id: "atelier-qd2t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide worker workflow transition command model"
updated_at: "2026-06-11T20:56:50.757820936+00:00"
---

Specify how workers and orchestrators move issues, epics, and missions through workflow states. Scope: whether start/finish/complete commands are separate concepts or ergonomic wrappers over generic workflow transitions; how transition commands run validators, surface missing evidence, support explicit waivers, and record durable handoff; how worker subagents should interact with Atelier from assignment through closeout. Acceptance: an artifact records the command model, the worker subagent flow, which commands are aliases/wrappers versus primitives, and how validation/evidence failures are handled without creating red tape for non-mission issue work.

## Resolution

Workflow transitions are the underlying model. Human-facing `start`, `finish`,
and `complete` commands are ergonomic wrappers over configured workflow
transitions plus the context setup each workflow needs.

Atelier should not expose a generic transition command as the primary worker
interface. Workers and orchestrators should use domain commands that describe
their intent:

- `atelier mission start <mission-id>` focuses and activates the mission for the
  current checkout/worktree. It enforces the one-active-mission invariant.
- `atelier work start <issue-id>` starts tracked issue work. It claims or
  associates the issue, checks active mission context when relevant, verifies
  blockers and worktree cleanliness according to policy, and creates or locates
  branch/worktree context when configured.
- `atelier work finish <issue-id>` remains the issue-work completion wrapper.
  It runs the configured finish/close transition validators, reports missing
  evidence precisely, records durable handoff or result notes, and closes only
  when the transition is allowed.
- Epic and mission closeout should use explicit completion commands, such as
  `atelier epic complete <id>` if epics gain a command group, or equivalent
  issue/mission completion surfaces. These commands run closeout validators
  before marking the record done.

The implementation may have an internal workflow-transition engine and may add
diagnostic workflow commands for inspection, but worker-facing commands should
stay intent-shaped rather than making agents remember raw transition names.

## Validation And Evidence Behavior

Completion commands must run configured validators before mutating status. When
evidence is missing, the command must say which record, criterion, or validator
requires proof and show the command shape needed to attach evidence. The command
must not silently close work with missing required evidence.

Waivers are allowed only as explicit backup behavior for configured waivable
gates. A waiver requires a reason, is recorded durably, and remains visible in
status/closeout views. Non-waivable mechanical gates, such as invalid tracker
state or multiple active missions in one checkout, remain hard failures.

## Worker Subagent Flow

A worker subagent should:

1. Inspect the assigned issue and parent mission/epic context.
2. Create or locate the worktree/branch when the workflow calls for isolation.
3. Run `atelier work start <issue-id>`.
4. Implement the assigned slice without reshaping mission scope.
5. Run the proof required by the issue or workflow.
6. Attach evidence or record durable notes when required.
7. Run `atelier work finish <issue-id>` or the relevant completion command.
8. If completion is blocked, leave a durable handoff with the blocking
   validator, missing evidence, failed proof, or follow-up item.

## Rationale

This keeps orchestration legible without making agents operate a low-level state
machine. Workflow policy owns which transitions and validators exist; CLI
commands provide the stable operator vocabulary. Lightweight non-mission issue
work stays simple because its configured validators remain light, while mission
work can opt into stricter evidence and closeout gates.
