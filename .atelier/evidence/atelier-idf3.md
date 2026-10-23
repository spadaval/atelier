---
created_at: "2026-10-23T15:13:18.168950454+00:00"
id: "atelier-idf3"
evidence_type: "test"
commit: "052b990459d71d425ebcdf3db6d1a65f9c870666"
result: "pass"
captured_at: "2026-10-23T15:12:37.227789056+00:00"
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
title: "full workspace suite with test hooks passes (912 tests) at the second review-fix commit"
updated_at: "2026-10-23T15:13:18.169210910+00:00"
---

## Summary

full workspace suite with test hooks passes (912 tests) at the second review-fix commit

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
SHA-256: eb29a310745990b47b7dfd4847bba47ba4f6763738a5021c60ca03ba9bc63a3f
Test summary: Summary [  39.213s] 912 tests run: 912 passed (1 slow), 0 skipped
