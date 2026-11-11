---
created_at: "2026-11-11T03:21:19.423063767+00:00"
id: "atelier-of2s"
evidence_type: "test"
commit: "e330db97f202997ab3e582eedcef7f004d652225"
result: "pass"
captured_at: "2026-11-11T03:21:05.372415070+00:00"
command: "sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run -j 4'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-1oel"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1oel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check, clippy -D warnings and nextest (239 tests) pass"
updated_at: "2026-11-11T03:21:19.584041616+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and nextest (239 tests) pass

## Command

```console
sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 28830
SHA-256: 9b6d5e99171d9527514b26770d1d1e173865d482d9eee4eaa27298b72bd1700e
Test summary: Summary [  12.947s] 239 tests run: 239 passed, 0 skipped
