---
schema: atelier.review
schema_version: 2
id: atelier-qu4a
title: finding on atelier-xtvh @e4fe00ed
target: atelier-xtvh
identity: gpt-6.1-sol
commit: e4fe00ed27b10c60221428eb73c253bc39b9e1e4
created_at: 2026-11-10T19:22:35.963650518Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-opus-5-5
    commit: 965a064b21c08b45b8ac7823fb5a162f717a47e2
    recorded_at: 2026-11-10T19:23:14.887937319Z
    follow_up: atelier-3iyd
    reason: Deferred to backlog follow-up atelier-3iyd under atelier-rsfm.
---

Medium: next/crates/atelier-cli/src/merge.rs:101 R1: CI revalidation under the merge lock uses a fresh Github with no deadline. If that gh call stalls, merge holds the lock indefinitely and blocks other items, even though the unlocked polling phase was bounded. Reproduced offline at e4fe00ed with the supplied FAKE_GH and local bare origin: return success on the first runs query, use ATELIER_TEST_HOLD_AFTER_READ to enable the double stall before the locked query, then release it. With ATELIER_TEST_CI_WAIT=1000,50, merge remained running and the merge lock remained exclusively held three seconds after the stalled query began; local and remote main were unchanged. The probe killed its process group and removed its scratch repository. Recommendation: defer, because the prerequisite merge already performed an unbounded CI query under its lock, and the present contract bounds the unlocked polling phase. In follow-up, bound the locked recheck too and return not yet on a timeout; cover a successful initial poll followed by a stalled recheck.
