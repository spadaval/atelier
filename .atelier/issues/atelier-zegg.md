---
created_at: "2026-10-23T17:02:43.991656890+00:00"
id: "atelier-zegg"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "review"
    id: "atelier-lkkh"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:33:00.647882504+00:00"
status: "done"
title: "Name a cherry-pick or revert in progress, and warn when recovery drops a merge"
updated_at: "2026-10-23T17:33:00.647882504+00:00"
---

## Goal

Two gaps in land's recovery advice from atelier-0uaj review. (1) `operation_in_progress` (crates/atelier-cli/src/commands/git.rs) detects only rebase-merge, rebase-apply and MERGE_HEAD; a conflicted cherry-pick or revert is not named, so land prints a rebase fix that fails with "Cannot autostash" (finding atelier-lkkh). Detect CHERRY_PICK_HEAD and REVERT_HEAD and print `git cherry-pick --abort` / `git revert --abort`. (2) When the commit that broke review append-only is a merge, `git rebase --onto <sha>~1 <sha>` drops the whole merge, including the merged branch's work; say so, or offer a recovery that keeps the side branch (finding atelier-hf3z).

## Done when

Tests: land in a conflicted cherry-pick and a conflicted revert prints the matching abort first and the helper runs it; land with an append-only violation inside a merge commit warns that the merged work is dropped (or prints a recovery that keeps it), checked by running the fix.

## Context

Review findings atelier-lkkh and atelier-hf3z on atelier-0uaj.

## Notes

- 2026-10-23 [decision] For an append-only violation in a merge commit, land keeps the rebase fix and adds a warning that it drops the N non-merge commits the merge brought in, to merge or redo; it does not offer a recovery that keeps the side branch. Cherry-pick and revert in progress use codes land_cherry_pick_in_progress / land_revert_in_progress (merge_* from merge).
  Reason: The commit rev-list names first may be the merge even when a side commit made the rewrite, so no single recovery keeps the side work safely; the issue allows either a warning or a keeping recovery.
  Rejected: Re-merging the side automatically after the rebase

## Close Reason

Land (and merge --check) names a conflicted cherry-pick or revert as `land_cherry_pick_in_progress` / `land_revert_in_progress` with `git cherry-pick --abort` / `git revert --abort` first, which the next_steps helper runs; an append-only violation in a merge commit adds that the rebase drops the N commits it merged in, checked by running the fix.

### Deviations
None.

### Decisions
See the decision note: warn rather than offer a recovery that keeps the side branch.

### Surprises
Git's operation name has a hyphen, which a check code cannot, so the code part (`cherry_pick`) and the shown name (`cherry-pick`) differ.

### Open items
None.
