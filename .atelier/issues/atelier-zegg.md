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
status: "backlog"
title: "Name a cherry-pick or revert in progress, and warn when recovery drops a merge"
updated_at: "2026-10-23T17:02:43.991656890+00:00"
---

## Goal

Two gaps in land's recovery advice from atelier-0uaj review. (1) `operation_in_progress` (crates/atelier-cli/src/commands/git.rs) detects only rebase-merge, rebase-apply and MERGE_HEAD; a conflicted cherry-pick or revert is not named, so land prints a rebase fix that fails with "Cannot autostash" (finding atelier-lkkh). Detect CHERRY_PICK_HEAD and REVERT_HEAD and print `git cherry-pick --abort` / `git revert --abort`. (2) When the commit that broke review append-only is a merge, `git rebase --onto <sha>~1 <sha>` drops the whole merge, including the merged branch's work; say so, or offer a recovery that keeps the side branch (finding atelier-hf3z).

## Done when

Tests: land in a conflicted cherry-pick and a conflicted revert prints the matching abort first and the helper runs it; land with an append-only violation inside a merge commit warns that the merged work is dropped (or prints a recovery that keeps it), checked by running the fix.

## Context

Review findings atelier-lkkh and atelier-hf3z on atelier-0uaj.
