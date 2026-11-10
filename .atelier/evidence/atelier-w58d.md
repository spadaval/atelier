---
created_at: "2026-11-10T15:29:53.467850749+00:00"
id: "atelier-w58d"
evidence_type: "test"
commit: "9af9d84d948cf5f80e538b847b2d5272c90de14f"
result: "pass"
captured_at: "2026-11-10T15:29:52.853998732+00:00"
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
updated_at: "2026-11-10T15:29:53.614245693+00:00"
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
SHA-256: c92ed51c242c33d56812f887ff1aa36f1ad103644a8e34f7809624f6423aa87a
Test summary: Summary [   0.079s] 37 tests run: 37 passed, 0 skipped
