---
created_at: "2026-11-10T18:22:08.635996896+00:00"
id: "atelier-s1u1"
evidence_type: "test"
commit: "eab0a9c0cdcf2b79d02daf2999c0d7d71fe3bcf4"
result: "pass"
captured_at: "2026-11-10T18:22:03.033040981+00:00"
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
title: "next/: fmt --check, clippy -D warnings, nextest (104 passed), -j 4; diff --check clean; nothing under crates/"
updated_at: "2026-11-10T18:22:08.787205582+00:00"
---

## Summary

next/: fmt --check, clippy -D warnings, nextest (104 passed), -j 4; diff --check clean; nothing under crates/

## Command

```console
bash -c 'git diff --check master...HEAD && test -z "$(git diff --stat master...HEAD -- crates/)" && cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 12533
SHA-256: 095b672156a46c81b05e22754e79acaf0a9af3cbe8bdbcd91c5494e901c9f55a
Test summary: Summary [   3.776s] 104 tests run: 104 passed, 0 skipped
