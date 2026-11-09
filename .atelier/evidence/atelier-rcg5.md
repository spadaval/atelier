---
created_at: "2026-11-09T17:02:27.479543409+00:00"
id: "atelier-rcg5"
evidence_type: "test"
commit: "594dc0f8fc5fd3a5955733f4aa4834cb3a47ce98"
result: "pass"
captured_at: "2026-11-09T17:01:17.162154135+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the round 1 fixes: full workspace suite passes, including the new naming, recovery and build-slot tests"
updated_at: "2026-11-09T17:02:27.613318459+00:00"
---

## Summary

After the round 1 fixes: full workspace suite passes, including the new naming, recovery and build-slot tests

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 141212
SHA-256: 72fb222d4747ad4a9ff8204ebd2a79d3346b98feff075cc4182bcf5eae311a69
Test summary: Summary [  67.334s] 1115 tests run: 1115 passed, 0 skipped
