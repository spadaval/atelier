---
created_at: "2026-06-24T20:37:04.736836459+00:00"
id: "atelier-qcbx"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8c91"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Fix objective status open-work reporting for done children"
updated_at: "2026-06-24T20:37:04.736836459+00:00"
---

## Description

Fix the issue status correctness bug where an all-done objective can report terminal health and done totals while still printing `Work: open` for done children. The audit points to `crates/atelier-cli/src/commands/issue_status.rs:196`, where `open_work_ids` appends every child ID not active/ready/blocked, which includes done children.

Constraints:
- Do not patch this only in presentation text; derive open work from objective/work domain state.
- This should align with the command-surface direction to remove `issue status` by moving objective rollup into `issue show`, but existing behavior must not remain wrong during migration.

Risks:
- A terminal objective that appears to have open done children undermines trust in workflow and closeout output.

## Outcome

All-done epic/objective status no longer reports done children as open work or suggests closing a done child. Open work is derived from active, ready, blocked, or backlog/non-terminal state, not from a catch-all remainder bucket.

## Evidence

- Regression test covers an objective with all children done and proves terminal health has no open-work footer.
- If objective status is removed in the same slice, equivalent `issue show <objective-id>` objective rollup test proves the corrected non-terminal work calculation.
- Focused command transcript or CLI test proves the current `atelier-kx2y`-style failure cannot recur.
