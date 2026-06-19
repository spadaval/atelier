---
created_at: "2026-06-19T03:58:46.844876242+00:00"
id: "atelier-onkp"
issue_type: "feature"
labels:
- "review"
- "room"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:57:47.420371174+00:00"
status: "done"
title: "Implement review merge authority for room mode"
updated_at: "2026-06-19T04:57:47.420371174+00:00"
---

## Description

Implement room-mode merge authority through `atelier review merge`. Merge must
be the review enforcement boundary while ordinary Atelier workflow transitions
remain explicit.

## Outcome

- `atelier review merge` requires a current approval for the current source
  branch head and rejects stale approvals.
- Open blocking findings prevent merge; non-blocking findings are rendered as
  residual risk but do not block.
- Merge preflights verify expected branch policy, clean local integration
  preconditions, and configured local merge/squash behavior.
- Successful merge records a room `merged` event and does not transition the
  issue or epic workflow status.

## Evidence

- CLI tests prove missing approval, stale approval, open blocking finding,
  branch mismatch, dirty worktree, successful merge, and no workflow transition.
- File change fixture for room YAML shows the `merged` event and relevant
  head/branch metadata.
- `atelier lint atelier-onkp` and focused merge tests pass.
