---
schema: atelier.review
schema_version: 2
id: atelier-6wpw
title: finding on atelier-va99 @76b870a3
target: atelier-va99
identity: independent-planning-review-astra
commit: 76b870a36854bdd6d98362946ec82eec502305bf
created_at: 2026-11-11T07:04:56.325863425Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-va99.md:41,58 replaces the strategy’s act-is-a-record scenario in a converted scratch clone with a scratch repository created by the new commands. A green test on a freshly initialized repository does not prove the approved converted-repository scenario; oc9k repeats this command, so the gap survives independent validation. Use an isolated copy of a verified converted clone, add a purpose-built draft mission there with the new commands if needed, commit that fixture, then make both comparison clones from that commit. Require exactly one additional commit on each side, matching message, paths and records except the recorded time. Recommendation: FIX NOW; preserve the approved criterion rather than substitute a different fixture environment.
