---
created_at: "2026-11-10T18:26:22.996930392+00:00"
id: "atelier-goi9"
evidence_type: "test"
commit: "b799d06244384d615f5669685b0d47eee70210e6"
result: "pass"
captured_at: "2026-11-10T18:26:18.300279604+00:00"
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
title: "next/: fmt --check, clippy -D warnings, nextest (105 passed), -j 4; diff --check clean; nothing under crates/"
updated_at: "2026-11-10T18:26:23.143219048+00:00"
---

## Summary

next/: fmt --check, clippy -D warnings, nextest (105 passed), -j 4; diff --check clean; nothing under crates/

## Command

```console
bash -c 'git diff --check master...HEAD && test -z "$(git diff --stat master...HEAD -- crates/)" && cd next && cargo fmt -- --check && cargo clippy --offline -j 4 --all-targets -- -D warnings && cargo nextest run --offline --build-jobs 4 -j 4'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 12427
SHA-256: c462c4bc36dab54a0aec041f16c367161af0570603daaa33081348f5e70cf730
Test summary: Summary [   3.492s] 105 tests run: 105 passed, 0 skipped
