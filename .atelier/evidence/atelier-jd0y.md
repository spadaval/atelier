---
created_at: "2026-10-23T15:03:39.341155824+00:00"
id: "atelier-jd0y"
evidence_type: "test"
commit: "0af43aa5f8868a7650477be5687a63b5277367f1"
result: "pass"
captured_at: "2026-10-23T15:02:58.493038939+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "full workspace suite with test hooks passes (912 tests) on the tree merged with master"
updated_at: "2026-10-23T15:03:39.341415248+00:00"
---

## Summary

full workspace suite with test hooks passes (912 tests) on the tree merged with master

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 114693
SHA-256: 402fe9716ffffbdd4775c67f22efdcdddb9eb7139d939dfe5c0f1424b2315c83
Test summary: Summary [  39.111s] 912 tests run: 912 passed (1 slow), 0 skipped
