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
status: "todo"
title: "Evaluate every gate in merge --check even when a precondition fails"
updated_at: "2026-10-23T15:19:35.194141096+00:00"
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
