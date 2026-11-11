---
created_at: "2026-11-11T00:11:12.529512454+00:00"
id: "atelier-jjwy"
evidence_type: "test"
commit: "363378a7a27f65b3aec3c1be416557e71f5f7944"
result: "pass"
captured_at: "2026-11-11T00:10:58.385409232+00:00"
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
title: "next/: cargo fmt --check, clippy -D warnings and the full nextest suite (220 tests) pass"
updated_at: "2026-11-11T00:11:12.682431070+00:00"
---

## Summary

next/: cargo fmt --check, clippy -D warnings and the full nextest suite (220 tests) pass

## Command

```console
bash -c 'cd next && cargo fmt --check && cargo clippy -j 4 --all-targets -- -D warnings && cargo nextest run --build-jobs 4 --test-threads 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 26569
SHA-256: 263e5bcbb883cd26730e3d4e05fca95ed8417ef438f5c6510aa1cce5a27c7f04
Test summary: Summary [  11.235s] 220 tests run: 220 passed, 0 skipped
