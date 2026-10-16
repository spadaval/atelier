---
created_at: "2026-10-16T18:58:32.824355004+00:00"
id: "atelier-v5tt"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-v5tt
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T21:55:56.565750305+00:00"
status: "done"
title: "Answer review history queries from one walk per land"
updated_at: "2026-10-16T21:55:56.565750305+00:00"
---

## Goal

`atelier land` cost grows with review records times history length: each historical approval and disposition runs about three path-limited `git log --root --reverse` walks over the whole history (atelier-83xi diagnosis). Answer them from fewer walks, and cut the serial per-call cost of the about 600 merge-base and rev-list calls.

## Done when

Land on the benchmark history fixture (scripts/bench_read_commands.py) and on this repository runs a number of `git log` walks that does not grow with the number of review records, with output identical to the current walks, including history simplification at merges, deleted and re-added paths, and grafts; and the time between Git calls (about 2.2 s of 5.5 s on this repository, review finding atelier-fvxd) is attributed and reduced or recorded as accepted. Measured before and after on both.

## Context

atelier-83xi measured land at 5.5-6.7 s on this repository (2,206 commits, 328 review records, 68 retained review events walked) with about 920 Git processes (successful execve, strace -f -z): 216 git log walks (about 2.5 s of 5.5 s by GIT_TRACE2), 302 merge-base, 306 rev-list (about 0.7 s in Git, about 2.2 s of gaps between calls). The history fixture (60 owners, 1,745 commits) lands in about 3.3 s against 122 ms for the guard fixture, 181 of its walks taking about 2.2 s.

## Unknowns

Path-limited history simplification depends on the starting commit, so a single walk from HEAD filtered to ancestors of each event is not equivalent in general; the queries in review_gate.rs (introduced_query, scope_queries) need a design that keeps their exact answers.

## Close Reason

Claim history walks run in the parallel prefetch (only for claims with proof or findings), and each commit range needs one rev-list instead of two (parity test ancestry_path_matches_git_on_branchy_history). land on this repository went from 7.3 s to 5.1 s; the benchmark history fixture is unchanged within noise. Profile and the Bloom filter measurement are in Design; batching the per-review walks is split into atelier-7b6e.
