---
schema: atelier.review
schema_version: 2
id: atelier-yz6k
title: finding on atelier-z5bp @3101ad76
target: atelier-z5bp
identity: sol-reviewer-r2
commit: 3101ad76ef6b3231c4242a7473e6b5f5f7716096
created_at: 2026-11-10T14:45:26.374800226Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/architecture/quality/scorecard-baseline/defect-method.md:75 R3: Step 1's fallback stops before the fix in row 26. At the frozen ae4d18ba, no eligible commit subject names astromech-g1ys. Walking back from its first done record 2e5e6df9 encounters c3094be9, whose subject names the other item astromech-5c5t, before reaching the listed fix bf36a8f2. c3094be9 changes only tracker records, but the written stopping rule does not exclude those. Following it literally therefore yields no fix for this sampled bug, contradicting the claim that step 1 recovers all 52 and preventing a fresh tracer from reproducing the table. Define which commits can end the walk, explicitly handle tracker-only follow-up records, and rerun commit discovery for all candidates. The seeded draw reproduced 52 candidates and the 30 table rows in order; all three requested merge checks passed. The prior test-defect and cross-file tracing findings have explicit fixes, but this independent discovery failure still blocks approval.
