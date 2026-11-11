---
created_at: "2026-10-17T01:10:57.966392545+00:00"
id: "atelier-hugc"
issue_type: "bug"
labels:
- "cli-output"
- "start"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:38.227336252+00:00"
status: "superseded"
title: "Refuse atelier start on parked items instead of branching"
updated_at: "2026-11-11T03:55:38.227336252+00:00"
---

## Goal

`atelier start fpxc` (status `backlog`) ran `git switch -c feature/atelier-fpxc`, recorded `workflow_branch` on the record, printed `Status: backlog (no start transition from this status)` and exited 0. The work now has a branch but is still classified as backlog, so the `work` views won't show it as active. `crates/atelier-cli/src/commands/start.rs:94-111` skips the transition when none starts from the current status. Rerunning start on an in_progress item prints the same `(no start transition from this status)` (`start.rs:279-283`) instead of saying it is already in progress.

## Done when

- [ ] `atelier start` on a todo-category status with no `start` transition (backlog, draft) refuses before any Git operation or write, naming the way out: `atelier-fpxc is backlog; plan it first: atelier issue transition atelier-fpxc plan`.
- [ ] On an item already in an active status it says `already in_progress on <branch>` and still switches to the branch.
- [ ] CLI tests cover backlog (refused, no branch created, record unchanged) and in_progress (already-started message).

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new `start` already refuses a draft item before any Git operation and names `atelier ready` (run in a scratch clone: `atelier-1a5k is draft, and atelier start runs on an item that is ready. Nothing changed.`).
