---
schema: atelier.review
schema_version: 2
id: atelier-k10m
title: finding on atelier-z5bp @c96db9f5
target: atelier-z5bp
identity: sol-reviewer-r1
commit: c96db9f52afd084701076406cabce5f4ba7bebbf
created_at: 2026-11-10T14:37:06.432030639Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/architecture/quality/scorecard-baseline/defect-method.md:80 R1: The written tracing rule excludes tests, but row 24 counts astromech-aiwv as traced and After merge. At the frozen ae4d18ba, its fix e5138f36 changes only test code; the three lines blamed on 47cf61a0 are test lines. Applying step 2 literally cannot produce this row, so the claimed 27 traced and 11 After merge do not follow the method. Make the treatment of test-only bugs explicit and consistent with the outcome measure, then rerun the sample and update the table, totals and Report. Keeping the current exclusion gives 26 traced, 10 After merge and 4 Not traceable. The seeded draw reproduced 52 candidates and all 30 table rows in order; all three requested merge checks passed.
