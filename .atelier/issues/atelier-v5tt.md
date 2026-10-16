---
created_at: "2026-10-16T18:58:32.824355004+00:00"
id: "atelier-v5tt"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Answer review history queries from one walk per land"
updated_at: "2026-10-16T18:58:32.824355004+00:00"
---

## Goal

`atelier land` cost grows with review records times history length: each historical approval and disposition runs two or three path-limited `git log --root --reverse` walks over the whole history (atelier-83xi diagnosis). Answer them from fewer walks.

## Done when

Land on the benchmark history fixture (scripts/bench_read_commands.py) and on this repository runs a number of `git log` walks that does not grow with the number of review records, with output identical to the current walks, including history simplification at merges, deleted and re-added paths, and grafts. Measured before and after on both.

## Context

atelier-83xi measured land at 5.8-6.7 s on this repository (2,206 commits, 323 review records), with 1,310 Git processes: 617 git log walks (about 27 ms each), 297 merge-base, 292 rev-list. The history fixture (60 owners, 1,745 commits) lands in about 3.4 s against 122 ms for the guard fixture.

## Unknowns

Path-limited history simplification depends on the starting commit, so a single walk from HEAD filtered to ancestors of each event is not equivalent in general; the queries in review_gate.rs (introduced_query, scope_queries) need a design that keeps their exact answers.
