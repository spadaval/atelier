---
created_at: "2026-11-10T21:36:36.152659415+00:00"
id: "atelier-6kac"
evidence_type: "test"
commit: "19cec0493dc02762fcefbea8fe53ffc3ff0c4833"
result: "pass"
captured_at: "2026-11-10T21:36:26.418421306+00:00"
command: "sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --locked --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4 --locked'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-rsfm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rsfm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: fmt check, clippy -D warnings and all 154 tests pass after the fix"
updated_at: "2026-11-10T21:36:36.298647956+00:00"
---

## Summary

next/: fmt check, clippy -D warnings and all 154 tests pass after the fix

## Command

```console
sh -c 'cd next && cargo fmt -- --check && cargo clippy -j 4 --locked --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4 --locked'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 18004
SHA-256: 70fa44139b2a3e3d195f04197341f6be5d979ddf95769ec11c593c2794974b87
Test summary: Summary [   8.914s] 154 tests run: 154 passed, 0 skipped
