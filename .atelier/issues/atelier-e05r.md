---
created_at: "2026-06-15T06:46:01.611248117+00:00"
id: "atelier-e05r"
issue_type: "bug"
labels:
- "validation"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T06:49:52.049820808+00:00"
status: "done"
title: "Allow closing freshly created issues after evidence"
updated_at: "2026-06-15T06:49:52.049820808+00:00"
---

## Description

Validation of `atelier-t35w` proved that the normal strict workflow can create a
new issue, start it, request review, request validation, and record passing
evidence without committing, but `issue close` is still blocked by
`closeout_clean` because the newly created issue Markdown file is untracked.
The closeout-clean validator already ignores tracker-generated evidence and
activity for the closing issue; it should treat that issue's own newly created
canonical Markdown as the same tracker-generated unit of work while still
blocking unrelated dirty files and hand-edited tracker records.

## Outcome

- `issue close` succeeds after `issue create`, `start`, review/validation
  transitions, and `evidence record` for the same issue without an intervening
  commit.
- `closeout_clean` still blocks unrelated dirty files and hand-edited issue
  Markdown.
- The fix is scoped to tracker-generated closeout bookkeeping and does not
  weaken mission closeout dirty-worktree checks for unrelated changes.

## Evidence

- Focused integration test covers create-to-close with fresh evidence and an
  untracked issue Markdown file.
- Existing dirty-worktree closeout tests still pass.
- `target/debug/atelier lint atelier-e05r`, `target/debug/atelier export
  --check`, and `git diff --check` pass.
