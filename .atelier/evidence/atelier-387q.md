---
created_at: "2026-11-11T00:40:19.696785045+00:00"
id: "atelier-387q"
evidence_type: "test"
commit: "0edc6d1163d91357bda19d29b7cfa0b95c448a52"
result: "pass"
captured_at: "2026-11-11T00:40:07.103299813+00:00"
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
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (223 tests) pass after round 4's fix"
updated_at: "2026-11-11T00:40:19.849287529+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (223 tests) pass after round 4's fix

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 26719
SHA-256: a134b31d835c630ddf3048fc3346d2248a2a33c6149fedec6c067881620129fe
Test summary: Summary [  11.735s] 223 tests run: 223 passed, 0 skipped
