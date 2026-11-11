---
created_at: "2026-11-11T00:12:12.607516552+00:00"
id: "atelier-oi1l"
evidence_type: "test"
commit: "25f7c4febb03d8d2a5a029d604d224fcc1e9eb32"
result: "pass"
captured_at: "2026-11-11T00:12:01.500270677+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 --manifest-path next/Cargo.toml --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nafc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nafc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/ suite after the status wording fix"
updated_at: "2026-11-11T00:12:12.760887250+00:00"
---

## Summary

next/ suite after the status wording fix

## Command

```console
cargo nextest run -j 4 --build-jobs 4 --manifest-path next/Cargo.toml --workspace
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 25208
SHA-256: ecb47984c3fedbb847a86350cf370eddfbb4804928b2850f93899961937290c4
Test summary: Summary [  10.653s] 211 tests run: 211 passed, 0 skipped
