---
created_at: "2026-10-15T23:47:14.043094544+00:00"
id: "atelier-0349"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-0349
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make atelier land fast enough for the Stop hook"
updated_at: "2026-10-16T01:35:13.864550713+00:00"
---

## Goal

`atelier land` takes about 60 seconds on this repository (89 reviews, a few hundred records), on master with nothing to check, while `prime` takes 0.1 seconds. The Claude Stop hook runs `land` at every stop, so every agent turn waits a minute. Most of the time is system time, which suggests many Git subprocesses.

## Done when

The cause is diagnosed and `land` on this repository's master completes in a few seconds, with a test or benchmark that would catch a return to per-record Git calls.

## Context

Measured 2026-10-15 with both the installed binary (68911771) and current master: real 1m0s, user 17s, sys 45s. The adoption reviews also measured 56-67 s in scratch clones.

## Design

Diagnosis (strace and phase timers): checking historical done claims spawned about 12,500 Git processes, one per object query: `ls-tree` per (commit, path) entry, `cat-file -t` and `cat-file blob` per blob, `rev-parse --verify` per commit, `show -s` per author, and `diff --name-status` per edge. The same answers were asked many times within one read phase. Once spawns were removed, the remaining cost was a quadratic tree diff, then about 150 serial `git log` path walks (about 46 ms each) from `original_independent`.

Changes, all inside `ReadContext` (one read phase), keeping every query's meaning:

- One `git cat-file --batch` process serves commit IDs, blobs and tree objects. Tree entries are resolved by walking tree objects, and edge paths by an in-process tree diff (both sides of a rename are listed, as `git diff --name-status` does). The first edge in each read phase still runs `git diff` as the reference, which also surfaces diff configuration errors. If the tree diff ever disagrees, `git diff` is used for every edge.
- Commit parents stay on `rev-list`, so shallow and graft boundaries still apply. Authors are parsed from the commit object only for UTF-8 commits with ordinary identities and UTF-8 log output; anything else still uses `git show`.
- Memoized per read phase: commit IDs, entries, blobs, edges (shared, not copied), first parents, authors, `git log` history queries, ancestry (a later commit asked about more than three times gets its ancestor set listed once), and the status-only comparison of two record blobs.
- The `git log` walks behind `original_independent` are prefetched in parallel (up to 8 at a time), as exactly the same queries.

Result on this repository's master: release build 4.2 s (from 60 s), debug build 6.6 s.

Regression found while measuring: since o6h9 merged, `land` on master failed. History still holds atelier-1ioo in the retired epic shape, and its migration commit read as a new completion claim, which made its approvals stale. `claim_endpoint` and the status-only edge check now read a pre-migration epic through the same rewrite `atelier migrate epics` applies (moved to `atelier_app::epic_migration`). So an exact migration is a shape change, not an authored edit, while any content change is still one.

Test: `landing_reads_git_objects_through_one_batch_not_per_record_processes` counts Git calls through a PATH wrapper and fails if per-object queries grow with the number of accepted records. It fails on master and passes here.
