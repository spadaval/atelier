---
created_at: "2026-10-01T22:47:42.442664281+00:00"
id: "atelier-z457"
evidence_type: "test"
commit: "dd67510f2ffa02cb4841df90e445985340102185"
result: "pass"
captured_at: "2026-10-01T22:47:42.442615887+00:00"
exit_status: "pass"
path: "/tmp/atelier-rebuild-dogfood-bootstrap.log"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-wp90"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Actual Codex prime and land dogfood passed"
updated_at: "2026-10-01T22:47:42.443611651+00:00"
---

Orchestrator observation on dd67510f after reviewed master bootstrap: prime restores atelier-1ioo; land passes, rejects a deliberately uncommitted canonical edit, then passes after exact byte restoration. HEAD and record bytes unchanged; checkout clean. Earlier blocked attempt remains recorded.
