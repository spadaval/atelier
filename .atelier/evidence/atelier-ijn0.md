---
created_at: "2026-11-10T15:25:02.783317331+00:00"
id: "atelier-ijn0"
evidence_type: "test"
commit: "90ef631949114062def96d142f563b18a680812e"
result: "pass"
captured_at: "2026-11-10T15:25:02.250492814+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mal9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mal9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check and the full nextest suite (37 tests) pass"
updated_at: "2026-11-10T15:25:02.926646721+00:00"
---

## Summary

next/: cargo fmt --check and the full nextest suite (37 tests) pass

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 4222
SHA-256: 229a72b56e8a462b0fe4ca7c17b04f2d0ee8feca52f9a435a79a8363e370915d
Test summary: Summary [   0.056s] 37 tests run: 37 passed, 0 skipped
