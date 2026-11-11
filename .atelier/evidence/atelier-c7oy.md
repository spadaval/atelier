---
created_at: "2026-11-11T07:15:19.428004223+00:00"
id: "atelier-c7oy"
evidence_type: "test"
commit: "2591288505e645142ea1be5952048170627b2705"
result: "pass"
captured_at: "2026-11-11T07:15:04.241473090+00:00"
command: "bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings 2>&1 | tail -1 && cargo nextest run -j 4 2>&1 | tail -1'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3hl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3hl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/ workspace suite, fmt and clippy on the branch synced with master"
updated_at: "2026-11-11T07:15:19.603488522+00:00"
---

## Summary

next/ workspace suite, fmt and clippy on the branch synced with master

## Command

```console
bash -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings 2>&1 | tail -1 && cargo nextest run -j 4 2>&1 | tail -1'
```

Exit status: 0

## Stdout

Bytes: 134
SHA-256: a93cdb5b2cf94f94330f7b4f1a87ecfd2613fa0b18cfa614d3dbe8debd2be604
Test summary: Summary [  14.248s] 283 tests run: 283 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
