---
created_at: "2026-10-01T23:04:17.648346752+00:00"
id: "atelier-8ud1"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xz11"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Meet the expanded rebuild read latency contract"
updated_at: "2026-10-02T00:27:56.651256106+00:00"
---

## Goal

Make every ordinary read and supported guard/audit read meet the rebuild S4 latency budget without weakening correctness guards.

## Done when

A quiet production benchmark on the independent 1000-issue/200-evidence mixed fixture and successful owner/review fixture shows each read under 100 ms, including epic dashboard, prime, landing, merge checks and prune dry run. Fresh full suites retain evidence freshness, hidden Git byte/mode checks, append-only review, policy and atomic prune behavior. All fixes are independently reviewed; physical non-test Rust remains below 20000 lines.

## Scope

Command-local Git query reuse and batching, scoped session projections, and prune read planning. No persistent cache or weaker mutation/freshness fences.

## Context

Independent acceptance V1 at b7aee82cb18837533ebc8ab2c1ff15041e2f5f66: /tmp/atelier-independent-acceptance.md and expanded benchmark logs. Existing six-read benchmark omitted slow dashboards and guards. Keep all earlier failed evidence.

## Notes

- 2026-10-01 [note] Expanded the checked-in release benchmark from six reads to 44 query/guard/refusal shapes (7756de5b). Fresh one-run functional smoke exercises both generated 1000-issue fixtures and reports the remaining budget failures honestly; this concurrent smoke is not latency acceptance. Prune candidate dcb7b022 reduces Git calls from 1690 to 6 and retains identical output. Its 113 ms author timing used one CPU, while the acceptance environment has 32 CPUs; preserve that result as a distinct runtime observation and remeasure the default environment quietly. Evidence candidate f030b7f4 still has epic max116 ms and successful guards above500 ms. Session/history batching and genuine source cleanup remain underway.
- 2026-10-02 [note] Prune owner planning at d34b6e1f is independently reviewed and passes the focused six-mode benchmark (maximum 88.79 ms); xj9h records that partial result, not whole S4 acceptance. Reviewed source cleanup through ed6ac4c0 is integrated at 610edd1d, with 19649 physical non-test Rust lines before the remaining guard changes. Command-scoped history/range/object batching and raw blob hashing are being profiled; repeated fresh filesystem checks and Git launch/wait remain material costs. Preserve all failed timing evidence. Ordered parallel fresh checks and bounded executable resolution are candidates; no mutable fence may be skipped.
