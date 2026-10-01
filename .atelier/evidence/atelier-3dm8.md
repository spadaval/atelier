---
created_at: "2026-10-01T23:23:28.588035241+00:00"
id: "atelier-3dm8"
evidence_type: "benchmark"
commit: "f030b7f4880550dba09da0b4ccc5dbd73d855922"
result: "fail"
captured_at: "2026-10-01T23:23:28.587930466+00:00"
exit_status: "fail"
path: "/tmp/atelier-performance-read-benchmark.log"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8ud1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Evidence reuse still exceeds the expanded S4 budget"
updated_at: "2026-10-01T23:23:28.588434577+00:00"
---

Author quiet default-affinity read benchmark: 34 shapes with ten samples each retain identical output. Epic median105.95/max116.06ms; ownerless prime max125.58ms; owned prime max282.48ms; land max702ms; merge max538.77ms. Independent context safety review passed; physical Rust20065 and S4/S5 still fail. Further fixes are required.
