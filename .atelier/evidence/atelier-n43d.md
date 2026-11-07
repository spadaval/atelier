---
created_at: "2026-11-07T04:10:44.076224917+00:00"
id: "atelier-n43d"
evidence_type: "test"
commit: "658d43ee98e582e39f89de045ead46bc5e3f4aaf"
result: "pass"
captured_at: "2026-11-07T04:10:41.591887353+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-n4mj"
  role: "validates"
criteria:
- slug: "lifecycle-cost"
  result: "pass"
criteria_mission: "atelier-n4mj"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n4mj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "lifecycle-cost at HEAD: scenario asserts 1 review file per finding"
updated_at: "2026-11-07T04:10:44.076743765+00:00"
---

## Summary

lifecycle-cost at HEAD: scenario asserts 1 review file per finding

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 538
SHA-256: d0c5db98b0ec15ae26a86a5b49f54c01bc575cf78131c8c07f9799fabe29db45
Test summary: Summary [   0.809s] 1 test run: 1 passed, 0 skipped
