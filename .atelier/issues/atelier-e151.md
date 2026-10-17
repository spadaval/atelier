---
created_at: "2026-10-17T01:10:57.967303104+00:00"
id: "atelier-e151"
issue_type: "feature"
labels:
- "cli-output"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let start create the owner worktree, or show the owner branch first"
updated_at: "2026-10-17T01:10:57.967303104+00:00"
---

## Goal

Agents working in parallel worktrees guess the owner branch name and create it by hand before starting: `git worktree add -q ../atelier-wt/ip9e -b bug/atelier-ip9e master && cd … && atelier start atelier-ip9e` ran 50 times in this repository's transcripts and about 22 times in astromech. The branch is `<issue_type>/<owner-id>`, and for a child it is the parent's. A wrong guess (wrong type prefix, or the child's own ID) fails with `start_branch_mismatch` after the worktree already exists. Nothing in `issue show` or `work ready` shows the owner branch before start.

## Done when

Either:
- `atelier start <id> --worktree <path>` creates the worktree on the owner branch (creating the branch from the policy base if needed), records the start there, and prints the path and branch; or
- `issue show <id>` and `work ready` show the owner branch for every startable item (e.g. `→ bug/atelier-ip9e`, or `→ feature/atelier-xxxx (parent)`), so the worktree command can be written without guessing.

Tests cover a standalone item and a child of a parent (whose branch is the parent's).

## Unknowns

Which option. `--worktree` removes the guess entirely but makes `start` manage worktrees; showing the branch is smaller and fits the issue show ownership item (i.show-ownership-proof).
