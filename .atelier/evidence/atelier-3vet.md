---
created_at: "2026-11-10T15:36:09.089490653+00:00"
id: "atelier-3vet"
evidence_type: "test"
commit: "1a14e2ff239af8db7aa0bd143e0ddb479084d583"
result: "pass"
captured_at: "2026-11-10T15:36:08.608808192+00:00"
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
title: "next/: cargo fmt --check and the full nextest suite (38 tests) pass"
updated_at: "2026-11-10T15:36:09.229740004+00:00"
---

## Summary

next/: cargo fmt --check and the full nextest suite (38 tests) pass

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 4328
SHA-256: 560f0e91902a6320a79135dc0ec63a3d59dae4a41ff7e1efccc79a9dd976222a
Test summary: Summary [   0.058s] 38 tests run: 38 passed, 0 skipped
