---
schema: atelier.review
schema_version: 2
id: atelier-rvrk
title: finding on atelier-u1ko @8b1b75e0
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:04:29.985864049Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:73-79 The prescribed inverse-journal rollback does not reconcile real clone bases with published reverts. In step 5 a fresh clone pushes product conversion C from base A; stop before step 6 (also reachable when the next product conversion fails). The real clone is still at A and has no journalled ref move to undo. Reverting C in the conversion clone and pushing yields remote R, while the real clone stays at A. After step 6, literally undoing its recorded ref move likewise returns the real base to A, not R. Thus line 79's required revert history and local/remote agreement cannot follow from line 78's reverse-only algorithm. Settle rollback's exception for published conversions: produce the rollback tip, reconcile each affected real base checkout/ref to that tip, and restore original feature branches/detached checkouts without resetting the published base back to A. Keep adopt rollback separate, since it must restore local refs without pushing. Assert this for a stop after a product push before any real-clone fetch and for the mixed step-6 stop.
