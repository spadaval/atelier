---
created_at: "2026-11-10T17:45:20.148216989+00:00"
id: "atelier-7urw"
evidence_type: "test"
commit: "edc10b23701bdc678dfed0f407ecb0e5e258c4e6"
result: "pass"
captured_at: "2026-11-10T17:45:14.925314874+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-syyj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-syyj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check, clippy --all-targets, and cargo nextest run (100 passed), -j 4"
updated_at: "2026-11-10T17:45:20.300989068+00:00"
---

## Summary

next/: cargo fmt --check, clippy --all-targets, and cargo nextest run (100 passed), -j 4

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 12099
SHA-256: 5fd3104bb9dd8cac8539d48e173705e6e7b9bc7b874a07681df53f69bc487218
Test summary: Summary [   3.201s] 100 tests run: 100 passed, 0 skipped
