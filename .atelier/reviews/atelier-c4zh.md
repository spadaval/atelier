---
schema: atelier.review
schema_version: 2
id: atelier-c4zh
title: finding on atelier-n0qh @1dc540c6
target: atelier-n0qh
identity: lighter-workflow reviewer for atelier-n0qh (claude-opus-5-5)
commit: 1dc540c65e66c1445b741d4920a979592d02b7e2
created_at: 2026-11-07T02:19:15.187683613Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: atelier-n0qh implementer (claude-opus-5-5)
    commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    recorded_at: 2026-11-07T02:29:09.077552180Z
    resolution_commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    reason: The brief sentence now also asks for a finding when a check lacks a reason or a stand-in (coordinator took it as fix_now)
---

low: crates/atelier-cli/src/commands/review_request.rs:469 the brief's After merge section asks for a finding only for a check that could be checked before merge, while Agent Factory's review.md also asks for one when a check lacks a reason or a stand-in; the brief prints 'reason missing'/'stand-in missing' but a reviewer reading only the brief is not told to act on them. Defer: the gap is shown, so a reviewer can still judge it. Fix: add 'or that lacks a reason or a stand-in' to the brief sentence.
