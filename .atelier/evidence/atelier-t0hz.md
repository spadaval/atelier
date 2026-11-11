---
created_at: "2026-11-11T03:27:57.030070250+00:00"
id: "atelier-t0hz"
evidence_type: "test"
commit: "5e6fe8f7791c6141c6d04c81fd3d226b9da7eb04"
result: "pass"
captured_at: "2026-11-11T03:27:43.392864815+00:00"
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
title: "next/: fmt --check, clippy -D warnings and nextest (239 tests) pass after the atelier-yul7 fix"
updated_at: "2026-11-11T03:27:57.188073407+00:00"
---

## Summary

next/: fmt --check, clippy -D warnings and nextest (239 tests) pass after the atelier-yul7 fix

## Command

```console
sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 28722
SHA-256: 14186cccb1039fdacd3f3f6c5fe2c9cbd78e07d801630311f4b7270024d0152a
Test summary: Summary [  12.917s] 239 tests run: 239 passed, 0 skipped
