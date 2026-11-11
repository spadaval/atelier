---
created_at: "2026-11-11T00:26:18.238002977+00:00"
id: "atelier-a7nf"
evidence_type: "test"
commit: "b81d68eae1b5ea46dd3f751e10900726a9335367"
result: "pass"
captured_at: "2026-11-11T00:26:05.871636054+00:00"
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
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 2's fix"
updated_at: "2026-11-11T00:26:18.391179727+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (222 tests) pass after round 2's fix

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
SHA-256: bc1148ac0f9468b19cac006146e9b4fdf2507d1923d38362aabd6d81eaf8354a
Test summary: Summary [  11.517s] 222 tests run: 222 passed, 0 skipped
