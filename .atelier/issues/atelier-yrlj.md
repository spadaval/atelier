---
created_at: "2026-10-17T01:10:57.976802445+00:00"
id: "atelier-yrlj"
issue_type: "bug"
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
status: "backlog"
title: "Give merge errors a recovery command that names the item and branch"
updated_at: "2026-10-23T14:14:22.355997745+00:00"
---

## Goal

Merge refusals don't say how to recover:

- After a successful merge, `merge --check atelier-nqgr` on master says `merge_branch_mismatch: expected work branch task/atelier-nqgr with distinct base master` rather than saying it's already merged.
- On the wrong branch, the mismatch message doesn't name the current branch or give `git switch`.
- An item that was never started: `merge requires committed branch ownership; run atelier start and commit its metadata` (crates/atelier-cli/src/commands/merge.rs:59), with no ID.

## Done when

- If the owner's recorded work tip is already contained in the base: `atelier-nqgr is already merged into master (c24af21a).` (exit 0 for `--check`).
- A mismatch names both branches and gives `git switch task/atelier-nqgr`.
- The missing-ownership error gives `atelier start atelier-l36s`.
- Tests cover all three.

## Unknowns

Whether `--check` should evaluate a named work branch from another checkout (for example an orchestrator on master checking several branches). Today it requires the work branch to be checked out (docs/product/reviews-and-merge.md "Run these from the recorded work branch"). That needs a spike on whether review_gate can run against a non-HEAD tip.

## Notes

- 2026-10-17 [note] 2026-10-17 (mission atelier-k8h8, merging atelier-w56o): `atelier merge atelier-w56o` run from the base checkout (master, in the main worktree) failed with `merge requires committed branch ownership; run atelier start and commit its metadata`, though w56o was started and its ownership committed on task/atelier-w56o. Merge reads ownership from the current checkout's records, so on the base the owner has no workflow_branch yet. The same command from the work branch's worktree merged fine. The refusal should say to run merge from the work branch (naming it), not to run start.
- 2026-10-17 [note] Retro 2026-10-17: k8h8 05:31Z, merge run from the base checkout said 'merge requires committed branch ownership; run atelier start' though the branch owned the item. The fix is naming the owner checkout path.
- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: 'no ID' bullet done. Still: from base, merge X --check says 'merge_owner_unstarted … atelier start X' though X is started on its branch (merge.rs:417-419); after a successful merge says merge_branch_mismatch with no switch/already-merged (:420-426).
