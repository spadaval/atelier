---
schema: atelier.review
schema_version: 2
id: atelier-ey3b
title: finding on atelier-efsl @57231b68
target: atelier-efsl
identity: gpt-6.1-sol
commit: 57231b68eb61df98c9748c0864b5b42599bc5154
created_at: 2026-11-11T00:58:58.785209599Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-efsl.md:37 requires every successful merge to say its configured checks ran and passed. Current merge.rs:138-155 permits a failed check under ATELIER_SKIP and still returns success; tests/merge.rs:1769 covers this. merge.rs:80-81 and publish at :531 also return success on an already-merged retry without running checks. A worker cannot satisfy the line literally without false output or changing accepted behaviour, forbidden by Scope. Limit the passed-check summary to a fresh merge whose checks actually passed; preserve truthful bypass output and the already-merged retry. Require a focused assertion that a bypassed failure is never described as passed, using the existing test.
