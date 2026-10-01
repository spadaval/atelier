---
created_at: "2026-10-01T23:23:28.588232636+00:00"
id: "atelier-1oaq"
evidence_type: "benchmark"
commit: "dcb7b02254006dc52bb242a681e762897fa73ad7"
result: "fail"
captured_at: "2026-10-01T23:23:28.588201139+00:00"
exit_status: "fail"
path: "/tmp/prune-perf-benchmark-default-affinity-final.log"
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
title: "Batched prune improves latency but quiet maxima still exceed S4"
updated_at: "2026-10-01T23:23:28.588606762+00:00"
---

Author quiet benchmark on two original 1000-issue fixtures, default32CPU affinity, ten samples each for full and quiet dry runs: medians83.09–94.16ms; maxima110.17,388.98,125.19,95.12ms. Keep all outliers; no cause is claimed for388.98ms. This supersedes interpretation of earlier113ms CPU0-constrained timings, which forced a single parser thread and remain a separate runtime observation. Candidate safety review passed; further margin is required.
