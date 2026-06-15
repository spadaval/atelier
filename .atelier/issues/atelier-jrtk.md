---
created_at: "2026-06-15T01:15:02.282172989+00:00"
id: "atelier-jrtk"
issue_type: "task"
labels:
- "cli"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-m2nh"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T01:22:30.484877543+00:00"
status: "done"
title: "Relax mission closeout mechanics"
updated_at: "2026-06-15T01:22:30.484877543+00:00"
---

## Description

Relax Atelier's mission closeout mechanics so status and closeout checks enforce the lightweight contract instead of demanding redundant mission-level validation artifacts after linked epics and issues have their own proof.

## Outcome

- Mission status and closeout continue to block on open linked work, open blockers, invalid tracker state, stale projections, missing child proof, and dirty worktree state.
- Mission status and closeout no longer report direct mission evidence or independent mission validation as mandatory for ordinary missions whose linked work is already proved.
- Workflow configuration and built-in policy names match the simplified mission closeout behavior without adding compatibility aliases or hidden fallback gates.

## Evidence

- Code diff for `src/commands/mission.rs`, `src/workflow_policy.rs`, `.atelier/workflow.yaml`, or equivalent implementation files shows redundant mission validation gates removed or narrowed.
- Focused test output demonstrates ordinary mission closeout does not require duplicate mission-level evidence when child work is complete and proved.
- `target/debug/atelier mission status atelier-rxpr` output shows remaining closeout blockers are actionable work/state blockers, not blanket mission revalidation.
