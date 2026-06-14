---
created_at: "2026-06-14T01:09:29.731606411+00:00"
id: "atelier-uy4o"
issue_type: "feature"
labels:
- "runtime"
- "workflow"
- "worktree"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Enforce one active issue per worktree"
updated_at: "2026-06-14T01:09:29.731606411+00:00"
---

## Description

Atelier currently allows multiple runtime `work_associations` with status
`active`, and `get_active_work_association()` picks the most recently started
row. That makes active work ambiguous for status, abandon, mission status,
worktree status, and closeout guidance.

The intended model should be one active issue per checkout/worktree. Parallel
work should use separate Git worktrees, each with its own local runtime state.

Expected behavior:

- `atelier start <issue-id>` fails in a worktree that already has a different
  active issue.
- The failure names the active issue and suggests `atelier abandon <active-id>
  --reason "..."` before switching.
- Starting the same issue again in the same worktree is idempotent or refreshes
  branch/path metadata without creating a second active row.
- `atelier abandon [issue-id] --reason "..."` clears the active issue for the
  current worktree scope.
- `atelier status`, `atelier mission status`, and `atelier worktree status`
  report a single active issue for the current worktree.
- Separate Git worktrees can each have their own active issue.
- The invariant remains runtime-local; active issue state is not written into
  canonical issue Markdown.

## Outcome

- Runtime work association enforces at most one active issue per worktree.
- Re-starting the same issue in the same worktree is safe and does not create
  duplicate active rows.
- Switching active work requires explicit `atelier abandon ... --reason "..."`
  first.
- Status, mission, and worktree surfaces report the scoped active issue
  consistently.

## Evidence

- Integration test where starting issue B after issue A in the same worktree is
  rejected with an actionable message.
- Integration test where starting issue A twice in the same worktree does not
  create multiple active rows.
- Integration test where abandon clears the scoped active issue and allows
  starting issue B.
- Test or fixture proving two separate worktrees can carry different active
  issues.
- Status/mission/worktree output tests proving only one active issue is surfaced
  for the current worktree.
- `atelier lint`, `atelier export --check`, and focused runtime/work test
  transcripts pass.
