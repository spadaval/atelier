---
created_at: "2026-11-10T17:55:54.280825926+00:00"
id: "atelier-z0gp"
evidence_type: "test"
commit: "889def0acdd142472a28c2a86c495db21f8c3bf7"
result: "pass"
captured_at: "2026-11-10T17:55:47.377654419+00:00"
command: "bash -c 'git diff --check master...HEAD && test -z \"$(git diff --stat master...HEAD -- crates/)\" && cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'"
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
title: "next/: fmt --check, clippy --all-targets -D warnings, nextest (101 passed), -j 4; diff --check clean; nothing under crates/"
updated_at: "2026-11-10T17:55:54.506787057+00:00"
---

## Summary

next/: fmt --check, clippy --all-targets -D warnings, nextest (101 passed), -j 4; diff --check clean; nothing under crates/

## Command

```console
bash -c 'git diff --check master...HEAD && test -z "$(git diff --stat master...HEAD -- crates/)" && cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 12316
SHA-256: 860c84d4275bb8ff20de365db6a68dd35a20ae2c5d3218ec946650d4aa6eb5b6
Test summary: Summary [   3.724s] 101 tests run: 101 passed, 0 skipped
