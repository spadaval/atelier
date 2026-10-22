---
created_at: "2026-10-22T19:58:07.743188662+00:00"
id: "atelier-yim7"
issue_type: "feature"
labels:
- "land"
- "start"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-yim7
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-81e5"
  - kind: "issue"
    id: "atelier-ecah"
  - kind: "issue"
    id: "atelier-mhx0"
  - kind: "issue"
    id: "atelier-mvzi"
  - kind: "issue"
    id: "atelier-yc4d"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:14:07.258663020+00:00"
status: "done"
title: "Push and start work safely"
updated_at: "2026-10-22T21:14:07.258663020+00:00"
---

## Goal

A refused land can never be pushed, and a push names base commits that belong to another owner. `start` creates the owner branch from an explicit base from any checkout. It refuses a branch whose type prefix does not match, and refuses an item another live session holds unless told to take over.

## Done when

- Each child's Done when holds, shown by CLI tests, with independent review.
- Delivers atelier-k8h8 S10.

## Unknowns

How atelier-81e5 tells a live session from a stale lease (process ID, heartbeat or worktree lock). Decide inside the child and record it as a decision note.

## Background

atelier-ecah and atelier-mvzi change land in `crates/atelier-cli/src/commands/session.rs` and the hooks `setup claude` writes. atelier-yc4d, atelier-mhx0 and atelier-81e5 change `crates/atelier-cli/src/commands/start.rs`. The saved workarounds this retires are: check land's exit status before pushing, and start from a fresh type branch.

## Close Reason

### Outcome
A refused land cannot be pushed: `atelier land --push` pushes exactly the commit land checked, only after land passes, only to the branch's own upstream, and land ends with a documented verdict line (atelier-ecah). Land warns before pushing about unpushed base commits that name neither the owner nor its scope (atelier-mvzi). `start` refuses a branch whose type prefix does not match (atelier-mhx0), takes `--base` from any checkout and resolves the item from the base's records (atelier-yc4d), and refuses an item another live worktree holds unless `--take-over` (atelier-81e5). Delivers atelier-k8h8 S10.

### Deviations
ecah's Done when asked for `set -o pipefail` in setup-generated hooks; it was removed after review because the hook command has no pipe and the prefix exits 2 under older dash, blocking every stop (decision note on atelier-ecah, finding atelier-ztla). The verdict uses the documented `land: passed` / `land: refused — …` forms rather than PASS/FAIL.

### Decisions
See the decision notes on atelier-ecah, atelier-mvzi, atelier-mhx0, atelier-yc4d and atelier-81e5 (lease liveness: worktree exists and heartbeat under 4 hours; no PID).

### Surprises
Review found that pushing to the configured upstream could send a branch tracking origin/main straight to the remote base (finding atelier-rg6u); it is now refused.

### Open items
Plain `start` still records any current branch as base, which land later refuses with land_base_mismatch. The lease cannot tell two sessions in one worktree apart, and land does not renew the heartbeat. mvzi on the base names every unpushed commit (finding atelier-3mrf, no action).
