---
schema: atelier.review
schema_version: 2
id: atelier-luqf
title: finding on atelier-ceko @25eb196c
target: atelier-ceko
identity: gpt-6.1-sol
commit: 25eb196c4625324deed571584ff0c57548e0dcb3
created_at: 2026-11-10T21:25:26.846541035Z
event:
  event: finding
  recommendation: fix_now
---

Shrink: .atelier/issues/atelier-ceko.md:31 (low). Replace the mandatory 30 runs of the merge tests with four test threads alongside another cargo build with focused regression evidence for this defect. Thirty passes under unspecified build contention cannot prove the universal claim that no wait test depends on a poll finishing inside a fixed time; they can still pass with the current race, and they impose unrelated repeated work on this small fix. Keep the failing-before/passing-after reproduction at line 29, make it establish an answered poll followed by a poll deliberately stalled through the deadline, and check the retained status/link, exit 3 and unchanged repository. Keep coverage for a first poll that never answers. This preserves the useful outcome without prescribing a separate load exercise or broad wait-test rewrite. Verdict: shrink; complexity low (two local defects, existing deadline adapter), value high (CI regression and indefinite merge-lock hold), risk low (planning only, reversible). The other child and the one-branch grouping are ready; the stated timeout mechanism matches merge.rs:312-323 and forge.rs:254-260.
