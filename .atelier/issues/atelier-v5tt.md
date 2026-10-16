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

Land runs fewer serial Git calls with identical answers: the per-claim history walks run in the parallel prefetch, and each commit range costs one rev-list instead of two, with a parity test against `git rev-list --ancestry-path`; land is measured before and after on this repository and the benchmark history fixture. Batching the per-review walks themselves, and attributing the time between Git calls, are split into atelier-7b6e (scope narrowed 2026-10-16; the original Done when asked for walk counts that do not grow with review records).

## Context

atelier-83xi measured land at 5.5-6.7 s on this repository (2,206 commits, 328 review records, 68 retained review events walked) with about 920 Git processes (successful execve, strace -f -z): 216 git log walks (about 2.5 s of 5.5 s by GIT_TRACE2), 302 merge-base, 306 rev-list (about 0.7 s in Git, about 2.2 s of gaps between calls). The history fixture (60 owners, 1,745 commits) lands in about 3.3 s against 122 ms for the guard fixture, 181 of its walks taking about 2.2 s.

## Design

Profile (2026-10-16, release build, this repository at 2,368 commits with 408 review records, 62 retained done claims): land on master took 7.3 s. Phase timers showed the review gate re-verifying all retained claims: prefetched review history walks 1.6 s, approval independence 1.25 s, FIX NOW resolutions 2.2 s (mostly 62 serial `git log` walks finding where each claim was made), proof 1.3 s (in-process record reads), append-only history 0.2 s.

Changes: the claim walks (`claim_history_query`, shared with `claim_endpoint`) join the first parallel prefetch round, for claims that have proof or findings (others never look up their claim point); `uncached_edges` computes the ancestry path from its `rev-list --parents` output instead of a second `rev-list --ancestry-path` (parity test ancestry_path_matches_git_on_branchy_history).

Result: Git processes per land on this repository 1,336 to 1,136 (GIT_TRACE2 start events), wall time 7.3 s to 5.1 s on a quiet machine; the benchmark history fixture unchanged within noise (2.9-3.7 s for both binaries under load average 9). Separately, Git changed-path Bloom filters (`git commit-graph write --reachable --changed-paths`) cut land here from 5.2 s to 3.3 s with no code change, because the remaining cost is path-limited log walks; whether Atelier sets them up is a product decision.

## Unknowns

Path-limited history simplification depends on the starting commit, so a single walk from HEAD filtered to ancestors of each event is not equivalent in general; the queries in review_gate.rs (introduced_query, scope_queries) need a design that keeps their exact answers.

## Close Reason

Claim history walks run in the parallel prefetch (only for claims with proof or findings), and each commit range needs one rev-list instead of two (parity test ancestry_path_matches_git_on_branchy_history). land on this repository went from 7.3 s to 5.1 s; the benchmark history fixture is unchanged within noise. Profile and the Bloom filter measurement are in Design; batching the per-review walks is split into atelier-7b6e.
