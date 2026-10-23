---
created_at: "2026-10-23T14:13:37.836011208+00:00"
id: "atelier-0uaj"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-0uaj
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-7ts2"
  - kind: "issue"
    id: "atelier-grsi"
  - kind: "issue"
    id: "atelier-jiz0"
  - kind: "issue"
    id: "atelier-kiry"
  - kind: "issue"
    id: "atelier-ns04"
  - kind: "issue"
    id: "atelier-r3oy"
  - kind: "issue"
    id: "atelier-y0en"
  - kind: "issue"
    id: "atelier-yrlj"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-h1nc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T16:06:36.918598118+00:00"
status: "done"
title: "Evaluate every gate when a precondition fails, with a recovery command"
updated_at: "2026-10-23T16:06:36.918598118+00:00"
---

## Goal

`land` and `merge --check` evaluate every gate even when a precondition fails (diverged base, uncommitted records, branch mismatch, dirty completion), reporting the precondition as one more failed check with its fix, and the remaining precondition errors name the item, the branch and a runnable recovery.

## Done when

Tests: `merge --check` on a diverged base also reports the review and claim gates; land with uncommitted records and a missing approval reports both; uncommitted and dirty paths are all named (first five, then a count) with the git command; `land <id>` from the wrong branch says `git switch <branch> && atelier land`; merge on an already merged item says so; an invalid policy is reported once. Completes S1. Author proof plus independent adversarial review.

## Scope

Children: atelier-kiry (including land's `land_checked` preconditions), atelier-yrlj, atelier-ns04, atelier-7ts2, atelier-grsi, atelier-y0en. Gate outcomes do not change except that more of them are reported.

## Context

Regroom 2026-10-23 notes. `merge.rs` `owned_branch`/`merge_once` (:397-509); `session.rs` `land_checked` (:197-213, :336-362), `records_uncommitted` (:49-58); `committed_snapshot.rs:236-312`; `base_policy.rs:47,74`.

## Unknowns

How a precondition failure that prevents reading HEAD's records (e.g. unreadable snapshot) is reported alongside gates that need those records.

## Close Reason

### Outcome
`land` and `merge --check` evaluate every gate when a precondition fails. Uncommitted records, dirty code, a diverged base, dirty completion, base mismatch and an invalid committed policy are reported as failed checks in the same list, each with a runnable fix, while the gates run against HEAD's committed records; a line about a record whose checkout copy differs says so and gets the records-commit fix. Preconditions that make gates impossible say which were not checked (atelier-kiry). Every uncommitted record and dirty path is named with its git command (atelier-grsi, atelier-7ts2). `land <id>` off the owner's branch points to `git switch <branch> && atelier land`, and missions get their own refusal (atelier-ns04). Merge errors name the item and branches, and an already merged item says so and passes (atelier-yrlj). An invalid policy is reported once (atelier-y0en). A disposition given a non-finding names its kind (atelier-jiz0). The fix helper reruns the gate after each fix, and the unreachable reopen fix is gone (atelier-r3oy). Completes atelier-rc8j S1; shown by CLI tests that run every printed fix (receipts atelier-bcau, atelier-wjfa and focused receipts on each child). Independent adversarial review found exit codes identical to master in every precondition state, and real merge never integrated when refusing.

### Deviations
Preconditions keep their existing codes as check names. `merge_base_records`, branch mismatch, mission, unstarted owner and merge conflicts still stop the gates, with a `Not checked:` line. `land_base_mismatch`'s fix edits the record with perl, since no command changes a recorded base. The invalid-policy fix is prose.

### Decisions
Decision notes on each child.

### Surprises
Review found a transition fix offered for a done claim present only in uncommitted records (atelier-6ajc) and missing or wrong fixes for base mismatch and owned items (atelier-9a6e); fixed in 1ac26495. Merging master brought two conflicts with atelier-imlh's advice text, resolved in fd8ad78e.

### Open items
Uncommitted review or evidence records are not mapped back to their issue in the HEAD marking. Paths hidden by assume-unchanged or skip-worktree are named but cannot be captured by the offered git command.
