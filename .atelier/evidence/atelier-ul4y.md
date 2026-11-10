---
created_at: "2026-11-10T15:14:10.351441123+00:00"
id: "atelier-ul4y"
evidence_type: "test"
commit: "134def9355d27a5760e415f0ce8dcfcb23109f9a"
result: "pass"
captured_at: "2026-11-10T15:14:09.279321029+00:00"
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
title: "next/: cargo fmt --check and the full nextest suite (35 tests) pass"
updated_at: "2026-11-10T15:14:10.495833605+00:00"
---

## Summary

next/: cargo fmt --check and the full nextest suite (35 tests) pass

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo nextest run --build-jobs 4 --test-threads 4 --locked'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 4242
SHA-256: 0c1ff04b1934a4c7e9c7219fced19774ff944133552c59467636931235e3a58b
Test summary: Summary [   0.052s] 35 tests run: 35 passed, 0 skipped
