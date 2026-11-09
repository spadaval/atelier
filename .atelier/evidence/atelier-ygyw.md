---
created_at: "2026-11-09T16:43:41.262105571+00:00"
id: "atelier-ygyw"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:42:30.607356821+00:00"
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
title: "Full workspace suite with test-hooks passes"
updated_at: "2026-11-09T16:43:41.390826744+00:00"
---

## Summary

Full workspace suite with test-hooks passes

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 142001
SHA-256: 625c25620e980396f57f9e476e1831a050f1e22a977257cc7396e27f38b0be80
Test summary: Summary [  66.978s] 1121 tests run: 1121 passed, 0 skipped
