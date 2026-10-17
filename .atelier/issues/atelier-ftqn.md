---
created_at: "2026-10-17T01:10:57.982420201+00:00"
id: "atelier-ftqn"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-tciw"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Render other local branches as a dense section of the status page"
updated_at: "2026-10-17T01:10:57.982420201+00:00"
---

## Goal

status.rs:38-44 calls `print_page` a second time, so the branches block gets a page-title `=====` underline where every other section uses `-----`. It also follows the Next Actions footer with no blank line:
```
  Inspect selected work transitions: atelier issue transition <issue-id>
Other local work branches
=========================

task/atelier-43m1 @ ddaf738d: atelier-43m1 [done on branch task/atelier-43m1, not merged into master] Define Close Reason's expected content in work-model.md — 0 active (committed branch records)
No started work on: feature-x, old-pre
```
- The branch name appears twice in each row.
- `0 active` is noise for a done owner.
- `(committed branch records)` is implementation jargon repeated on every row.
- Unowned branches (`No started work on:`, `No tracker on:`) are mixed into the same list.
- A row does not say whether a done branch is approved or what to run next.

## Done when

- The branches render as a `Branches` section with a `-----` underline, inside the single status page and before Next Actions.
- Each row is one dense line, for example `task/atelier-43m1  atelier-43m1 done, not merged ✓ approved  Define Close Reason…` for a done owner and `task/atelier-7b6e  atelier-7b6e in_progress  Batch the per-review…` for an active one.
- Unowned branches are collected on one trailing line, `Unowned: feature-x, old-pre`.
- Redirected output matches terminal content.
- Tests are updated (crates/atelier-cli/tests/session.rs and cli_integration/mission_worktree.rs assert `No started work on:`).

## Context

Rows are built in crates/atelier-cli/src/commands/git.rs:604-624. atelier-7n5u marks ready work that is in flight on these branches, so the two views should use the same branch wording.
