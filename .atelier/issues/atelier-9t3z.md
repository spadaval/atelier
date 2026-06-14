---
created_at: "2026-06-13T17:36:33.910714883+00:00"
id: "atelier-9t3z"
issue_type: "epic"
labels:
- "epic"
- "workflow"
priority: "P0"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3z35"
  - kind: "issue"
    id: "atelier-jwcz"
  - kind: "issue"
    id: "atelier-n0p4"
  - kind: "issue"
    id: "atelier-y041"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T19:34:49.866848598+00:00"
status: "done"
title: "Epic: Enforce configured issue workflow transitions"
updated_at: "2026-06-13T19:34:49.866848598+00:00"
---

## Description

Replace hardcoded issue lifecycle behavior with configured workflow transitions
while preserving the small operator path that `zen.md` calls for. This epic
groups the implementation work that makes policy-backed issue states visible,
actionable, and enforceable across mutation and orientation commands.

Children own the executable work: `atelier-n0p4` bootstraps and migrates policy
state, `atelier-y041` replaces the transition engine, `atelier-3z35` rewires
start/close/abandon wrappers, and `atelier-jwcz` updates ready queues and
status surfaces.
- Repositories can initialize `.atelier/workflow.yaml`, migrate existing issue
  statuses deliberately, and reject unmigrated or missing-policy mutation paths
  with actionable repair commands.
- `atelier issue transition`, `atelier start`, `atelier issue close`, and
  `atelier abandon` execute or explain configured workflow transitions using
  built-in validators rather than hardcoded open/closed behavior.
- `atelier status`, issue views, ready queues, mission summaries, and status
  filters show workflow categories and exact status IDs without requiring
  operators to inspect raw workflow-validator internals.
- Child evidence for `atelier-n0p4`, `atelier-y041`, `atelier-3z35`, and
  `atelier-jwcz` proves init, migration, transition execution, blocked
  transitions, close gating, abandon behavior, and orientation output.
- Epic closeout note or evidence maps each Outcome bullet to child issue
  evidence and names any residual risks or follow-up IDs.
- Focused workflow tests or transcripts, `atelier lint`, and
  `atelier export --check` pass after the child implementation work.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
