---
created_at: "2026-11-10T19:35:15.340114041+00:00"
id: "atelier-05he"
evidence_type: "test"
commit: "0ced2a7280f795789586dcad6183bc14bee58dcf"
result: "pass"
captured_at: "2026-11-10T19:35:08.005479243+00:00"
command: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -q -- -D warnings && cargo nextest run -j 4 --no-fail-fast 2>&1 | tail -2'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-67nn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-67nn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging master (e599642f): next/ fmt --check, clippy -D warnings, nextest (123 of 123)"
updated_at: "2026-11-10T19:35:15.485540090+00:00"
---

## Summary

After merging master (e599642f): next/ fmt --check, clippy -D warnings, nextest (123 of 123)

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -q -- -D warnings && cargo nextest run -j 4 --no-fail-fast 2>&1 | tail -2'
```

Exit status: 0

## Stdout

Bytes: 99
SHA-256: ab39a3421f6ad325aa5a3954f38d926fa88f0f7c1f1cfa680f8df3c6a86f6dcc
Test summary: Summary [   6.604s] 123 tests run: 123 passed, 0 skipped

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
