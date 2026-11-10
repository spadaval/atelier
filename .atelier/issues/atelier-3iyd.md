---
created_at: "2026-11-10T19:23:14.887937319+00:00"
id: "atelier-3iyd"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-qu4a"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Medium: next/crates/atelier-cli/src/merge.rs:101 R1: CI revalidation under the merge lock uses a fresh Github with no d…"
updated_at: "2026-11-10T19:23:14.887937319+00:00"
---

## Goal

Deferred from finding atelier-qu4a on atelier-xtvh @e4fe00ed, by `atelier review defer`.

Medium: next/crates/atelier-cli/src/merge.rs:101 R1: CI revalidation under the merge lock uses a fresh Github with no deadline. If that gh call stalls, merge holds the lock indefinitely and blocks other items, even though the unlocked polling phase was bounded. Reproduced offline at e4fe00ed with the supplied FAKE_GH and local bare origin: return success on the first runs query, use ATELIER_TEST_HOLD_AFTER_READ to enable the double stall before the locked query, then release it. With ATELIER_TEST_CI_WAIT=1000,50, merge remained running and the merge lock remained exclusively held three seconds after the stalled query began; local and remote main were unchanged. The probe killed its process group and removed its scratch repository. Recommendation: defer, because the prerequisite merge already performed an unbounded CI query under its lock, and the present contract bounds the unlocked polling phase. In follow-up, bound the locked recheck too and return not yet on a timeout; cover a successful initial poll followed by a stalled recheck.

## Done when

Done when was not specified.
