---
created_at: "2026-11-11T00:33:23.209815177+00:00"
id: "atelier-rcpp"
evidence_type: "test"
commit: "c6c15c4bed27d0b1bbaf77072782b57e7ccd5aff"
result: "pass"
captured_at: "2026-11-11T00:33:10.733157147+00:00"
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
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 3's fixes"
updated_at: "2026-11-11T00:33:23.361148300+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 3's fixes

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
SHA-256: 39ddf6267d236e55abe667eacc5982312805e53aea3401d2f305508820ac96b7
Test summary: Summary [  11.611s] 222 tests run: 222 passed, 0 skipped
