---
created_at: "2026-11-10T18:03:33.587788939+00:00"
id: "atelier-gwh2"
evidence_type: "test"
commit: "c9e4c6d074a02246828a2317a71fac7bb80f9ba2"
result: "pass"
captured_at: "2026-11-10T18:03:29.522434169+00:00"
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
title: "next/: fmt --check, clippy -D warnings, nextest (102 passed), -j 4; diff --check clean; nothing under crates/"
updated_at: "2026-11-10T18:03:33.729490897+00:00"
---

## Summary

next/: fmt --check, clippy -D warnings, nextest (102 passed), -j 4; diff --check clean; nothing under crates/

## Command

```console
bash -c 'git diff --check master...HEAD && test -z "$(git diff --stat master...HEAD -- crates/)" && cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 11870
SHA-256: db4f96f86a6e1676e5e3d30e455d339c4ba7f36ddc5b966b265eeb6edad6688b
Test summary: Summary [   3.301s] 102 tests run: 102 passed, 0 skipped
