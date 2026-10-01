---
created_at: "2026-10-01T22:47:42.442184079+00:00"
id: "atelier-icr8"
evidence_type: "benchmark"
commit: "dd67510f2ffa02cb4841df90e445985340102185"
result: "pass"
captured_at: "2026-10-01T22:47:42.442073082+00:00"
exit_status: "pass"
path: "/tmp/atelier-rebuild-final2-benchmark.log"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Integrated production reads meet the 100 ms budget"
updated_at: "2026-10-01T22:47:42.442985009+00:00"
---

Orchestrator benchmark on dd67510f: 1000 issues and 200 evidence, one warm-up and ten runs for six ordinary reads; medians31.2–51.0ms, max54.7ms, every maximum under100ms. Earlier120ms failure remains recorded; expanded independent validation pending.
