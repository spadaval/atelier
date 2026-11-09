---
created_at: "2026-11-09T17:03:40.138054898+00:00"
id: "atelier-m7wg"
evidence_type: "test"
commit: "594dc0f8fc5fd3a5955733f4aa4834cb3a47ce98"
result: "pass"
captured_at: "2026-11-09T17:02:28.179258184+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 --workspace --features test-hooks"
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
title: "After the round 1 fixes: full workspace suite with test-hooks passes"
updated_at: "2026-11-09T17:03:40.267745026+00:00"
---

## Summary

After the round 1 fixes: full workspace suite with test-hooks passes

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 142478
SHA-256: 1c34a01cab29c0ea2463a5b22f680fddf2a998c9c6b8cbee2e690e8ae6ede3d6
Test summary: Summary [  68.611s] 1125 tests run: 1125 passed, 0 skipped
