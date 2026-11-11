---
created_at: "2026-11-11T00:20:50.789408276+00:00"
id: "atelier-9g7z"
evidence_type: "test"
commit: "052fd709cdb164df07d3431a05c5bd4b64e6d3b7"
result: "pass"
captured_at: "2026-11-11T00:20:38.517764502+00:00"
command: "bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zrss"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zrss"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 1's fixes"
updated_at: "2026-11-11T00:20:50.943888216+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 1's fixes

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 26582
SHA-256: c60246532309b2a1f686fd1e99b1e24ccefe6d081f7e963a4d891464112205d4
Test summary: Summary [  11.410s] 222 tests run: 222 passed, 0 skipped
