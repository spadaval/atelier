---
created_at: "2026-11-10T18:09:52.627426764+00:00"
id: "atelier-orch"
evidence_type: "test"
commit: "a7a712dbce554a0265504888fc15d408190a044f"
result: "pass"
captured_at: "2026-11-10T18:09:47.933407940+00:00"
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
updated_at: "2026-11-10T18:09:52.771030956+00:00"
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

Bytes: 12317
SHA-256: 8273b1ed159be77021b720735126a5797b0cba0e548568bad2d2e4a92e843dc0
Test summary: Summary [   3.519s] 104 tests run: 104 passed, 0 skipped
