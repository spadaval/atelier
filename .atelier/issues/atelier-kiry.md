---
created_at: "2026-10-17T01:10:57.976180364+00:00"
id: "atelier-kiry"
issue_type: "feature"
labels:
- "cli-output"
- "merge"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T15:49:55.106397530+00:00"
status: "done"
title: "Evaluate every gate in merge --check even when a precondition fails"
updated_at: "2026-10-23T15:49:55.106397530+00:00"
---

## Goal

`atelier merge --check` stops at the first precondition and never shows the gates. On task/atelier-bw0b it prints only `Error: merge_diverged: master is not an ancestor of task/atelier-bw0b…`. An uncommitted record (`canonical_snapshot_uncommitted`) and the wrong branch (`merge_branch_mismatch`) also stop it before any gate runs (crates/atelier-cli/src/commands/merge.rs:37-110). The reader finds the other failing gates (for bw0b: findings, review, work.terminal) only after fixing the first, which costs a fix–rerun round per gate.

## Done when

- With `--check`, divergence from the base and uncommitted canonical records appear as their own gate lines (`⚠ base.ancestor: master is not an ancestor…; git merge master`), and the remaining gates are still evaluated against HEAD and printed.
- A real `atelier merge` (no `--check`) still refuses before integrating.
- A test shows a diverged branch with an open finding reporting both failures.

## Unknowns

Which preconditions can't be turned into gates because evaluating the gates needs them (for example, no recorded work branch at all).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed, and S1 also covers land: merge_diverged bails at merge.rs:478-486 before review_gate::evaluate :535; uncommitted records bail in owned_branch :408, branch mismatch :420. Land bails in land_checked session.rs:339-362 (completion_dirty, branch_mismatch, base_mismatch) before evaluate_session. Make both report preconditions as gate failures and keep evaluating.
- 2026-10-23 [decision] A failed precondition keeps its shipped code (merge_diverged, land_records_uncommitted, canonical_snapshot_uncommitted, ...) as its check name in the gate line and verdict.
  Reason: Codes never change once they ship; gates, tests and agents match on them, and the verdict already listed them.
  Rejected: New check names such as base.ancestor
- 2026-10-23 [decision] Soft preconditions: uncommitted records and code, merge_diverged under fast_forward_only, land_completion_dirty, land_base_mismatch and an invalid candidate policy; the gates then read HEAD's committed records and judge from the fork point. Hard stops (wrong branch, mission, unstarted, merge_conflict, merge_base_records, policy validation, append-only) print after the preconditions found before them with a Not checked: line.
  Reason: Each hard stop leaves nothing the gates can read correctly (wrong checkout, no branch, no merged tree), or, for merge_base_records, would run arbitrary shell checks on a retry its records already refuse.
  Rejected: Making merge_base_records soft (reran merge_checks on a refused retry)
- 2026-10-23 [decision] After records or code were dirty at the start, merge skips its post-check clean-checkout fence; the refusal already stands.
  Reason: The fence compares against a clean start, which a dirty start never had, so it could only replace the precondition lines with a bare error.

## Close Reason

`merge --check`, `merge` and `land` report a failed precondition as one more gate line with its shipped code and a runnable fix, then keep evaluating every gate: against HEAD's committed records when the checkout's differ, and from the fork point when the base has moved on, so a diverged branch with an open finding reports merge_diverged, review.current, findings.disposed and claims.reviewed in one run. A real merge still refuses before integrating.

### Deviations
merge_base_records, merge_conflict, the wrong branch, a mission and an unstarted owner stay hard stops: they print after the preconditions found before them with a `Not checked:` line naming the skipped gates. merge_checkout_dirty became soft too, since the records fallback needed it reported beside them.

### Decisions
See the decision notes on this item: shipped codes kept as check names; which preconditions are soft; the post-check clean fence skipped after a dirty start.

### Surprises
Converting a hard stop after soft preconditions needed a `gates_ran` flag, so a fence refusal after the gates ran is not misreported as "Not checked".

### Open items
None.
