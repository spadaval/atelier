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
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:41:02.032816239+00:00"
status: "done"
title: "Render other local branches as a dense section of the status page"
updated_at: "2026-10-22T20:49:41.018911688+00:00"
---

## Goal

Partly done as of 2026-10-22: the block is already a `-----` section inside the status page and before Next, titled `Other Local Work Branches (committed records)`. The rows, the unowned-branch line and the approval mark below are what remain.

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

## Notes

- 2026-10-22 [decision] The approval mark on Branches rows moves to atelier-tciw
  Reason: Review currency for another branch needs a new per-branch computation in review_gate.rs (history walk from the approval pin to that branch's tip plus the independence rule), which tciw's approval-state fact needs anyway; atelier-1ztv is changing that file now
  Rejected: Mark ✓ approved when the branch snapshot holds an approval record: it shows stale approvals as approved
- 2026-10-22 [note] 2026-10-22 correction to this Close Reason (review finding atelier-f4b2): since 1d2f67df the Branches rows do carry the ✓/⚠ status mark and the in-flight mark's wording, as in `x ✓ done, not merged  Title`, `x ✓ done, merged into <base>  Title` and `x ⚠ blocked  Title`, per the decision note on atelier-9g48 (finding atelier-qjin). The Close Reason's row shape and 'no ✓ status mark' line predate that fix.

## Close Reason

`atelier status` now renders a `Branches` section (`-----` underline, before Next) with one dense line per owned branch, `<branch>  <id> <status>[, merged|, not merged]  <title>`, and one trailing `Unowned: ...` line for branches with no started work or no tracker. Redirected output is identical to terminal output. The approval mark is not delivered here.

### Deviations
The `✓ approved` mark from Done when is not implemented; it moved to atelier-tciw by the orchestrator's decision (review_gate.rs is being changed by atelier-1ztv). tciw is now blocked_by atelier-n642 and carries a note.

### Decisions
See the decision note on this item (approval mark moves to tciw; snapshot-only shortcut rejected).

### Surprises
The old row carried an `N active` cue and `branch @ commit`; both were dropped with the rewrite, so the active-count computation and the commit/priority fields on the row struct were removed. Rows no longer use the ✓ status mark, following the issue's examples.

### Open items
Approval mark and the merge-readiness facts: atelier-tciw.
