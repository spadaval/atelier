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
status: "in_progress"
title: "Evaluate every gate when a precondition fails, with a recovery command"
updated_at: "2026-10-23T15:19:38.227359458+00:00"
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
