---
created_at: "2026-10-01T23:38:16.300961555+00:00"
id: "atelier-xj9h"
evidence_type: "benchmark"
commit: "d34b6e1f721e181f0faed327f4dd03dab63023e8"
result: "pass"
captured_at: "2026-10-01T23:38:16.300862114+00:00"
exit_status: "pass"
path: "/tmp/prune-perf-owner-final-benchmark-exact.log"
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
title: "Prune full and quiet reads meet S4 on three 1000-issue fixtures"
updated_at: "2026-10-01T23:38:16.301221768+00:00"
---

Bounded prune acceptance only; the complete S4 mission criterion remains open for landing/merge/session reads. Production d34b6e1f exact SHA256 6f35c488d344b123f354c087c4b931449b71479e3e310ba89700b852876bdb0f, quiet default32CPU environment, one warm-up and ten timed runs for full and quiet prune on two original mixed 1000-issue/200-evidence fixtures plus a successful-owner fixture. All six maxima below100ms, overallmax88.79ms; all exact outputs compared with the original dd67510f release on the same fixture. Prior failed timings and the corrected cross-fixture path assertion remain preserved. Independent safety review accepted d34 after29 targetedtests and adversarial shared-branch worktree checks. Main composition source19943 before pending guards.
