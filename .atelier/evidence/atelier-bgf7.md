---
created_at: "2026-11-09T17:15:13.785424730+00:00"
id: "atelier-bgf7"
evidence_type: "test"
commit: "eef3a56fbe14ece1694390d496e5b11b2f0ded5b"
result: "pass"
captured_at: "2026-11-09T17:13:59.891407618+00:00"
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
title: "After the round 2 fixes: full workspace suite with test-hooks passes"
updated_at: "2026-11-09T17:15:13.917300179+00:00"
---

## Summary

After the round 2 fixes: full workspace suite with test-hooks passes

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 142756
SHA-256: 4bae2428b10261b77bcf11b4baa6d1b174f0b60c2eca8f27792dc09bc459de24
Test summary: Summary [  68.351s] 1127 tests run: 1127 passed, 0 skipped
