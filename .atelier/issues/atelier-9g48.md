---
created_at: "2026-10-22T19:58:07.550441930+00:00"
id: "atelier-9g48"
issue_type: "feature"
labels:
- "merge"
- "orientation"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-9g48
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ftqn"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:57:56.608917335+00:00"
status: "done"
title: "Show which waiting branches are ready to land"
updated_at: "2026-10-22T20:57:56.608917335+00:00"
---

## Goal

`status` lists each local work branch on one dense line. For each branch waiting to land it shows commits behind the base, whether it merges cleanly or which files conflict, approval state, and whether proof covers the branch head. It also names waiting branches that touch the same files, so an orchestrator brings the base in only for the branch that is next.

## Done when

- atelier-ftqn's and atelier-tciw's Done when hold, shown by session tests, with independent review.
- The view changes no Git state.
- Delivers atelier-k8h8 S1.

## Unknowns

atelier-tciw wants the cause of a stale approval from atelier-cisa (mission atelier-rc8j), which is not planned yet. Until it lands, show `approval stale` without the cause.

## Background

Both children change `crates/atelier-cli/src/commands/status.rs`; atelier-ftqn's row format comes first, and atelier-tciw adds the readiness facts. The branches block is already a `-----` section before Next (it renders as `Other Local Work Branches (committed records)`); the dense rows, the `Unowned:` line and approval marks are what remain of atelier-ftqn.

## Notes

- 2026-10-22 [decision] Branches rows mark the status word with ✓ or ⚠ and say 'merged into <base>' / 'not merged', matching the ↳ in-flight mark
  Reason: The general output rule marks status words; the ftqn example without ✓ was incidental (review finding atelier-qjin)
  Rejected: Plain status word as in the Done when example

## Close Reason

### Outcome
`atelier status` lists other local work branches in a `Branches` section before Next, one dense line per owned branch (`<branch>  <id> <status>[, <merge>]  <title>`, with the ✓/⚠ status mark and the in-flight mark's merge wording), and collects unowned branches on one `Unowned:` line (atelier-ftqn). The format is documented in docs/product/human-cli-output.md#branches. This is the row atelier-k8h8 S1's readiness facts will extend.

### Deviations
atelier-tciw (merge readiness per branch) was taken out of this increment on 2026-10-22. Its approval state needs the approval-currency rule atelier-n642 settles on another branch, and keeping it here would have held this reviewed work off the base. It runs on its own branch once n642 lands, and it also owns the approval mark on done-owner rows, moved out of atelier-ftqn (decision note on atelier-ftqn). So this increment delivers S1's row format, not S1.

### Decisions
Rows follow the general output rule: ✓/⚠ beside the status word, wording shared with the ↳ in-flight mark (decision note on atelier-9g48, finding atelier-qjin).

### Surprises
None.

### Open items
atelier-tciw: readiness facts, the approval mark, and activity under parent-owned branches (finding atelier-p3dv), all noted on it.
