---
created_at: "2026-11-07T02:13:19.141694282+00:00"
id: "atelier-qn61"
evidence_type: "test"
commit: "9c6c1468cde4aad9f13ee0d7c3ed6ad9f6b4fc1b"
result: "pass"
captured_at: "2026-11-07T02:13:14.183119123+00:00"
command: "cargo nextest run -p atelier-cli -j 4 -E 'test(lifecycle_cost)'"
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
title: "cargo nextest run -p atelier-cli -j 4 -E 'test(lifecycle_cost)'"
updated_at: "2026-11-07T02:13:19.142389497+00:00"
---

## Summary

cargo nextest run -p atelier-cli -j 4 -E 'test(lifecycle_cost)'

## Command

```console
cargo nextest run -p atelier-cli -j 4 -E 'test(lifecycle_cost)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 561
SHA-256: 3b25b849b566bdd660e596bb4e4fdd301841dc2676e3d36f28b28f8cb4e599a4
Test summary: Summary [   1.725s] 1 test run: 1 passed, 766 skipped
