---
created_at: "2026-10-15T19:31:01.940323187+00:00"
id: "atelier-83xi"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-83xi
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:58:42.510580920+00:00"
status: "done"
title: "Reduce land and merge --check cost on long Git histories"
updated_at: "2026-10-16T18:58:42.510580920+00:00"
---

## Goal

Keep `atelier land` and `merge --check` practical on repositories with long histories, and make the benchmark show it.

## Done when

The read-command benchmark includes a long-history fixture; the cause of the per-commit cost is diagnosed; land/merge --check on this repository are measured before and after any change.

## Context

Independent validation 2026-10-15 (S4 evidence atelier-ggv8): land and merge --check take 7.5-8.2 s on this repository (1,943 commits), with about 4,600 git processes per land, while the benchmark fixture shows about 0.3 s. Still under the 30 s Claude hook timeout. No latency budget applies (human direction 2026-10-04); this is a practical-impact observation.

## Design

Diagnosis (2026-10-16, installed binary 0.2.0+b9cdced8, this repository at 2,206 commits with 323 review records): `land` on master takes 5.8-6.7 s and starts 1,310 Git processes. 617 are `git log --root --reverse` walks, the history queries `original_independent` runs for every retained approval and finding disposition (`introduced_query` and the two `scope_queries`), each about 27 ms because it walks the whole history; prefetching runs them 8 at a time. Another 297 are `merge-base --is-ancestor` (about 4.5 ms each) and 292 are the `rev-list` pairs behind commit edges. So cost grows with review records times history length; record parsing and object reads (one `cat-file --batch` per read context) are small.

The benchmark now has a history fixture: 60 owners filed, started, closed, independently approved and integrated with `atelier merge`, spread over 1,500 filler commits (1,745 commits). `land` on an active owner there takes about 3.4 s, against 122 ms on the single-approval guard fixture (3 runs, release build, 12 cores). The benchmark generator also wrote retired epic records, which `check` refuses since atelier-o6h9, so it could not run; it now writes parent features.

No product change in this item: removing the per-record walks needs a design that keeps path-limited history simplification exact, filed as atelier-v5tt. Before/after for this item is therefore the same binary.

## Close Reason

The read-command benchmark includes a long-history fixture (60 integrated, approved owners over 1,745 commits; land about 3.4 s against 122 ms) and generates parent features again, so it runs. Diagnosed: land cost is review records times history length, chiefly two or three path-limited git log walks per retained review event (617 of 1,310 Git processes on this repository, 5.8-6.7 s). Measured on this repository; no product change here, the walk reduction is filed as atelier-v5tt. Details in Design.
