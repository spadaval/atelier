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
updated_at: "2026-10-02T01:33:16.570450430+00:00"
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
- 2026-10-02 [note] Guard correctness stack through aab00773 and root composition f331bf0b is independently accepted: Git boolean parity and older-version batch framing, rename-source review immutability, root introduction history, and original-object proof pins. G9-G13 native findings have reviewed resolution events. Root fresh 549 test-hooks tests pass; exact physical non-test Rust is 19887. S4 is still failing. Ordered parallel filesystem checks showed no consistent gain and were discarded; all serial/parallel/short-PATH samples remain in /tmp/session-filesystem-timing.json. Portable-source attribution now identifies repeated immutable tree enumeration and duplicate canonical mode scans; next bounded slice will reuse only full-OID tree entries and freshly rerun all filesystem checks at every fence.
- 2026-10-02 [observation] Quiet broad production diagnosis at 520c62e0 (binary SHA256 3f7a08c9, retained /tmp/atelier-root-original-objects-release) exercises all 44 supported read forms: 40 are below 100 ms in this one-sample run; valid land and merge check forms remain 210-224 ms. Log /tmp/atelier-guard-integrated-44-diagnostic.log preserves the failures; this is not final acceptance. Pinned-tree candidate 5b5dca62 is composed on the mission branch at 517f4f85, with 19953 physical non-test Rust lines; quiet paired diagnostics show 40-48 ms guard improvement but still fail S4. Independent review found staged canonical index divergence (G14, 9yms) and an older-Git root-review option gap (G15); repairs and review remain outstanding. Master remains at reviewed functional baseline dd67510f.
