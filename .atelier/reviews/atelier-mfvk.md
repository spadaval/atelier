---
schema: atelier.review
schema_version: 2
id: atelier-mfvk
title: finding on atelier-u1ko @5119f89e
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: 5119f89ea61ebfb2dc3bfbdc8b8348d39c023db1
created_at: 2026-11-11T05:32:50.684722814Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-u1ko.md:122 (plan at 5119f89e), medium: The freeze does not define a reliable status signal or when a temporary thaw ends. It says to read u1ko status until cutover has run, but merging makes u1ko done while wd1i is still todo and is never started (atelier-wd1i:45-54). The Desk worker following atelier-l5ai:53, which checks only whether either is in progress, can therefore start in that gap. During the permitted thaw, nothing explicitly restores the freeze before the final sync and proof runs; another allowed Desk merge can change the source tree after the last rehearsal. Step G:46 asks for sync and CI on later base code, but does not repeat the root checks and rehearsal that this amendment requires after a thaw. This undermines the claimed stable source for the switch and its rehearsed cutover. State the observable rule at each Desk start and merge: once u1ko has started, including after it is done, the freeze remains until wd1i has successfully completed, except for an explicitly bounded thaw. End that thaw before the final sync, root checks, CI and rehearsal, keep the freeze through cutover, and identify the source commit those proofs cover. Reconcile the Desk workers instructions with that rule. This needs a planning clarification, not a new locking system.
