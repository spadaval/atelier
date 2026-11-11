---
schema: atelier.review
schema_version: 2
id: atelier-l2fr
title: finding on atelier-zrss @fccfc15e
target: atelier-zrss
identity: reviewer-sol-r3
commit: fccfc15e4f37cfa212b6afa84e841dd3559edf41
created_at: 2026-11-11T00:30:24.740516456Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-app/src/acts.rs:1008: R6. The NotDeferred match arm hides Settled, so an already-settled non-deferred finding can be settled again with ATELIER_SKIP. Reproduced at the pinned head on the base of a scratch repository with a done item and an approving review containing a no-action finding: run drop <item>/<finding> --reason first with a skip, then run it again with --reason second and the same skip. Both exit 0. The second command skips NotDeferred and validation, adds a second Dropped line, and leaves findings() unable to read the review (it has two Taken or Dropped lines). An already-settled finding is explicitly not skippable in the specification; the bypass must not manufacture another settlement or duplicate follow-up work. Check settlement independently of, or before, whether the finding is deferred, and test repeated drop and new --from after a skipped settlement of a non-deferred finding.
